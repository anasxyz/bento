use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{DeriveInput, Field, Fields, Type, Visibility, parse_macro_input};

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

    let fields = match &input.data {
        syn::Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            _ => panic!("Widget derive only supports named fields"),
        },
        _ => panic!("Widget derive only supports structs"),
    };

    let setters: TokenStream2 = fields
        .iter()
        .filter_map(|f| {
            let field_name = f.ident.as_ref()?;
            if field_name == "base" {
                return None;
            }
            match &f.vis {
                Visibility::Public(_) => {}
                _ => return None,
            }
            let setter_name = format_ident!("set_{}", field_name);
            let ty = &f.ty;
            Some(quote! {
                pub fn #setter_name(&mut self, value: #ty) -> &mut Self {
                    self.#field_name = value;
                    self.base_mut().dirty = true;
                    self
                }
            })
        })
        .collect();

    quote! {
        impl #impl_generics #root::widget::AsAny for #name #ty_generics #where_clause {
            fn as_any(&self) -> &dyn ::std::any::Any { self }
            fn as_any_mut(&mut self) -> &mut dyn ::std::any::Any { self }
        }

        impl #impl_generics #root::HasBase for #name #ty_generics #where_clause {
            fn base(&self) -> &#root::Base {
                &self.base
            }
            fn base_mut(&mut self) -> &mut #root::Base {
                &mut self.base
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            #setters
        }
    }
    .into()
}
