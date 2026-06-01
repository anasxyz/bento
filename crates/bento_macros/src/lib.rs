use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let name = &func.sig.ident;
    let inputs = &func.sig.inputs;
    let body = &func.block;
    let vis = &func.vis;

    quote! {
        #vis fn #name(#inputs) -> impl ::bento_ui::View {
            let __owner = ::bento_ui::reactive::owner::Owner::new();
            let __view = (move || #body)();
            ::bento_ui::OwnedView::new(__owner.collect(), __view)
        }
    }
    .into()
}
