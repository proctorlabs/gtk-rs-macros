#![recursion_limit = "128"]

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};

use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    LitStr,
};

pub(crate) struct GladeKey {
    pub ident: Ident,
    pub colon: Token![:],
    pub val: syn::Expr,
}

impl Parse for GladeKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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
    fn parse(input: ParseStream) -> syn::Result<Self> {
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
    file: LitStr,
    name: Ident,
}

fn parse_glade_macro(keys: GladeKeys) -> TokenStream {
    let md = keys.into_metadata();
    let struct_name = md.name;
    let glade_file = md.file;
    let result = quote! {
        #[derive(Debug)]
        pub struct #struct_name {
            app: gtk::Application,
        }

        impl #struct_name {
            pub fn start() {
                let new_app = std::sync::Arc::new(#struct_name {
                    app: gtk::Application::new(Some("com.glade_app.demo"), Default::default())
                        .expect("Initialization failed...")
                });

                let send_app = new_app.clone();
                new_app.app.connect_activate(move |app| {
                    let builder = Builder::new_from_string(include_str!(#glade_file));
                    let window: ApplicationWindow = builder.get_object("main").expect("Couldn't get window");
                    window.set_application(Some(&send_app.app));
                    window.show_all();
                });

                new_app.app.run(&args().collect::<Vec<_>>());
            }
        }
    };

    result.into()
}

#[proc_macro]
pub fn glade_app(item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();
    parse_glade_macro(ast)
}
