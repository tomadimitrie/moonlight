extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn moonlight_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemFn);
    let name = &ast.sig.ident;
    (quote! {
        use crate::types::native_view::NativeView;
        extern "C" {
            fn render(tree: *const NativeView);
        }

        #[no_mangle]
        pub extern "C" fn run_app() {
            #ast
            let view = #name();
            unsafe {
                render(NativeView::from_view(&view).into_ptr());
            }
        }
    })
    .into()
}
