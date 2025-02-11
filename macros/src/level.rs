use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, ExprArray, ExprLit, Ident, Lit, Token};

#[derive(Debug)]
enum LevelInput {
  Wall,
  Space,
  Goal,
  Box,
  BoxOnGoal,
  Player,
  PlayerOnGoal,
}

impl LevelInput {
  pub fn is_box(&self) -> bool {
    match self {
      Self::Box | Self::BoxOnGoal => true,
      _ => false,
    }
  }

  pub fn is_player(&self) -> bool {
    match self {
      Self::Player | Self::PlayerOnGoal => true,
      _ => false,
    }
  }

  pub fn is_goal(&self) -> bool {
    match self {
      Self::Goal | Self::BoxOnGoal | Self::PlayerOnGoal => true,
      _ => false,
    }
  }
}

impl<'a> TryFrom<&'a char> for LevelInput {
  type Error = String;

  fn try_from(value: &'a char) -> Result<Self, Self::Error> {
    match value {
      '#' => Ok(Self::Wall),
      ' ' => Ok(Self::Space),
      'g' => Ok(Self::Goal),
      'b' => Ok(Self::Box),
      'B' => Ok(Self::BoxOnGoal),
      'p' => Ok(Self::Player),
      'P' => Ok(Self::PlayerOnGoal),
      c => Err(format!("Expected any of `# bBpPg`, got `{}`", c)),
    }
  }
}

#[derive(Debug)]
pub struct Input {
  layout: Vec<(Vec<LevelInput>, Span)>,
  name: Ident,
  width: usize,
  height: usize,
  span: Span,
}

impl Parse for Input {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let name: Ident = input.parse()?;

    let _: Token![=] = input.parse()?;

    let exprs: ExprArray = input.parse()?;
    let span = exprs.span();

    let exprs = exprs.elems;

    if exprs.is_empty() {
      return Err(syn::Error::new(span, "Levels should not be empty"));
    }

    let mut expected_width = None;
    let height = exprs.len();

    let layout: syn::Result<Vec<(Vec<LevelInput>, Span)>> = exprs
      .into_iter()
      .map(|element| {
        let span = element.span();

        let Expr::Lit(ExprLit {
          lit: Lit::Str(string),
          ..
        }) = element
        else {
          return Err(syn::Error::new(span, "Expected a string here :("));
        };

        let string = string.value();
        let len = string.chars().count();

        if let Some(width) = expected_width {
          if len != width {
            return Err(syn::Error::new(
              span,
              format!(
                "Widths should all be the same, expected: {}, but got: {}",
                width, len,
              ),
            ));
          }
        } else {
          expected_width = Some(len);
        }

        string
          .chars()
          .map(|c| LevelInput::try_from(&c).map_err(|message| syn::Error::new(span, message)))
          .collect::<syn::Result<_>>()
          .map(|ok| (ok, span))
      })
      .collect();

    Ok(Self {
      layout: layout?,
      name,
      width: expected_width.unwrap(),
      height,
      span,
    })
  }
}

pub fn macro_impl(input: Input) -> TokenStream {
  let mut layout: Vec<Vec<TokenStream>> = Vec::new();
  let mut player: Option<TokenStream> = None;
  let mut boxes: Vec<TokenStream> = Vec::new();
  let mut goal_count = 0;

  for (y, (input_row, span)) in input.layout.into_iter().enumerate() {
    let mut row: Vec<TokenStream> = Vec::new();

    for (x, item) in input_row.into_iter().enumerate() {
      let layout_variant = "crate::level::LevelObject::".to_string()
        + match item {
          _ if item.is_goal() => {
            goal_count += 1;
            "Goal"
          }
          LevelInput::Wall => "Wall",
          _ => "Space",
        };

      row.push(layout_variant.parse().unwrap());

      let (x, y) = (x, y);
      let position = quote! {(#x, #y)};
      if item.is_box() {
        boxes.push(position);
        continue;
      } else if !item.is_player() {
        continue;
      }

      if let Some(_) = player {
        return syn::Error::new(span, format!("A level should only have one player"))
          .to_compile_error();
      } else {
        player = Some(position)
      }
    }

    layout.push(row);
  }

  let Some(player) = player else {
    return syn::Error::new(input.span, format!("A level should have a player")).to_compile_error();
  };

  if goal_count == 0 {
    return syn::Error::new(input.span, format!("There must be at least one goal"))
      .to_compile_error();
  }

  let box_count = boxes.len();
  if box_count < goal_count {
    return syn::Error::new(
      input.span,
      format!("There must be at least as many boxes as goals. Found {box_count} boxes and {goal_count} goals"),
    )
    .to_compile_error();
  }

  let width = input.width;
  let height = input.height;
  let name = input.name;
  let view_name = Ident::new(&(name.to_string() + "_VIEW"), name.span());
  let static_name = Ident::new(&(name.to_string() + "_STATIC"), name.span());
  let exec_name = Ident::new(&(name.to_string() + "_EXEC"), name.span());
  let print_name = Ident::new(&(name.to_string() + "_PRINT"), name.span());

  return quote! {
    #[allow(non_upper_case_globals)]
    static mut #static_name: ::core::mem::MaybeUninit<crate::level::Level<#width, #height, #box_count>> = ::core::mem::MaybeUninit::uninit();
    #[allow(non_upper_case_globals)]
    static mut #view_name: ::core::mem::MaybeUninit<crate::graphics::LevelView<#width, #height>> = ::core::mem::MaybeUninit::uninit();

    pub fn #name() -> (fn() -> (), fn(&crate::level::Movement) -> bool) {
      unsafe {
        let level = crate::level::Level::__new_from_raw([#([#(#layout),*]),*], [#(#boxes),*], #player);
        #view_name = ::core::mem::MaybeUninit::new(crate::graphics::LevelView::from(&level));
        #static_name = ::core::mem::MaybeUninit::new(level);
      }

      (#print_name, #exec_name)
    }

    #[allow(static_mut_refs)]
    #[allow(non_snake_case)]
    fn #print_name() {
      unsafe {
        #view_name.assume_init_ref().print();
      }
    }

    #[allow(static_mut_refs)]
    #[allow(non_snake_case)]
    fn #exec_name(movement: &crate::level::Movement) -> bool {
      unsafe {
        let level = #static_name.assume_init_mut();
        level.move_player(movement);

        #view_name.assume_init_mut().update(&level);
        level.is_solved()
      }
    }
  };
}
