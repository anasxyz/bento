use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// generates hasbase for your widget struct, which unlocks:
/// - all layout setters (set_width, set_padding, set_align_items, etc.)
/// - all dirty tracking (is_dirty, set_dirty)
/// - layout() and layout_mut_internal() forwarded from base
/// - as_any / as_any_mut for downcasting
///
/// requires a field named `base: base` on the struct
///
/// after deriving, implement widget with only what's specific to your widget:
///
/// ```ignore
/// #[derive(Widget)]
/// pub struct MyWidget {
///     base: Base,
///     value: f32,
/// }
///
/// impl Widget for MyWidget {
///     fn draw_calls(&self, clip: Option<[f32; 4]>, z: i32, opacity: f32) -> Vec<DrawCall> {
///         vec![]
///     }
/// }
/// ```
#[proc_macro_derive(Widget)]
pub fn derive_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let has_base_field = match &input.data {
        syn::Data::Struct(s) => s
            .fields
            .iter()
            .any(|f| f.ident.as_ref().map(|i| i == "base").unwrap_or(false)),
        _ => false,
    };

    if !has_base_field {
        return syn::Error::new_spanned(
            &input.ident,
            "#[derive(Widget)] requires a field named `base: Base`",
        )
        .to_compile_error()
        .into();
    }

    // when used inside bento itself, the crate root is `crate::`.
    // when used outside bento, it's `bento::`.
    // we detect this by checking if the cargo_pkg_name env var is "bento".
    let is_internal = std::env::var("CARGO_PKG_NAME")
        .map(|n| n == "bento")
        .unwrap_or(false);

    let root: TokenStream2 = if is_internal {
        quote! { crate }
    } else {
        quote! { bento }
    };

    quote! {
        impl #impl_generics #root::HasBase for #name #ty_generics #where_clause {
            fn base(&self) -> &#root::Base { &self.base }
            fn base_mut(&mut self) -> &mut #root::Base { &mut self.base }
        }

        impl #impl_generics #root::AsAny for #name #ty_generics #where_clause {
            fn as_any(&self) -> &dyn ::std::any::Any { self }
            fn as_any_mut(&mut self) -> &mut dyn ::std::any::Any { self }
        }
    }
    .into()
}
