use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// Marks a function as a bento component
/// Used for view ownership and resource cleanup
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let vis = &input.vis;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let body = &input.block;

    TokenStream::from(quote! {
        #vis fn #name(#inputs) #output {
            let __owner = bento_ui::Owner::new();
            let __view = (move || #body)();
            let __owner = __owner.collect();
            bento_ui::view::OwnedView::new(__owner, __view)
        }
    })
}

/// Marks a function as the main entry point for a bento app
/// Expands to a wasm_bindgen start function or a regular main function based on target arch
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let body = &input.block;

    TokenStream::from(quote! {
        #[cfg(not(target_arch = "wasm32"))]
        fn main() #body

        #[cfg(target_arch = "wasm32")]
        #[wasm_bindgen::prelude::wasm_bindgen(start)]
        pub fn wasm_entry() {
            console_error_panic_hook::set_once();
            #body
        }
    })
}
