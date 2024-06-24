mod from_derive;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;

use from_derive::impl_from;

#[proc_macro_derive(From)]
pub fn from_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_from(&ast)
}
