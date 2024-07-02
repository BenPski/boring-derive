use std::clone;

use proc_macro2::TokenStream;
use proc_macro2::{self};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    meta::ParseNestedMeta, punctuated::Punctuated, spanned::Spanned, Attribute, Error, Fields,
    FieldsNamed, FieldsUnnamed, Ident, Meta, Token,
};

use crate::core::container::AttrField;
use crate::core::container::AttrVariant;
use crate::core::container::Container;
use crate::core::data::Data;
use crate::core::data::Field;
use crate::core::data::Style;
use crate::core::{
    attr::{Attr, BoolAttr},
    context::Context,
    symbol::Symbol,
};

const FROM: Symbol = Symbol("from");
const SKIP: Symbol = Symbol("skip");

struct FromVariant {
    skip: bool,
}

impl AttrVariant for FromVariant {
    fn from_ast(cx: &Context, variant: &syn::Variant) -> Self {
        let mut skip = BoolAttr::none(cx, SKIP);

        for attr in &variant.attrs {
            if attr.path() != FROM {
                continue;
            }

            if let Err(err) = attr.parse_nested_meta(|meta| {
                if meta.path == SKIP {
                    skip.set_true(&meta.path);
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    return Err(
                        meta.error(format_args!("unknown from variant attribute: `{}`", path))
                    );
                }
                Ok(())
            }) {
                cx.syn_error(err);
            }
        }

        FromVariant { skip: skip.get() }
    }
}

struct FromField;

impl AttrField for FromField {
    fn from_ast(cx: &Context, index: usize, field: &syn::Field) -> Self {
        FromField
    }
}

pub(crate) fn impl_from(ast: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let ctxt = Context::new();
    let cont: Option<Container<FromField, FromVariant>> = Container::from_ast(&ctxt, ast);
    let cont = match cont {
        Some(cont) => cont,
        None => return Err(ctxt.check().unwrap_err()),
    };

    ctxt.check()?;
    let ident = &cont.ident;
    let (impl_generics, type_generics, where_clause) = cont.generics.split_for_impl();

    let expanded = match cont.data {
        Data::Struct(style, fields) => {
            let (from_type, from_body) = gen_info(ident, &style, &fields);
            quote! {
                impl #impl_generics From<#from_type> for #ident #type_generics #where_clause {
                    fn from(value: #from_type) -> Self {
                        #from_body
                    }
                }
            }
        }

        Data::Enum(variants) => {
            let variants = variants.iter().filter_map(|v| {
                if v.attrs.skip {
                    None
                } else {
                let v_name = &v.ident;
                let (from_type, from_body) = gen_info(v_name, &v.style, &v.fields);
                Some(quote! {
                    impl #impl_generics From<#from_type> for #ident #type_generics #where_clause {
                        fn from(value: #from_type) -> Self {
                            #ident::#from_body
                        }
                    }
                })
                }
            });

            quote! { #(#variants)* }
        }
        Data::Union(_) => {
            return Err(Error::new(
                cont.ident.span(),
                format_args!("deriving from not supported for unions"),
            ));
        }
    };

    // panic!("{:?}", expanded.to_string());
    Ok(expanded.into())
}

fn gen_info<'a, F: AttrField>(
    constructor: &Ident,
    style: &Style,
    fields: &Vec<Field<'a, F>>,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    match style {
        Style::Unit => (quote! {()}, quote! {#constructor}),
        Style::Newtype => {
            let field = &fields[0];
            let ty = field.ty;
            (quote! {#ty}, quote! {#constructor(value)})
        }
        Style::Tuple => {
            if fields.len() == 1 {
                let field = &fields[0];
                let ty = field.ty;
                (quote! {#ty}, quote! {#constructor(value)})
            } else {
                let recurse = fields.iter().map(|f| {
                    let ty = &f.ty;
                    quote_spanned! {f.original.span() => #ty}
                });
                let from_type = quote! { ( #(#recurse),* )};
                let recurse = fields.iter().enumerate().map(|(i, f)| {
                    let index = syn::Index::from(i);
                    quote_spanned! {f.original.span() => value.#index}
                });
                let from_body = quote! { #constructor(#(#recurse),*) };
                (from_type, from_body)
            }
        }
        Style::Struct => {
            if fields.len() == 1 {
                let field = &fields[0];
                let name = &field.original.ident;
                let ty = field.ty;
                (quote! {#ty}, quote! {#constructor { #name: value }})
            } else {
                let recurse = fields.iter().map(|f| {
                    let ty = &f.ty;
                    quote_spanned! {f.original.span() => #ty}
                });
                let from_type = quote! { ( #(#recurse),* )};
                let recurse = fields.iter().enumerate().map(|(i, f)| {
                    let index = syn::Index::from(i);
                    let name = &f.original.ident;
                    quote_spanned! {f.original.span() => #name: value.#index}
                });
                let from_body = quote! { #constructor { #(#recurse),* } };

                (from_type, from_body)
            }
        }
    }
}
