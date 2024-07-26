// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use proc_macro::TokenStream;
use std::collections::HashMap;

use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{Data, Expr, ExprLit, Fields, Lit, Meta, Token};

macro_rules! enumize {
    ($k: expr, $e: expr, $m: expr, $c: expr, $v: expr) => {{
        let num;
        let key_str = capitalize_first_letter(&$k.value());
        if $m.contains_key(&key_str) {
            num = *$m.get(&key_str).unwrap();
        } else {
            $c += 1;
            let variant_name = syn::Ident::new(&key_str, $k.span());
            $e.push(quote! {#variant_name});
            $m.insert(key_str, $c);
            num = $c;
        }
        $v = quote! {Some(#num)};
    }}
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

const TEXTUREVIEW_ATTRIBUTE: &str = "textureview";
#[proc_macro_derive(View2dCollection, attributes(textureview))]
pub fn actors_textures_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_actors_textures(ast).unwrap_or_default().into()
}

fn impl_actors_textures(ast: syn::DeriveInput) -> Result<proc_macro2::TokenStream, Vec<syn::Error>> {
    let struct_name = &ast.ident;

    if let Data::Struct(data_struct) = &ast.data {
        if let Fields::Named(fields) = &data_struct.fields {
            let mut fields_info = Vec::new();
            let mut actor_enum = Vec::new();
            let mut actor_map = HashMap::new();
            let mut actor_count: u64 = 0;
            let mut action_enum = Vec::new();
            let mut action_map = HashMap::new();
            let mut action_count: u16 = 0;

            for field in fields.named.iter() {
                let field_name = field.ident.as_ref().unwrap();
                let mut actor_value = quote! {None};
                let mut action_value = quote! {None};
                let mut angle_value = quote! {None};
                let mut image_value = quote! {None};
                let mut atlas_layout_value = quote! {None};

                for attr in field
                    .attrs
                    .iter()
                    .filter(|attribute| attribute.path().is_ident(TEXTUREVIEW_ATTRIBUTE))
                {
                    let view_meta_list = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated);

                    for attribute in view_meta_list.unwrap() {
                        match attribute {
                            Meta::NameValue(named_value) if named_value.path.is_ident("actor") => {
                                if let Expr::Lit(ExprLit { lit: Lit::Str(key), .. }) = &named_value.value {
                                    enumize!(key, actor_enum, actor_map, actor_count, actor_value);
                                }
                            }
                            Meta::NameValue(named_value) if named_value.path.is_ident("action") => {
                                if let Expr::Lit(ExprLit { lit: Lit::Str(key), .. }) = &named_value.value {
                                    enumize!(key, action_enum, action_map, action_count, action_value);
                                }
                            }
                            Meta::NameValue(named_value) if named_value.path.is_ident("angle") => {
                                if let Expr::Lit(ExprLit { lit: Lit::Str(key), .. }) = &named_value.value {
                                    let key_str = capitalize_first_letter(&key.value());
                                    let variant_name = syn::Ident::new(&key_str, key.span());
                                    angle_value = quote! {Some(Angle::#variant_name)};
                                }
                            }
                            _ => {}
                        }
                    }
                }

                match &field.ty {
                    ty if quote!(#ty).to_string() == "Handle < Image >" => {
                        image_value = quote! {Some(&self.#field_name)}
                    }
                    ty if quote!(#ty).to_string() == "Handle < TextureAtlasLayout >" => {
                        atlas_layout_value = quote! {Some(&self.#field_name)}
                    }
                    _ => {}
                }

                let field_info = quote! {
                    (
                        #actor_value,
                        #action_value,
                        #angle_value,
                        #image_value,
                        #atlas_layout_value,
                    )
                };

                fields_info.push(field_info);
            }

			let actor_enum_name = format_ident!("Actor{}", struct_name);
			let action_enum_name = format_ident!("Action{}", struct_name);
            let expanded = quote! {
                use bevy_2dviewangle::Angle;

                #[derive(Default, Eq, PartialEq)]
                #[repr(u64)]
                pub enum #actor_enum_name {
                    #[default]
                    Any,
                    #( #actor_enum ),*
                }

                impl From<#actor_enum_name> for u64 {
                    fn from(actor: #actor_enum_name) -> Self {
                        actor as u64
                    }
                }

                #[derive(Default, Eq, PartialEq)]
                #[repr(u16)]
                pub enum #action_enum_name {
                    #[default]
                    Any,
                    #( #action_enum ),*
                }

                impl From<#action_enum_name> for u16 {
                    fn from(action: #action_enum_name) -> Self {
                        action as u16
                    }
                }

                #[automatically_derived]
                impl View2dCollection for #struct_name {
                    fn get_all(&self) -> Vec<(
                        Option<u64>,
                        Option<u16>,
                        Option<Angle>,
                        Option<&Handle<Image>>,
                        Option<&Handle<TextureAtlasLayout>>,
                    )> {
                        vec![#( #fields_info ),*]
                    }
                }
            };
            return Ok(expanded);
        }
    }

    return Err(vec![]);
}
