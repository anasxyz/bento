use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{DeriveInput, Fields, Visibility, parse_macro_input};

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

    let mut setters = Vec::new();
    let mut animate_methods = Vec::new();
    let mut pre_update_lines = Vec::new();

    for f in fields.iter() {
        let field_name = match f.ident.as_ref() {
            Some(n) => n,
            None => continue,
        };

        if field_name == "base" {
            continue;
        }

        match &f.vis {
            Visibility::Public(_) => {}
            _ => continue,
        }

        let ty = &f.ty;
        let setter_name = format_ident!("set_{}", field_name);
        let field_str = field_name.to_string();
        let field_str_static = quote! { #field_str };

        if is_f32(ty) {
            let animate_name = format_ident!("animate_{}", field_name);
            let stop_name = format_ident!("stop_{}_animation", field_name);
            let set_transition_name = format_ident!("set_transition_{}", field_name);
            let clear_transition_name = format_ident!("clear_transition_{}", field_name);

            setters.push(quote! {
                pub fn #setter_name(&mut self, value: f32) -> &mut Self {
                    let transition = self.base.transitions.get(#field_str_static)
                        .copied()
                        .or(self.base.default_transition);
                    if let Some((duration, easing)) = transition {
                        self.base.animations.insert(#field_str_static, #root::Animation {
                            from: #root::AnimatableValue::Float(self.#field_name),
                            to: #root::AnimatableValue::Float(value),
                            duration,
                            elapsed: 0.0,
                            easing,
                            loop_mode: #root::LoopMode::Once,
                            on_start: None,
                            on_tick: None,
                            on_complete: None,
                        });
                    } else {
                        self.#field_name = value;
                    }
                    self.base.dirty = true;
                    self
                }
            });

            animate_methods.push(quote! {
                pub fn #animate_name(&mut self, to: f32, duration: f32, easing: #root::Easing, loop_mode: #root::LoopMode) -> &mut Self {
                    self.base.animations.insert(#field_str_static, #root::Animation {
                        from: #root::AnimatableValue::Float(self.#field_name),
                        to: #root::AnimatableValue::Float(to),
                        duration,
                        elapsed: 0.0,
                        easing,
                        loop_mode,
                        on_start: None,
                        on_tick: None,
                        on_complete: None,
                    });
                    self.base.dirty = true;
                    self
                }

                pub fn #set_transition_name(&mut self, duration: f32, easing: #root::Easing) -> &mut Self {
                    self.base.transitions.insert(#field_str_static, (duration, easing));
                    self
                }

                pub fn #clear_transition_name(&mut self) -> &mut Self {
                    self.base.transitions.remove(#field_str_static);
                    self
                }

                pub fn #stop_name(&mut self) {
                    self.base.stop_animation(#field_str_static);
                }
            });

            pre_update_lines.push(quote! {
                if let #root::AnimatableValue::Float(v) = self.base.animated_value(#field_str_static, #root::AnimatableValue::Float(self.#field_name)) {
                    self.#field_name = v;
                }
            });
        } else if is_color(ty) {
            let animate_name = format_ident!("animate_{}", field_name);
            let stop_name = format_ident!("stop_{}_animation", field_name);
            let set_transition_name = format_ident!("set_transition_{}", field_name);
            let clear_transition_name = format_ident!("clear_transition_{}", field_name);

            setters.push(quote! {
                pub fn #setter_name(&mut self, value: [f32; 4]) -> &mut Self {
                    let transition = self.base.transitions.get(#field_str_static)
                        .copied()
                        .or(self.base.default_transition);
                    if let Some((duration, easing)) = transition {
                        self.base.animations.insert(#field_str_static, #root::Animation {
                            from: #root::AnimatableValue::Color(self.#field_name),
                            to: #root::AnimatableValue::Color(value),
                            duration,
                            elapsed: 0.0,
                            easing,
                            loop_mode: #root::LoopMode::Once,
                            on_start: None,
                            on_tick: None,
                            on_complete: None,
                        });
                    } else {
                        self.#field_name = value;
                    }
                    self.base.dirty = true;
                    self
                }
            });

            animate_methods.push(quote! {
                pub fn #animate_name(&mut self, to: [f32; 4], duration: f32, easing: #root::Easing, loop_mode: #root::LoopMode) -> &mut Self {
                    self.base.animations.insert(#field_str_static, #root::Animation {
                        from: #root::AnimatableValue::Color(self.#field_name),
                        to: #root::AnimatableValue::Color(to),
                        duration,
                        elapsed: 0.0,
                        easing,
                        loop_mode,
                        on_start: None,
                        on_tick: None,
                        on_complete: None,
                    });
                    self.base.dirty = true;
                    self
                }

                pub fn #set_transition_name(&mut self, duration: f32, easing: #root::Easing) -> &mut Self {
                    self.base.transitions.insert(#field_str_static, (duration, easing));
                    self
                }

                pub fn #clear_transition_name(&mut self) -> &mut Self {
                    self.base.transitions.remove(#field_str_static);
                    self
                }

                pub fn #stop_name(&mut self) {
                    self.base.stop_animation(#field_str_static);
                }
            });

            pre_update_lines.push(quote! {
                if let #root::AnimatableValue::Color(v) = self.base.animated_value(#field_str_static, #root::AnimatableValue::Color(self.#field_name)) {
                    self.#field_name = v;
                }
            });
        } else {
            setters.push(quote! {
                pub fn #setter_name(&mut self, value: #ty) -> &mut Self {
                    self.#field_name = value;
                    self.base.dirty = true;
                    self
                }
            });
        }
    }

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
            fn pre_update(&mut self) {
                #(#pre_update_lines)*
                if self.base.tick() {
                    self.base.dirty = true;
                }
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            #(#setters)*
            #(#animate_methods)*
        }
    }
    .into()
}

fn is_f32(ty: &syn::Type) -> bool {
    if let syn::Type::Path(p) = ty {
        if let Some(seg) = p.path.segments.last() {
            return seg.ident == "f32";
        }
    }
    false
}

fn is_color(ty: &syn::Type) -> bool {
    if let syn::Type::Array(a) = ty {
        if let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(n),
            ..
        }) = &a.len
        {
            if n.base10_parse::<usize>().unwrap_or(0) == 4 {
                return is_f32(&a.elem);
            }
        }
    }
    false
}
