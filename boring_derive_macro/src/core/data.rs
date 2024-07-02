use syn::{punctuated::Punctuated, Token};

use super::{attr, container::Container, context::Context};
use crate::core::container::{AttrField, AttrVariant};

pub enum Data<'a, F, V>
where
    F: AttrField,
    V: AttrVariant,
{
    Struct(Style, Vec<Field<'a, F>>),
    Enum(Vec<Variant<'a, F, V>>),
    Union(Vec<Field<'a, F>>),
}

impl<'a, F, V> Data<'a, F, V>
where
    F: AttrField,
    V: AttrVariant,
{
    pub fn all_fields(&'a self) -> Box<dyn Iterator<Item = &'a Field<'a, F>> + 'a> {
        match self {
            Data::Struct(_, fields) => Box::new(fields.iter()),
            Data::Enum(variants) => Box::new(variants.iter().flat_map(|v| v.fields.iter())),
            Data::Union(fields) => Box::new(fields.iter()),
        }
    }
}

pub struct Variant<'a, F, V>
where
    F: AttrField,
    V: AttrVariant,
{
    pub ident: syn::Ident,
    pub attrs: V,
    pub style: Style,
    pub fields: Vec<Field<'a, F>>,
    pub original: &'a syn::Variant,
}

pub struct Field<'a, F>
where
    F: AttrField,
{
    pub member: syn::Member,
    pub attrs: F,
    pub ty: &'a syn::Type,
    pub original: &'a syn::Field,
}

#[derive(Debug)]
pub enum Style {
    Struct,
    Tuple,
    Newtype,
    Unit,
}
