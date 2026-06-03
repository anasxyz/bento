use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

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
