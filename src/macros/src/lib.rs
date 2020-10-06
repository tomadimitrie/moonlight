extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn moonlight_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    (quote! {
        use crate::types::native_view::NativeView;
        extern "C" {
            fn render(tree: *const NativeView);
        }

        #[no_mangle]
        pub extern "C" fn run_app() {
            #input
            let view = #name();
            unsafe {
                render(view.to_native());
            }
        }
    })
    .into()
}
