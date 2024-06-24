use proc_macro::TokenStream;
use proc_macro2::{self};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, Fields, FieldsNamed, FieldsUnnamed, Ident};

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

pub(crate) fn impl_from(ast: &syn::DeriveInput) -> TokenStream {
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
                let (from_type, from_body) = handle_fields(v_name, &v.fields);
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
        Data::Union(ref _data) => {
            unimplemented!()
        }
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
        Fields::Named(ref fields) => from_fields_named(constructor, fields),
        Fields::Unnamed(ref fields) => from_fields_unnamed(constructor, fields),
    };
    (from_type, from_body)
}

fn from_fields_named(
    constructor: &Ident,
    fields: &FieldsNamed,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    if fields.named.len() == 1 {
        let field = fields.named.iter().next().unwrap();
        let ty = &field.ty;
        let field_name = &field.ident;
        (
            quote! { #ty },
            quote! { #constructor { #field_name: value} },
        )
    } else if fields.named.is_empty() {
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

fn from_fields_unnamed(
    constructor: &Ident,
    fields: &FieldsUnnamed,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    if fields.unnamed.len() == 1 {
        let field = fields.unnamed.iter().next().unwrap();
        let ty = &field.ty;
        (quote! { #ty }, quote! { #constructor(value) })
    } else if fields.unnamed.is_empty() {
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
