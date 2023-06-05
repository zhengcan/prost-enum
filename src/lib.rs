extern crate proc_macro;

mod parse;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::parse::Input;

#[proc_macro_derive(Serialize_enum)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);
    let ident = input.ident;

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

#[proc_macro_derive(Deserialize_enum, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);
    let ident = input.ident;

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
                        match #ident::from_str_name(v) {
                            Some(e) => Ok(e),
                            None => Err(serde::de::Error::custom(format!(
                                "unknown enum value: {}",
                                v
                            ))),
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
