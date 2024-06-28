use core::panic;
use std::{cell::RefCell, fmt::Display, thread};

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{punctuated::Punctuated, spanned::Spanned, Field, Ident, Path, Token};

#[derive(Copy, Clone)]
pub struct Symbol(&'static str);

const BUILDER: Symbol = Symbol("builder");
const SKIP: Symbol = Symbol("skip");

impl PartialEq<Symbol> for Ident {
    fn eq(&self, other: &Symbol) -> bool {
        self == other.0
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, other: &Symbol) -> bool {
        *self == other.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, other: &Symbol) -> bool {
        self.is_ident(other.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, other: &Symbol) -> bool {
        self.is_ident(other.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

#[derive(Default)]
struct Ctxt {
    errors: RefCell<Option<Vec<syn::Error>>>,
}

impl Ctxt {
    fn new() -> Self {
        Ctxt {
            errors: RefCell::new(Some(Vec::new())),
        }
    }

    fn error_spanned_by<A: ToTokens, T: Display>(&self, obj: A, msg: T) {
        self.errors
            .borrow_mut()
            .as_mut()
            .unwrap()
            .push(syn::Error::new_spanned(obj.into_token_stream(), msg));
    }

    fn syn_error(&self, err: syn::Error) {
        self.errors.borrow_mut().as_mut().unwrap().push(err);
    }

    fn check(self) -> syn::Result<()> {
        let mut errors = self.errors.borrow_mut().take().unwrap().into_iter();

        let mut combined = match errors.next() {
            Some(first) => first,
            None => return Ok(()),
        };

        for rest in errors {
            combined.combine(rest)
        }

        Err(combined)
    }
}

impl Drop for Ctxt {
    fn drop(&mut self) {
        if !thread::panicking() && self.errors.borrow().is_some() {
            panic!("forgot to check for errors");
        }
    }
}

struct Attr<'c, T> {
    cx: &'c Ctxt,
    name: Symbol,
    tokens: TokenStream,
    value: Option<T>,
}

impl<'c, T> Attr<'c, T> {
    fn none(cx: &'c Ctxt, name: Symbol) -> Self {
        Attr {
            cx,
            name,
            tokens: TokenStream::new(),
            value: None,
        }
    }

    fn set<A: ToTokens>(&mut self, obj: A, value: T) {
        let tokens = obj.into_token_stream();

        if self.value.is_some() {
            let msg = format!("duplicate attribute `{}`", self.name);
            self.cx.error_spanned_by(tokens, msg);
        } else {
            self.tokens = tokens;
            self.value = Some(value);
        }
    }

    fn set_opt<A: ToTokens>(&mut self, obj: A, value: Option<T>) {
        if let Some(value) = value {
            self.set(obj, value);
        }
    }

    fn get(self) -> Option<T> {
        self.value
    }

    fn get_with_tokens(self) -> Option<(TokenStream, T)> {
        self.value.map(|v| (self.tokens, v))
    }
}

struct BoolAttr<'c>(Attr<'c, ()>);

impl<'c> BoolAttr<'c> {
    fn none(cx: &'c Ctxt, name: Symbol) -> Self {
        BoolAttr(Attr::none(cx, name))
    }

    fn set_true<A: ToTokens>(&mut self, obj: A) {
        self.0.set(obj, ())
    }

    fn get(&self) -> bool {
        self.0.value.is_some()
    }
}

struct AttrContainer {
    name: String,
    skip: bool,
}

impl AttrContainer {
    fn from_ast(cx: &Ctxt, item: &syn::DeriveInput) -> Self {
        let mut skip = BoolAttr::none(cx, SKIP);

        for attr in &item.attrs {
            if attr.path() != BUILDER {
                continue;
            }

            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            if let Err(err) = attr.parse_nested_meta(|meta| {
                if meta.path == SKIP {
                    skip.set_true(meta.path);
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    return Err(meta.error(format_args!("unknown builder attribute: `{}`", path)));
                }
                Ok(())
            }) {
                cx.syn_error(err)
            }
        }
        AttrContainer {
            name: item.ident.to_string(),
            skip: skip.get(),
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn skip(&self) -> bool {
        self.skip
    }
}

struct Container<'a> {
    ident: syn::Ident,
    attrs: AttrContainer,
    data: Data<'a>,
    generics: &'a syn::Generics,
    original: &'a syn::DeriveInput,
}

enum Data<'a> {
    Enum(Vec<Variant<'a>>),
    Struct(Style, Vec<DataField<'a>>),
}

struct Variant<'a> {
    ident: syn::Ident,
    attrs: AttrVariant,
    style: Style,
    fields: Vec<DataField<'a>>,
    original: &'a syn::Variant,
}

struct DataField<'a> {
    member: syn::Member,
    attrs: AttrField,
    ty: &'a syn::Type,
    original: &'a syn::Field,
}

enum Style {
    Struct,
    Tuple,
    Newtype,
    Unit,
}

impl<'a> Container<'a> {
    fn from_ast(cx: &Ctxt, item: &'a syn::DeriveInput) -> Option<Container<'a>> {
        let mut attrs = AttrContainer::from_ast(cx, item);
        let mut data = match &item.data {
            // syn::Data::Enum(data) => Data::Enum(enum_from_ast(cx, &data.variants, attrs.default())
            syn::Data::Struct(data) => {
                let (style, fields) = struct_from_ast(cx, &data.fields, None);
                Data::Struct(style, fields)
            }
            syn::Data::Enum(_) => {
                cx.error_spanned_by(
                    item,
                    "Deriving the builder pattern for enums is not supported",
                );
                return None;
            }
            syn::Data::Union(_) => {
                cx.error_spanned_by(
                    item,
                    "Deriving the builder pattern for unions is not supported",
                );
                return None;
            }
        };

        // let mut has_flatten = false;
        // match &mut data {
        //     Data::Struct(_, fields) => {
        //
        //     }
        //     Data::Enum(_) => {}
        // }
        let mut item = Container {
            ident: item.ident.clone(),
            attrs,
            data,
            generics: &item.generics,
            original: item,
        };
        check(cx, &mut item);
        Some(item)
    }
}

fn check(cx: &Ctxt, cont: &mut Container) {}

impl<'a> Data<'a> {
    fn all_fields(&'a self) -> Box<dyn Iterator<Item = &'a DataField<'a>> + 'a> {
        match self {
            Data::Enum(variants) => {
                Box::new(variants.iter().flat_map(|variant| variant.fields.iter()))
            }
            Data::Struct(_, fields) => Box::new(fields.iter()),
        }
    }
}

fn struct_from_ast<'a>(
    cx: &Ctxt,
    fields: &'a syn::Fields,
    attrs: Option<&Variant>,
) -> (Style, Vec<DataField<'a>>) {
    match fields {
        syn::Fields::Named(fields) => (Style::Struct, fields_from_ast(cx, &fields.named, attrs)),
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            (Style::Newtype, fields_from_ast(cx, &fields.unnamed, attrs))
        }
        syn::Fields::Unnamed(fields) => (Style::Tuple, fields_from_ast(cx, &fields.unnamed, attrs)),
        syn::Fields::Unit => (Style::Unit, Vec::new()),
    }
}

fn fields_from_ast<'a>(
    cx: &Ctxt,
    fields: &'a Punctuated<syn::Field, Token![,]>,
    attrs: Option<&Variant>,
) -> Vec<DataField<'a>> {
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| DataField {
            member: match &field.ident {
                Some(ident) => syn::Member::Named(ident.clone()),
                None => syn::Member::Unnamed(i.into()),
            },
            attrs: AttrField::from_ast(cx, i, field, attrs),
            ty: &field.ty,
            original: field,
        })
        .collect()
}

