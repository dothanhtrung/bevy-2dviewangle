// Copyright 2024 Trung Do <dothanhtrung@pm.me>

use proc_macro::TokenStream;

use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Data, Expr, ExprLit, Fields, Lit, Meta, Token};

const TEXTUREVIEW_ATTRIBUTE: &str = "textureview";
#[proc_macro_derive(ActorsTexturesCollection, attributes(textureview))]
pub fn actors_textures_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_actors_textures(ast).unwrap_or_default().into()
}

fn impl_actors_textures(
    ast: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, Vec<syn::Error>> {
    let struct_name = &ast.ident;

    if let Data::Struct(data_struct) = &ast.data {
        if let Fields::Named(fields) = &data_struct.fields {
            let field_info = fields.named.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let mut actor_value = None;
                let mut action_value = None;
                let mut angle_value = String::new();
                let mut type_value = String::new();

                for attr in field
                    .attrs
                    .iter()
                    .filter(|attribute| attribute.path().is_ident(TEXTUREVIEW_ATTRIBUTE))
                {
                    let view_meta_list =
                        attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated);

                    for attribute in view_meta_list.unwrap() {
                        match attribute {
                            Meta::NameValue(named_value) if named_value.path.is_ident("actor") => {
                                if let Expr::Lit(ExprLit {
                                    lit: Lit::Int(key), ..
                                }) = &named_value.value
                                {
                                    actor_value = Some(key.base10_parse::<u64>().unwrap());
                                }
                            }
                            Meta::NameValue(named_value) if named_value.path.is_ident("action") => {
                                if let Expr::Lit(ExprLit {
                                    lit: Lit::Int(key), ..
                                }) = &named_value.value
                                {
                                    action_value = Some(key.base10_parse::<u16>().unwrap());
                                }
                            }
                            Meta::NameValue(named_value) if named_value.path.is_ident("angle") => {
                                if let Expr::Lit(ExprLit {
                                    lit: Lit::Str(key), ..
                                }) = &named_value.value
                                {
                                    angle_value = key.value();
                                }
                            }
                            Meta::NameValue(named_value) if named_value.path.is_ident("handle") => {
                                if let Expr::Lit(ExprLit {
                                    lit: Lit::Str(key), ..
                                }) = &named_value.value
                                {
                                    type_value = key.value();
                                }
                            }
                            _ => {}
                        }
                    }
                }

                let field_value = quote! {&self.#field_name};

                if type_value == "image" {
                    quote! {
                        FieldInfo {
                            actor: #actor_value.into(),
                            action: #action_value.into(),
                            angle: Some(#angle_value.to_string()),
                            image: Some(#field_value),
                            atlas_layout: None,
                        }
                    }
                } else {
                    quote! {
                        FieldInfo {
                            actor: #actor_value.into(),
                            action: #action_value.into(),
                            angle: Some(#angle_value.to_string()),
                            image: None,
                            atlas_layout: Some(#field_value),
                        }
                    }
                }
            });
            let expanded = quote! {
                #[automatically_derived]
                impl ActorsTexturesCollection for #struct_name {
                    fn get_all(&self) -> Vec<FieldInfo> {
                        vec![#( #field_info ),*]
                    }
                }
            };
            return Ok(expanded);
        }
    }

    return Err(vec![]);
}
