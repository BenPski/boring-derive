use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data};

pub(crate) fn impl_builder(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let expanded = match ast.data {
        Data::Struct(ref data) => {
            let functions = data.fields.iter().map(|f| {
                let field_name = &f.ident;
                let field_ty = &f.ty;
                quote_spanned! {f.span() =>
                    pub fn #field_name (mut self, value: impl Into< #field_ty >) -> Self {
                        self.#field_name = value.into();
                        self
                    }
                }
            });
            quote! {
                impl #impl_generics #name #type_generics #where_clause {
                    #(#functions)*
                }

            }
        }
        Data::Enum(ref _data) => {
            unimplemented!();
        }
        Data::Union(ref _data) => {
            unimplemented!()
        }
    };

    // panic!("{:?}", expanded.to_string());
    expanded.into()
}
