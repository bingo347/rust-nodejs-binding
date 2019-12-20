extern crate proc_macro;

use find_crate::find_crate;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

const CRATE_NAME: &'static str = "nodejs-binding";

#[proc_macro_attribute]
pub fn nodejs_export(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemFn);
    let result = wrap_export_fn(&ast);
    println!("{}", result.to_string());
    result
}

fn wrap_export_fn(fn_ast: &ItemFn) -> TokenStream {
    let span = fn_ast.sig.ident.span();
    let crate_name = Ident::new(&find_crate(|s| s == CRATE_NAME).unwrap().name, span.clone());
    let fn_body = &fn_ast.block;
    let fn_name = &fn_ast.sig.ident;
    let wrapped_fn_name = Ident::new(&format!("nb_{}", item_to_string(fn_name)), span.clone());
    TokenStream::from(quote! {
        pub extern "C" fn #fn_name(
            env: #crate_name::sys::napi_env,
            cb_info: #crate_name::sys::napi_callback_info
        ) -> #crate_name::sys::napi_value {
            fn #wrapped_fn_name(
                ctx: #crate_name::Context,
                args: &[#crate_name::JSValue]
            ) -> #crate_name::NodeJSResult<#crate_name::JSValue> { #fn_body }
            let args = Vec::new();
            #wrapped_fn_name(env.into(), &args).unwrap();
            std::ptr::null_mut()
        }
    })
}

fn item_to_string<T: quote::ToTokens>(item: &T) -> String {
    TokenStream::from(quote! { #item }).to_string()
}
