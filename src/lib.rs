//! Derive macros for some common patterns
//!
//! The currently implemented patterns are:
//!  - Builder
//!  - From
//!
//! # Builder
//! for the `Builder` macro it generates an impl with methods of the form:
//! ```text
//! fn field(mut self, value: impl Into<Type>) -> Self {
//!     self.field = value.into()
//!     self
//! }
//! ```
//!
//! An example of the generated code for a struct is:
//! ```text
//! #[derive(Default, Builder)]
//! struct Example {
//!     item: String,
//!     value: usize,
//! }
//!
//! // generated impl
//! impl Example {
//!     fn item(mut self, value: impl Into<String>) -> Self {
//!         self.item = value.into();
//!         self
//!     }
//!
//!     fn value(mut self, value: impl Into<usize>) -> Self {
//!         self.value = value.into();
//!         self
//!     }
//! }
//!
//! // using the values
//! fn func() {
//!     let ex = Example::default()
//!         .item("something")
//!         .value(1);
//!     ...
//! }
//! ```
//!
//! if you want to not include a field in the builder pattern use the `skip` attribute:
//! ```text
//! #[derive(Builder)]
//! struct Example {
//!     #[builder(skip)]
//!     item: String,
//!     value: usize,
//! }
//! ```
//!
//! if you do not want to have the `Into` use the `no_into` attribute:
//! ```text
//! #[derive(Builder)]
//! struct Example {
//!     #[builder(no_into)]
//!     item: String,
//!     value: usize,
//! }
//! ```
//!
//! The Builder pattern is not defined for enums, unit-like struct, newtypes, and tuple structs
//!
//! # From
//! For the `From` derive it implements the trivial `From<Type>` implementations:
//! ```text
//! #[derive(From)]
//! enum Example {
//!     Empty,
//!     Number(f32),
//!     Pair(String, String),
//! }
//!
//! // will generate
//! impl From<()> for Example {
//!     fn from(value: ()) -> Self {
//!         Example::Empty
//!     }
//! }
//! impl From<f32> for Example {
//!     fn from(value: f32) -> Self {
//!         Example::Number(f32)
//!     }
//! }
//! impl From<(String, String)> for Example {
//!     fn from(value: (String, String)) -> Self {
//!         Example::Pair(value.0, value.1)
//!     }
//! }
//! ```
//!
//! For struct datatypes it uses tuples as the type to convert from:
//! ```text
//! #[derive(From)]
//! struct Example {
//!     item: usize
//!     value: String
//! }
//!
//! // generates
//! impl From<(usize, String)> for Example {
//!     fn from(value: (usize, String)) -> Self {
//!         Example {
//!             item: value.0,
//!             value: value.1,
//!         }
//!     }
//! }
//! ```
//!
//! If you need to not generate a `From` implementation use the `skip` attribute
//! ```text
//! #[derive(From)]
//! enum Example {
//!     #[from(skip)]
//!     Empty,
//!     Number(f32),
//!     Pair(String, String),
//! }
//! ```
mod builder_derive;
mod core;
mod from_derive;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;

use builder_derive::impl_builder;
use from_derive::impl_from;

#[proc_macro_derive(From, attributes(from))]
pub fn from_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_from(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_builder(&ast)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
