use std::alloc::Layout;

use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, ExprArray, ExprLit, Lit};

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

impl TryFrom<char> for LevelInput {
  type Error = String;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      '#' => Ok(Self::Wall),
      ' ' => Ok(Self::Space),
      '*' => Ok(Self::Goal),
      'b' => Ok(Self::Box),
      'B' => Ok(Self::BoxOnGoal),
      'p' => Ok(Self::Player),
      'P' => Ok(Self::PlayerOnGoal),
      c => Err(format!("Expected any of `# *bBpP`, got `{}`", c)),
    }
  }
}

struct Input {
  layout: Vec<Vec<LevelInput>>,
}

impl Parse for Input {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let exprs: ExprArray = input.parse()?;
    let exprs = exprs.elems;

    let layout: Vec<Vec<LevelInput>> = exprs
      .into_iter()
      .map(|element| {
        let Expr::Lit(ExprLit {
          lit: Lit::Str(string),
          ..
        }) = element
        else {
          return Err(syn::Error::new(element.span(), "Expected a string here :("));
        };

        string
          .value()
          .chars()
          .map(|c| LevelInput::try_from(c).map_err(op))
      })
      .collect();

    Ok(Self { layout })
  }
}

#[proc_macro]
pub fn level(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = TokenStream::from(input);

  let output = input;

  return proc_macro::TokenStream::from(output);
}
