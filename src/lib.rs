extern crate proc_macro;

mod parse;

use parse::ProstEnum;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn enhance(_args: TokenStream, input: TokenStream) -> TokenStream {
    let prost_enum = {
        let input = input.clone();
        parse_macro_input!(input as ProstEnum)
    };

    let mut output = proc_macro2::TokenStream::new();
    match prost_enum.repr {
        Some(_) => {
            output.extend(Some(quote! {
                #[derive(prost_enum::Serialize_enum, prost_enum::Deserialize_enum)]
            }));
            #[cfg(feature = "sea-orm")]
            output.extend(Some(quote! {
                #[derive(sea_orm::entity::prelude::EnumIter, sea_orm::entity::prelude::DeriveActiveEnum)]
                #[sea_orm(rs_type = "i32", db_type = "Integer")]
            }));
        }
        None => output.extend(Some(quote! {
            #[derive(serde::Serialize, serde::Deserialize)]
        })),
    }
    output.extend(proc_macro2::TokenStream::from(input));
    output.into()
}

#[proc_macro_derive(Serialize_enum)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ProstEnum);

    match input.repr {
        Some(_) => gen_serialize(input.ident),
        None => TokenStream::from(quote! {}),
    }
}

#[proc_macro_derive(Deserialize_enum, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ProstEnum);

    match input.repr {
        Some(_) => gen_deserialize(input.ident),
        None => TokenStream::from(quote! {}),
    }
}

fn gen_serialize(ident: Ident) -> TokenStream {
    TokenStream::from(quote! {
        impl serde::Serialize for #ident {
            #[allow(clippy::use_self)]
            fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer
            {
                let value = self.as_str_name();
                serde::Serialize::serialize(&value, serializer)
            }
        }
    })
}

fn gen_deserialize(ident: Ident) -> TokenStream {
    TokenStream::from(quote! {
        impl<'de> serde::Deserialize<'de> for #ident {
            #[allow(clippy::use_self)]
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct discriminant;

                impl<'de> serde::de::Visitor<'de> for discriminant {
                    type Value = #ident;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(formatter, "a string or an integer")
                    }

                    fn visit_str<R>(self, v: &str) -> Result<Self::Value, R>
                    where
                        R: serde::de::Error,
                    {
                        if v.is_empty() {
                            match #ident::from_i32(0) {
                                Some(e) => Ok(e),
                                None => Err(serde::de::Error::custom(format!(
                                    "unknown enum value: {}",
                                    v
                                )))
                            }
                        } else {
                            match #ident::from_str_name(v) {
                                Some(e) => Ok(e),
                                None => Err(serde::de::Error::custom(format!(
                                    "unknown enum value: {}",
                                    v
                                ))),
                            }
                        }
                    }

                    fn visit_i64<R>(self, v: i64) -> Result<Self::Value, R>
                    where
                        R: serde::de::Error,
                    {
                        match #ident::from_i32(v as i32) {
                            Some(e) => Ok(e),
                            None => Err(serde::de::Error::custom(format!(
                                "unknown enum value: {}",
                                v
                            )))
                        }
                    }

                    fn visit_u64<R>(self, v: u64) -> Result<Self::Value, R>
                    where
                        R: serde::de::Error,
                    {
                        match #ident::from_i32(v as i32) {
                            Some(e) => Ok(e),
                            None => Err(serde::de::Error::custom(format!(
                                "unknown enum value: {}",
                                v
                            )))
                        }
                    }
                }

                deserializer.deserialize_any(discriminant)
            }
        }
    })
}
