// use proc_macro::TokenStream;
// use quote::{quote, quote_spanned};
// use syn::{spanned::Spanned, Data, Field, Fields};
//
// /// Derive macros implementations for traits that often have very simple implementations
// ///
// /// For example a `From` trait can be implemented very trivially for most data types
// ///
// /// Can be useful for enums that just wrap other types
// /// enum Example {
// ///     First(A),
// ///     Second(B),
// /// }
// ///
// /// impl From<A> for Example {
// ///     fn from(value: A) -> Self {
// ///         Self::First(value)
// ///     }
// /// }
// ///
// /// impl From<B> for Example {
// ///     fn from(value: B) -> Self {
// ///         Self::Second(value)
// ///     }
// /// }
// ///
//
// enum Example {
//     Empty,
//     First(String),
//     Second(usize),
//     Multiple(String, usize),
//     Struct { a: String, b: String },
// }
//
// impl From<()> for Example {
//     fn from(_: ()) -> Self {
//         Self::Empty
//     }
// }
//
// impl From<String> for Example {
//     fn from(value: String) -> Self {
//         Self::First(value)
//     }
// }
//
// impl From<usize> for Example {
//     fn from(value: usize) -> Self {
//         Self::Second(value)
//     }
// }
//
// impl From<(String, usize)> for Example {
//     fn from(value: (String, usize)) -> Self {
//         Self::Multiple(value.0, value.1)
//     }
// }
//
// impl From<(String, String)> for Example {
//     fn from(value: (String, String)) -> Self {
//         Self::Struct {
//             a: value.0,
//             b: value.1,
//         }
//     }
// }
//
// // would need to know the error type ahead of time
// impl TryFrom<Example> for () {
//     type Error = ();
//     fn try_from(value: Example) -> Result<Self, Self::Error> {
//         if let Example::Empty = value {
//             Ok(())
//         } else {
//             Err(())
//         }
//     }
// }
//
// fn test() {
//     let x = (String::from("weiner"), String::from("balls"));
//     let example: Example = x.into();
//     let ex: Example = example.into();
//     // let y: (String, String) = example.into();
// }
//
// impl Example {
//     fn new() -> Self {
//         Example::Empty
//     }
// }
//
// impl Example {
//     fn another(&self) -> String {
//         String::new()
//     }
// }
//
// fn something() -> String {
//     let x = Example::new();
//     x.another()
// }
//
// // can also do simple builder implementations
//
// #[derive(Debug, Default)]
// struct Build {
//     a: String,
//     b: usize,
// }
//
// impl Build
// where
//     Self: Default,
// {
//     // the default and the new aren't necessarily needed
//     fn new() -> Self {
//         Self::default()
//     }
//
//     fn a(mut self, value: impl Into<String>) -> Self {
//         self.a = value.into();
//         self
//     }
//
//     fn b(mut self, value: impl Into<usize>) -> Self {
//         self.b = value.into();
//         self
//     }
// }
//
// impl Build {
//     fn formated(&self) -> String {
//         format!("{}, {}", self.a, self.b)
//     }
// }
//
// // something more complex is possibly is adding in generic contraints, not sure how well that works
//
// enum Thing<T> {
//     Item(T),
// }
//
// impl<T> From<T> for Thing<T> {
//     fn from(value: T) -> Self {
//         Self::Item(value)
//     }
// }
