use proc_macro2::{Ident, Span};

use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    LitStr, Result,
};

pub(crate) struct GladeKey {
    pub ident: Ident,
    pub colon: Token![:],
    pub val: syn::Expr,
}

impl Parse for GladeKey {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(GladeKey {
            ident: input.parse()?,
            colon: input.parse()?,
            val: input.parse()?,
        })
    }
}

pub(crate) struct GladeKeys {
    pub contents: Punctuated<GladeKey, Token![,]>,
}

impl Parse for GladeKeys {
    fn parse(input: ParseStream) -> Result<Self> {
        let contents = input.parse_terminated(GladeKey::parse)?;
        Ok(GladeKeys { contents })
    }
}

impl GladeKeys {
    pub fn into_metadata(self) -> GladeMetadata {
        let mut result = GladeMetadata {
            file: LitStr::new("app.glade", Span::call_site()),
            name: Ident::new("GladeApp", Span::call_site()),
        };
        for key in self.contents.into_iter() {
            let k = key.ident.to_string();
            let v = key.val;
            match k.as_str() {
                "file" => result.file = parse_quote! (#v),
                "name" => result.name = parse_quote! (#v),
                "connect" => {}
                i => panic!("Unexpected key {}", i),
            };
        }
        result
    }
}

pub(crate) struct GladeMetadata {
    pub file: LitStr,
    pub name: Ident,
}