struct AttrVariant {
    name: String,
    skip: bool,
}

impl AttrVariant {
    fn from_ast(cx: &Ctxt, variant: &syn::Variant) -> Self {
        let mut skip = BoolAttr::none(cx, SKIP);

        for attr in &variant.attrs {
            if attr.path() != BUILDER {
                continue;
            }

            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            if let Err(err) = attr.parse_nested_meta(|meta| {
                if meta.path == SKIP {
                    skip.set_true(&meta.path);
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    return Err(meta.error(format_args!(
                        "unknown builder variant attribute: `{}`",
                        path
                    )));
                }
                Ok(())
            }) {
                cx.syn_error(err);
            }
        }

        AttrVariant {
            name: variant.ident.to_string(),
            skip: skip.get(),
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn skip(&self) -> bool {
        self.skip
    }
}

struct AttrField {
    name: String,
    skip: bool,
}

// enum ContainerDefault {
//     None,
//     Default,
//     Path(syn::ExprPath),
// }
//
// impl ContainerDefault {
//     fn is_none(&self) -> bool {
//         match self {
//             Self::None => true,
//             _ => false,
//         }
//     }
// }

impl AttrField {
    fn from_ast(cx: &Ctxt, index: usize, field: &syn::Field, attrs: Option<&Variant>) -> Self {
        let mut skip = BoolAttr::none(cx, SKIP);

        let ident = match &field.ident {
            Some(ident) => ident.to_string(),
            None => index.to_string(),
        };

        for attr in &field.attrs {
            if attr.path() != BUILDER {
                continue;
            }

            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            if let Err(err) = attr.parse_nested_meta(|meta| {
                if meta.path == SKIP {
                    skip.set_true(&meta.path);
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

        AttrField {
            name: ident,
            skip: skip.get(),
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn skip(&self) -> bool {
        self.skip
    }
}

pub(crate) fn impl_builder(ast: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let ctxt = Ctxt::new();
    let cont = match Container::from_ast(&ctxt, ast) {
        Some(cont) => cont,
        None => return Err(ctxt.check().unwrap_err()),
    };

    ctxt.check()?;

    let ident = &cont.ident;
    let vis = &ast.vis;

    let (impl_generics, type_generics, where_clause) = cont.generics.split_for_impl();

    let expanded = match cont.data {
        Data::Struct(Style::Struct, fields) => {
            let functions = fields.iter().filter_map(|f| {
                if f.attrs.skip {
                    None
                } else {
                    let field_name = &f.original.ident;
                    let field_ty = f.ty;
                    Some(quote! {
                        #vis fn #field_name (mut self, value: impl Into< #field_ty >) -> Self {
                            self.#field_name = value.into();
                            self
                        }
                    })
                }
            });
            quote! {
                impl #impl_generics #ident #type_generics #where_clause {
                    #(#functions)*
                }

            }
        }
        _ => unimplemented!(),
    };

    Ok(expanded.into())

    // let name = &ast.ident;
    // let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    //
    // let expanded = match ast.data {
    //     syn::Data::Struct(ref data) => {
    //         let functions = data.fields.iter().filter_map(|f| {
    //             if !skip_field(f) {
    //                 f.ident.as_ref().map(|field_name| {
    //                     let field_ty = &f.ty;
    //                     quote_spanned! {f.span() =>
    //                         pub fn #field_name (mut self, value: impl Into< #field_ty >) -> Self {
    //                             self.#field_name = value.into();
    //                             self
    //                         }
    //                     }
    //                 })
    //             } else {
    //                 None
    //             }
    //         });
    //         quote! {
    //             impl #impl_generics #name #type_generics #where_clause {
    //                 #(#functions)*
    //             }
    //
    //         }
    //     }
    //     syn::Data::Enum(ref _data) => {
    //         unimplemented!();
    //     }
    //     syn::Data::Union(ref _data) => {
    //         unimplemented!()
    //     }
    // };
    //
    // // panic!("{:?}", expanded.to_string());
    // expanded.into()
}

fn skip_field(field: &Field) -> bool {
    for attr in &field.attrs {
        if attr.path().is_ident("builder_skip") {
            return true;
        }
    }
    false
}
