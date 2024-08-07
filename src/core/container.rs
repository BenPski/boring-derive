use syn::{punctuated::Punctuated, Token};

use super::{context::Context, data::Data};
use crate::core::data::{Field, Style, Variant};

pub trait AttrContainer {
    fn from_ast(cx: &Context, item: &syn::DeriveInput) -> Self;
}

pub trait AttrVariant {
    fn from_ast(cx: &Context, variant: &syn::Variant) -> Self;
}

pub trait AttrField {
    fn from_ast(cx: &Context, index: usize, field: &syn::Field) -> Self;
}

// parsing the ast into the data structure and the attributes
#[allow(dead_code)]
pub struct Container<'a, F, V, C>
where
    F: AttrField,
    V: AttrVariant,
    C: AttrContainer,
{
    pub ident: syn::Ident,
    pub attrs: C,
    pub data: Data<'a, F, V>,
    pub generics: &'a syn::Generics,
    pub original: &'a syn::DeriveInput,
}

impl<'a, F, V, C> Container<'a, F, V, C>
where
    F: AttrField,
    V: AttrVariant,
    C: AttrContainer,
{
    pub fn from_ast(cx: &Context, item: &'a syn::DeriveInput) -> Option<Container<'a, F, V, C>> {
        let attrs = C::from_ast(cx, item);
        let data = match &item.data {
            syn::Data::Struct(data) => {
                let (style, fields) = struct_from_ast(cx, &data.fields);
                Data::Struct(style, fields)
            }
            syn::Data::Enum(data) => Data::Enum(enum_from_ast(cx, &data.variants)),
            syn::Data::Union(data) => Data::Union(union_from_ast(cx, &data.fields)),
        };

        let item = Container {
            ident: item.ident.clone(),
            attrs,
            data,
            generics: &item.generics,
            original: item,
        };
        Some(item)
    }
}

fn enum_from_ast<'a, F, V>(
    cx: &Context,
    variants: &'a Punctuated<syn::Variant, Token![,]>,
) -> Vec<Variant<'a, F, V>>
where
    F: AttrField,
    V: AttrVariant,
{
    let variants: Vec<_> = variants
        .iter()
        .map(|variant| {
            let attrs = V::from_ast(cx, variant);
            let (style, fields) = struct_from_ast(cx, &variant.fields);
            Variant {
                ident: variant.ident.clone(),
                attrs,
                style,
                fields,
                original: variant,
            }
        })
        .collect();

    variants
}

fn struct_from_ast<'a, A>(cx: &Context, fields: &'a syn::Fields) -> (Style, Vec<Field<'a, A>>)
where
    A: AttrField,
{
    match fields {
        syn::Fields::Unit => (Style::Unit, Vec::new()),
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            (Style::Newtype, fields_from_ast(cx, &fields.unnamed))
        }
        syn::Fields::Unnamed(fields) => (Style::Tuple, fields_from_ast(cx, &fields.unnamed)),
        syn::Fields::Named(fields) => (Style::Struct, fields_from_ast(cx, &fields.named)),
    }
}

fn union_from_ast<'a, A>(cx: &Context, fields: &'a syn::FieldsNamed) -> Vec<Field<'a, A>>
where
    A: AttrField,
{
    fields_from_ast(cx, &fields.named)
}

fn fields_from_ast<'a, A>(
    cx: &Context,
    fields: &'a Punctuated<syn::Field, Token![,]>,
) -> Vec<Field<'a, A>>
where
    A: AttrField,
{
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| Field {
            member: match &field.ident {
                Some(ident) => syn::Member::Named(ident.clone()),
                None => syn::Member::Unnamed(i.into()),
            },
            attrs: A::from_ast(cx, i, field),
            ty: &field.ty,
            original: field,
        })
        .collect()
}
