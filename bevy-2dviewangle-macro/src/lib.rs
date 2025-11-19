// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use proc_macro::TokenStream;
use std::collections::HashMap;

use quote::{
    format_ident,
    quote,
};
use syn::punctuated::Punctuated;
use syn::{
    Data,
    Expr,
    ExprLit,
    Fields,
    Lit,
    Meta,
    Token,
};
use xxhash_rust::xxh3::xxh3_64;

macro_rules! numberize {
    ($key: expr, $map: expr, $value: expr, $enum_into: expr, $enums: expr) => {{
        let num;
        let key_str = $key.value();
        if $map.contains_key(&key_str) {
            num = *$map.get(&key_str).unwrap();
        } else {
            num = xxh3_64(key_str.as_bytes());
            $map.insert(key_str, num);

            let enum_name = capitalize_first_letter(&$key.value());
            let variant_name = syn::Ident::new(&enum_name, $key.span());
            $enum_into.push(quote! {Self::#variant_name => #num});
            $enums.push(quote! {#variant_name});
        }
        $value = quote! {Some(#num)};
    }};
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
            let mut actor_map = HashMap::new();
            let mut action_map = HashMap::new();
            let mut actor_nums = Vec::new();
            let mut action_nums = Vec::new();
            let mut actor_enums = Vec::new();
            let mut action_enums = Vec::new();

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
                                    numberize!(key, actor_map, actor_value, actor_nums, actor_enums);
                                }
                            }
                            Meta::NameValue(named_value) if named_value.path.is_ident("action") => {
                                if let Expr::Lit(ExprLit { lit: Lit::Str(key), .. }) = &named_value.value {
                                    numberize!(key, action_map, action_value, action_nums, action_enums);
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
                use bevy_2dviewangle::*;

                #[automatically_derived]
                impl View2dCollection for #struct_name {
                    fn get_all(&self) -> Vec<(
                        Option<u64>,
                        Option<u64>,
                        Option<Angle>,
                        Option<&Handle<Image>>,
                        Option<&Handle<TextureAtlasLayout>>,
                    )> {
                        vec![#( #fields_info ),*]
                    }
                }

                #[derive(Eq, PartialEq, Clone)]
                pub enum #actor_enum_name {
                    #( #actor_enums ),*
                }

                impl Into<u64> for #actor_enum_name {
                    fn into(self) -> u64 {
                        match self {
                            #( #actor_nums ),*
                        }
                    }
                }

                #[derive(Eq, PartialEq, Clone)]
                pub enum #action_enum_name {
                    #( #action_enums ),*
                }

                impl Into<u64> for #action_enum_name {
                    fn into(self) -> u64 {
                        match self {
                            #( #action_nums ),*
                        }
                    }
                }
            };
            return Ok(expanded);
        }
    }

    Err(vec![])
}
