use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// generates hasbase for your element struct, which unlocks:
/// - all layout setters (set_width, set_padding, set_align_items, etc.)
/// - all dirty tracking (is_dirty, set_dirty)
/// - layout() and layout_mut_internal() forwarded from base
/// - as_any / as_any_mut for downcasting
///
/// requires a field named `base: base` on the struct
///
/// after deriving, implement element with only what's specific to your element:
///
/// ```ignore
/// #[derive(Element)]
/// pub struct MyElement {
///     base: Base,
///     value: f32,
/// }
///
/// impl Element for MyElement {
///     fn draw_calls(&self, clip: Option<[f32; 4]>, z: i32, opacity: f32) -> Vec<DrawCall> {
///         vec![]
///     }
/// }
/// ```
#[proc_macro_derive(Element)]
pub fn derive_element(input: TokenStream) -> TokenStream {
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
            "#[derive(Element)] requires a field named `base: Base`",
        )
        .to_compile_error()
        .into();
    }

    quote! {
        impl #impl_generics crate::element::base::HasBase for #name #ty_generics #where_clause {
            fn base(&self) -> &crate::element::base::Base { &self.base }
            fn base_mut(&mut self) -> &mut crate::element::base::Base { &mut self.base }
        }

        impl #impl_generics crate::element::element::AsAny for #name #ty_generics #where_clause {
            fn as_any(&self) -> &dyn ::std::any::Any { self }
            fn as_any_mut(&mut self) -> &mut dyn ::std::any::Any { self }
        }
    }
    .into()
}
