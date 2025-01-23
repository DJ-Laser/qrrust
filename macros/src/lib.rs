mod level;

macro_rules! fn_macro {
  ($name:ident) => {
    #[proc_macro]
    pub fn $name(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
      $crate::$name::macro_impl(::syn::parse_macro_input!(input)).into()
    }
  };
}

fn_macro! { level }
