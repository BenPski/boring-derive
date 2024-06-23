use core::panic;

use proc_macro::TokenStream;
use proc_macro2::{self, Span};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, token::Token, Data, DataEnum, DeriveInput,
    Fields, Ident,
};

#[proc_macro_derive(From)]
pub fn from_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_from(&ast)
}

// struct Thing<T> {
//  item: T
// }
//
// impl<T> From<T> for Thing<T> {
//  fn from(value: T) -> Self {
//      Self { item: value }
//  }
// }
//
// struct Thing {
//  first: A,
//  second: B,
// }
//
// impl From<(A, B)> for Thing {
//  fn from(value: (A, B)) -> Self {
//      Self { first: value.0, second: value.1 }
//  }
// }
//
// struct Thing(T);
//
// impl<T> From<T> for Thing {
//  fn from(value: T) -> Self {
//      Self(value)
//  }
// }
//
// struct Thing;
//
// impl From<()> for Thing {
//  fn from(value: ()) -> Self {
//      Self
//  }
// }
//

fn impl_from(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let expanded = match ast.data {
        Data::Struct(ref data) => {
            let (from_type, from_body) = handle_fields(name, &data.fields);

            quote! {
                impl #impl_generics From<#from_type> for #name #type_generics #where_clause {
                    fn from(value: #from_type) -> Self {
                        #from_body
                    }
                }
            }
        }
        Data::Enum(ref data) => {
            let variants = data.variants.iter().map(|v| {
                let v_name = &v.ident;
                let (from_type, from_body) = handle_fields(&v_name, &v.fields);
                quote! {
                    impl #impl_generics From<#from_type> for #name #type_generics #where_clause {
                        fn from(value: #from_type) -> Self {
                            Self::#from_body
                        }
                    }
                }
            });
            quote! { #(#variants)* }
        }
        Data::Union(ref data) => unimplemented!(),
    };

    // panic!("{:?}", expanded.to_string());
    expanded.into()
}

fn handle_fields(
    constructor: &Ident,
    fields: &Fields,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let (from_type, from_body) = match fields {
        Fields::Unit => (quote! { () }, quote! { #constructor }),
        Fields::Named(ref fields) => {
            if fields.named.len() == 1 {
                let field = fields.named.iter().nth(0).unwrap();
                let ty = &field.ty;
                let field_name = &field.ident;
                (
                    quote! { #ty },
                    quote! { #constructor { #field_name: value} },
                )
            } else if fields.named.len() == 0 {
                (quote! { () }, quote! { #constructor { }})
            } else {
                let recurse = fields.named.iter().map(|f| {
                    let ty = &f.ty;
                    quote_spanned! {f.span() => #ty}
                });
                let tuple = quote! { ( #(#recurse),* )};
                let recurse = fields.named.iter().enumerate().map(|(i, f)| {
                    let ident = &f.ident;
                    let index = syn::Index::from(i);
                    quote_spanned! {f.span() => #ident: value.#index }
                });
                let body = quote! { #constructor { #(#recurse),* } };
                (tuple, body)
            }
        }
        Fields::Unnamed(ref fields) => {
            if fields.unnamed.len() == 1 {
                let field = fields.unnamed.iter().nth(0).unwrap();
                let ty = &field.ty;
                (quote! { #ty }, quote! { #constructor(value) })
            } else if fields.unnamed.len() == 0 {
                (quote! { () }, quote! { #constructor })
            } else {
                let recurse = fields.unnamed.iter().map(|f| {
                    let ty = &f.ty;
                    quote_spanned! {f.span() => #ty}
                });
                let tuple = quote! { ( #(#recurse),* )};
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = syn::Index::from(i);
                    quote_spanned! {f.span() => value.#index }
                });
                let body = quote! { #constructor(#(#recurse),*) };
                (tuple, body)
            }
        }
    };
    (from_type, from_body)
}
