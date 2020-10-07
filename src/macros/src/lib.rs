extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, ItemFn};

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
                render(view.to_native());
            }
        }
    })
    .into()
}

#[proc_macro_derive(NativeImpl, attributes(convert, next))]
pub fn native_impl(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let data = ast.data;
    if let Data::Struct(struct_ast) = data {
        let fields_quote = struct_ast
            .fields
            .iter()
            .filter(|field| {
                field
                    .attrs
                    .iter()
                    .find(|attr| attr.path.get_ident().unwrap().to_string() == "convert")
                    .is_some()
            })
            .map(|field| {
                let ident = &field.ident;
                let ident_string = &ident.as_ref().unwrap().to_string();
                let ty_string = &field.ty.clone().into_token_stream().to_string();
                let value_quote = if ty_string == "String" {
                    quote! {
                        let value = CString::new(self.#ident.clone()).unwrap();
                        let value_ptr = value.as_ptr() as *const c_void;
                        attributes.push(value_ptr);
                        forget(value);
                    }
                } else {
                    quote! {
                        let value = Box::new(self.#ident.clone());
                        let value_ptr = Box::into_raw(value) as *const c_void;
                        attributes.push(value_ptr);
                    }
                };
                quote! {
                     let key = CString::new(#ident_string).unwrap();
                     let key_ptr = key.as_ptr() as *const c_void;
                     attributes.push(key_ptr);
                     forget(key);

                     #value_quote

                     let kind = CString::new(#ty_string).unwrap();
                     let kind_ptr = kind.as_ptr() as *const c_void;
                     attributes.push(kind_ptr);
                     forget(kind);

                }
            })
            .collect::<Vec<_>>();
        let children_quote = if let Some(children_field) = struct_ast
            .fields
            .iter()
            .filter(|field| {
                field
                    .attrs
                    .iter()
                    .find(|attr| attr.path.get_ident().unwrap().to_string() == "next")
                    .is_some()
            })
            .collect::<Vec<_>>()
            .first()
        {
            let ident = &children_field.ident;
            quote! {
                let children = self.#ident
                    .iter()
                    .map(|child| child.to_native())
                    .collect::<Vec<_>>();
            }
        } else {
            quote! {
                let children = Vec::<Box<dyn View>>::new()
                    .iter()
                    .map(|child| child.to_native())
                    .collect::<Vec<_>>();;
            }
        };
        return (quote! {
            use crate::NativeView;
            use std::{
                ffi::{c_void, CString},
                mem::forget
            };
            impl View for Container {
                fn to_native(&self) -> *const NativeView {
                    let mut attributes = Vec::<*const c_void>::new();

                    #(#fields_quote)*

                    let attributes_size = attributes.len();
                    let attributes_ptr = attributes.as_ptr();
                    forget(attributes);

                    #children_quote
                    let children_size = children.len();
                    let children_ptr = children.as_ptr();
                    forget(children);

                    Box::into_raw(Box::new(NativeView {
                        attributes_ptr,
                        attributes_size,
                        children_ptr,
                        children_size,
                    })) as *const NativeView
                }
            }
        })
        .into();
    }
    unreachable!()
}
