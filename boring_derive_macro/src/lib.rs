mod builder_derive;
mod from_derive;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;

use builder_derive::impl_builder;
use from_derive::impl_from;

#[proc_macro_derive(From)]
pub fn from_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_from(&ast)
}

#[proc_macro_derive(Builder)]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_builder(&ast)
}
