#![recursion_limit = "128"]

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;

mod glade_macro;
mod glade_xml;
mod metadata;

#[proc_macro]
pub fn glade_app(item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();
    glade_macro::parse_glade_macro(ast)
}
