use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::Error;

use crate::core::{
    attr::{Attr, BoolAttr},
    container::{AttrContainer, AttrField, AttrVariant, Container},
    context::Context,
    data::{Data, Style},
    symbol::Symbol,
};

const BUILDER: Symbol = Symbol("builder");
const SKIP: Symbol = Symbol("skip");
const NO_INTO: Symbol = Symbol("no_into");
const PREFIX: Symbol = Symbol("prefix");
const RENAME: Symbol = Symbol("rename");

struct BuilderContainer {
    prefix: Option<String>,
}

struct BuilderVariant;

struct BuilderField {
    skip: bool,
    no_into: bool,
    rename: Option<String>,
}

impl AttrContainer for BuilderContainer {
    fn from_ast(cx: &Context, item: &syn::DeriveInput) -> Self {
        let mut prefix = Attr::none(cx, PREFIX);

        for attr in &item.attrs {
            if attr.path() != BUILDER {
                continue;
            }

            if let Err(err) = attr.parse_nested_meta(|meta| {
                if meta.path == PREFIX {
                    let expr: syn::Expr = meta.value()?.parse()?;
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }) = expr
                    {
                        prefix.set(&meta.path, s.value());
                    } else {
                        return Err(meta.error(format_args!(
                            "prefix must be a string not `{}`",
                            expr.to_token_stream().to_string()
                        )));
                    }
                } else {
                    let path = meta.path.to_token_stream().to_string();
                    return Err(meta.error(format_args!(
                        "unexpected builder container attribute: `{}`",
                        path
                    )));
                }
                Ok(())
            }) {
                cx.syn_error(err);
            }
        }

        BuilderContainer {
            prefix: prefix.get(),
        }
    }
}

impl AttrVariant for BuilderVariant {
    fn from_ast(_cx: &Context, _variant: &syn::Variant) -> Self {
        BuilderVariant
    }
}

impl AttrField for BuilderField {
    fn from_ast(cx: &Context, _index: usize, field: &syn::Field) -> Self {
        let mut skip = BoolAttr::none(cx, SKIP);
        let mut no_into = BoolAttr::none(cx, NO_INTO);
        let mut rename = Attr::none(cx, RENAME);

        for attr in &field.attrs {
            if attr.path() != BUILDER {
                continue;
            }

            if let Err(err) = attr.parse_nested_meta(|meta| {
                if meta.path == SKIP {
                    skip.set_true(&meta.path);
                } else if meta.path == NO_INTO {
                    no_into.set_true(&meta.path);
                } else if meta.path == RENAME {
                    let expr: syn::Expr = meta.value()?.parse()?;
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }) = expr
                    {
                        rename.set(&meta.path, s.value());
                    } else {
                        return Err(meta.error(format_args!(
                            "rename must be a string not `{}`",
                            expr.to_token_stream().to_string()
                        )));
                    }
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    return Err(
                        meta.error(format_args!("unknown builder field attribute: `{}`", path))
                    );
                }
                Ok(())
            }) {
                cx.syn_error(err);
            }
        }

        BuilderField {
            skip: skip.get(),
            no_into: no_into.get(),
            rename: rename.get(),
        }
    }
}

pub(crate) fn impl_builder(ast: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let ctxt = Context::new();
    let cont: Option<Container<BuilderField, BuilderVariant, BuilderContainer>> =
        Container::from_ast(&ctxt, ast);
    let cont = match cont {
        Some(cont) => cont,
        None => return Err(ctxt.check().unwrap_err()),
    };

    ctxt.check()?;

    let prefix = &cont.attrs.prefix.unwrap_or(String::new());
    let ident = &cont.ident;
    let vis = &ast.vis;

    let (impl_generics, type_generics, where_clause) = cont.generics.split_for_impl();

    let expanded = match cont.data {
        Data::Struct(Style::Struct, fields) => {
            let functions = fields.iter().filter_map(|f| {
                if f.attrs.skip {
                    None
                } else {
                    let method_name = if let Some(rename) = &f.attrs.rename {
                        format_ident!("{}", rename)
                    } else {
                        format_ident!("{}{}", prefix, &f.original.ident.as_ref().unwrap())
                    };
                    let field_name = &f.original.ident;
                    let field_ty = f.ty;
                    if f.attrs.no_into {
                        Some(quote! {
                            #vis fn #method_name (mut self, value: #field_ty) -> Self {
                                self.#field_name = value;
                                self
                            }
                        })
                    } else {
                        Some(quote! {
                            #vis fn #method_name (mut self, value: impl Into< #field_ty >) -> Self {
                                self.#field_name = value.into();
                                self
                            }
                        })
                    }
                }
            });
            quote! {
                impl #impl_generics #ident #type_generics #where_clause {
                    #(#functions)*
                }

            }
        }
        Data::Struct(Style::Newtype, _) => {
            return Err(Error::new(
                ast.ident.span(),
                format_args!("deriving builder pattern not supported for newtype style structs"),
            ))
        }
        Data::Struct(Style::Unit, _) => {
            return Err(Error::new(
                ast.ident.span(),
                format_args!("deriving builder pattern not supported for unit-like structs"),
            ))
        }
        Data::Struct(Style::Tuple, _) => {
            return Err(Error::new(
                ast.ident.span(),
                format_args!("deriving builder pattern not supported for tuple structs"),
            ))
        }
        Data::Enum(_) => {
            return Err(Error::new(
                ast.ident.span(),
                format_args!("deriving builder pattern not supported for enums"),
            ))
        }
        Data::Union(_) => {
            return Err(Error::new(
                ast.ident.span(),
                format_args!("deriving builder pattern not supported for unions"),
            ))
        }
    };

    Ok(expanded)
}
