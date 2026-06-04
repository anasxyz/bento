use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprCall, ExprPath, ItemFn, parse_macro_input, visit::Visit};

struct ReactiveCallFinder {
    found: Option<proc_macro2::Span>,
}

impl<'ast> Visit<'ast> for ReactiveCallFinder {
    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if self.found.is_some() {
            return;
        }
        if let Expr::Path(ExprPath { path, .. }) = &*node.func {
            if let Some(ident) = path.get_ident() {
                let name = ident.to_string();
                if name == "state" || name == "effect" || name == "derived" {
                    self.found = Some(ident.span());
                }
            }
        }
        syn::visit::visit_expr_call(self, node);
    }
}

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

/// Marks a function as a bento snippet — a stateless view helper.
/// Snippets cannot contain state(), effect(), or derived().
/// If you need reactive state, use #[component] instead.
#[proc_macro_attribute]
pub fn snippet(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let mut finder = ReactiveCallFinder { found: None };
    syn::visit::visit_block(&mut finder, &input.block);

    if let Some(span) = finder.found {
        return syn::Error::new(
            span,
            "snippets cannot contain reactive state, mark as #[component] instead",
        )
        .to_compile_error()
        .into();
    }

    TokenStream::from(quote! { #input })
}

/// Marks a function as the main entry point for a bento app
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
