use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Widget)]
pub fn derive_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let is_internal = std::env::var("CARGO_PKG_NAME")
        .map(|n| n == "bento_ui")
        .unwrap_or(false);

    let root: TokenStream2 = if is_internal {
        quote! { crate }
    } else {
        quote! { bento_ui }
    };

    quote! {
        impl #impl_generics #root::widget::AsAny for #name #ty_generics #where_clause {
            fn as_any(&self) -> &dyn ::std::any::Any { self }
            fn as_any_mut(&mut self) -> &mut dyn ::std::any::Any { self }
        }
    }
    .into()
}
