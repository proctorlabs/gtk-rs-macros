use crate::glade_xml::GladeRoot;
use crate::metadata::*;
use proc_macro::TokenStream;

pub(crate) fn parse_glade_macro(keys: GladeKeys) -> TokenStream {
    let md = keys.into_metadata();
    let struct_name = md.name;
    let glade_file = md.file;
    let xml_data = GladeRoot::parse(&glade_file.value());
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
