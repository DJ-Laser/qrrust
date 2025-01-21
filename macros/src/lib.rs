use proc_macro2::TokenStream;

#[proc_macro]
pub fn test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = TokenStream::from(input);

  let output = input;

  return proc_macro::TokenStream::from(output);
}
