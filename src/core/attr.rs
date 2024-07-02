use proc_macro2::TokenStream;
use quote::ToTokens;

use super::{context::Context, symbol::Symbol};

pub struct Attr<'c, T> {
    cx: &'c Context,
    name: Symbol,
    tokens: TokenStream,
    value: Option<T>,
}

impl<'c, T> Attr<'c, T> {
    pub fn none(cx: &'c Context, name: Symbol) -> Self {
        Attr {
            cx,
            name,
            tokens: TokenStream::new(),
            value: None,
        }
    }

    pub fn set<A: ToTokens>(&mut self, obj: A, value: T) {
        let tokens = obj.into_token_stream();

        if self.value.is_some() {
            let msg = format!("duplicate attribute `{}`", self.name);
            self.cx.error_spanned_by(tokens, msg);
        } else {
            self.tokens = tokens;
            self.value = Some(value);
        }
    }

    #[allow(dead_code)]
    pub fn set_opt<A: ToTokens>(&mut self, obj: A, value: Option<T>) {
        if let Some(value) = value {
            self.set(obj, value);
        }
    }

    #[allow(dead_code)]
    pub fn get(self) -> Option<T> {
        self.value
    }

    #[allow(dead_code)]
    pub fn get_with_tokens(self) -> Option<(TokenStream, T)> {
        self.value.map(|v| (self.tokens, v))
    }
}

pub struct BoolAttr<'c>(Attr<'c, ()>);

impl<'c> BoolAttr<'c> {
    pub fn none(cx: &'c Context, name: Symbol) -> Self {
        BoolAttr(Attr::none(cx, name))
    }

    pub fn set_true<A: ToTokens>(&mut self, obj: A) {
        self.0.set(obj, ())
    }

    pub fn get(&self) -> bool {
        self.0.value.is_some()
    }
}
