use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, Expr, ExprLit, Fields, Lit, Meta, Token};
use syn::punctuated::Punctuated;

const TEXTUREVIEW_ATTRIBUTE: &str = "textureview";

#[proc_macro_derive(ActorsTexturesCollection, attributes(textureview))]
pub fn actors_textures_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
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
                let mut angle_value = None;
                let mut type_value = String::from("");

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
                                    angle_value = Some(key.value());
                                }
                            }
                            Meta::NameValue(named_value) if named_value.path.is_ident("type") => {
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

                let mut image_value = None;
                let mut atlas_layout_value = None;

                if type_value == "image" {
                    image_value = Some(quote! {self.#field_name});
                } else if type_value == "atlas_layout" {
                    atlas_layout_value = Some(quote! {self.#field_name});
                }
                quote! {
                FieldInfo {
                    actor: #actor_value,
                    action: #action_value,
                    angle: #angle_value,
                    image: #image_value,
                    atlas_layout: #atlas_layout_value,
                    }
                }
            });
            let expanded = quote! {
                use bevy_2dviewange_common::{ActorsTexturesLoader, FieldInfo};

                impl ActorsTexturesLoader for #struct_name {
                    fn get_all(&self) -> Vec<FieldInfo> {
                        vec![#( #field_info ),*]
                    }
                }
            };
            return Ok(proc_macro2::TokenStream::from(expanded));
        }
    }

    return Err(vec![]);
}
