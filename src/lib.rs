//! Procedural macros for serializing and deserializing enums using labeled and custom string
//! representations. Implemented for compatibility with [serde](https://serde.rs).
//!
//! # Examples
//!
//! ## Labeled Strings
//! ```
//! #[cfg(feature = "alloc")]
//! extern crate alloc;
//!
//! use serde_string_enum::{
//!     DeserializeLabeledStringEnum,
//!     SerializeLabeledStringEnum,
//! };
//!
//! #[derive(Debug, PartialEq, SerializeLabeledStringEnum, DeserializeLabeledStringEnum)]
//! enum Type {
//!     #[string = "Grass"]
//!     Grass,
//!     #[string = "Fire"]
//!     #[alias = "Flame"]
//!     Fire,
//!     #[string = "Water"]
//!     Water,
//! }
//!
//! fn main() -> serde_json::Result<()> {
//!     let j = serde_json::to_string(&Type::Grass)?;
//!     assert_eq!(j, "\"Grass\"");
//!     let t: Type = serde_json::from_str(&j)?;
//!     assert_eq!(t, Type::Grass);
//!
//!     // Alias strings.
//!     let t: Type = serde_json::from_str("\"Flame\"")?;
//!     assert_eq!(t, Type::Fire);
//!
//!     // Case-insensitive conversion also works.
//!     if cfg!(feature = "unicase") {
//!         let t: Type = serde_json::from_str("\"water\"")?;
//!         assert_eq!(t, Type::Water);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Enums with Display and FromStr
//! ```
//! use core::{
//!     fmt::Display,
//!     str::FromStr,
//! };
//! use serde_string_enum::{
//!     DeserializeStringEnum,
//!     SerializeStringEnum,
//! };
//!
//! #[derive(Debug, PartialEq, SerializeStringEnum, DeserializeStringEnum)]
//! enum Move {
//!     Stay,
//!     Forward(u8),
//!     Left(u8),
//! }
//!
//! impl Display for Move {
//!     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//!         match self {
//!             Self::Stay => write!(f, "S"),
//!             Self::Forward(n) => write!(f, "F{n}"),
//!             Self::Left(n) => write!(f, "L{n}"),
//!         }
//!     }
//! }
//!
//! impl FromStr for Move {
//!     type Err = String;
//!     fn from_str(s: &str) -> Result<Self, Self::Err> {
//!         Ok(match &s[0..1] {
//!             "S" => Self::Stay,
//!             "F" => Self::Forward(s[1..].parse::<u8>().map_err(|err| err.to_string())?),
//!             "L" => Self::Left(s[1..].parse::<u8>().map_err(|err| err.to_string())?),
//!             _ => return Err(format!("invalid move {s}")),
//!         })
//!     }
//! }
//!
//! fn main() -> serde_json::Result<()> {
//!     let j = serde_json::to_string(&Move::Forward(10))?;
//!     assert_eq!(j, "\"F10\"");
//!     let m: Move = serde_json::from_str(&j)?;
//!     assert_eq!(m, Move::Forward(10));
//!
//!     let moves: Vec<Move> = serde_json::from_str("[\"S\",\"F2\",\"S\",\"L4\"]")?;
//!     assert_eq!(
//!         moves,
//!         vec![Move::Stay, Move::Forward(2), Move::Stay, Move::Left(4)]
//!     );
//!
//!     Ok(())
//! }
//! ```

#![no_std]

extern crate alloc;
extern crate proc_macro;

use alloc::fmt::format;
use parse::{
    Input,
    LabeledStringInput,
};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{
    quote,
    ToTokens,
};
use syn::{
    parse_macro_input,
    Ident,
};

mod parse;

/// Procedural macro for serializing enums as strings.
///
/// Enums deriving this macro must have implemented [`core::fmt::Display`].
#[proc_macro_derive(SerializeStringEnum)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);
    let ident = input.ident;

    TokenStream::from(quote! {
        impl serde::Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> where S: serde::Serializer {
                serializer.collect_str(self)
            }
        }
    })
}

/// Procedural macro for deserializing strings to enum variants.
///
/// Enums deriving this macro must have implemented [`core::str::FromStr`].
#[proc_macro_derive(DeserializeStringEnum)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);
    let ident = input.ident;

    let visitor_ident = Ident::new(&format(format_args!("{ident}Visitor")), Span::call_site());

    TokenStream::from(quote! {
        struct #visitor_ident;

        impl<'de> serde::de::Visitor<'de> for #visitor_ident {
            type Value = #ident;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_fmt(format_args!("a valid {} string value", stringify!(#ident)))
            }

           fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E> where E: serde::de::Error {
            match Self::Value::from_str(&v) {
                Ok(v) => Ok(v),
                Err(_) => Err(E::invalid_value(serde::de::Unexpected::Str(&v), &self)),
            }
           }
        }

        impl<'de> serde::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error> where D: serde::Deserializer<'de> {
                deserializer.deserialize_str(#visitor_ident)
            }
        }
    })
}

/// Procedural macro for serializing enums as strings, where each variant is labeled with a
/// `#[string = ...]` attribute.
#[proc_macro_derive(SerializeLabeledStringEnum, attributes(string))]
pub fn derive_labeled_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LabeledStringInput);
    let ident = input.ident;

    let match_variants = input.variants.iter().map(|variant| {
        let string = variant.attrs.string.as_ref().unwrap();
        let variant = &variant.ident;
        quote! {
            Self::#variant => write!(f, #string),
        }
    });

    TokenStream::from(quote! {
        impl core::fmt::Display for #ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    #(#match_variants)*
                }
            }
        }

        impl serde::Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> where S: serde::Serializer {
                serializer.collect_str(self)
            }
        }
    })
}

fn wrap_unicase<T>(t: &T) -> proc_macro2::TokenStream
where
    T: ToTokens,
{
    if cfg!(feature = "unicase") {
        quote! {
            unicase::UniCase::new(#t)
        }
    } else {
        quote! {
            #t
        }
    }
}

/// Procedural macro for deserializing strings to enum variants, where each variant is labeled with
/// a `#[string = ...]` attribute.
#[proc_macro_derive(DeserializeLabeledStringEnum, attributes(string, alias))]
pub fn derive_labeled_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LabeledStringInput);
    let call_site = Span::call_site();
    let ident = input.ident;
    let visitor_ident = Ident::new(&format(format_args!("{ident}Visitor")), call_site);
    let input_ident = Ident::new("s", call_site);

    let match_variants = input.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let alias_match = variant.attrs.aliases.iter().map(|alias| {
            let alias = wrap_unicase(alias);
            quote! {
                if s == #alias {
                    return Ok(Self::#variant_ident)
                }
            }
        });
        let string = variant.attrs.string.as_ref().unwrap();
        let string = wrap_unicase(string);
        quote! {
            if #input_ident == #string {
                return Ok(Self::#variant_ident)
            }
            #(#alias_match)*
        }
    });

    let error_type = if cfg!(feature = "std") {
        quote! {
            std::string::String
        }
    } else if cfg!(feature = "alloc") {
        quote! {
            alloc::string::String
        }
    } else {
        quote! {
            &'static str
        }
    };

    let error = if cfg!(feature = "std") {
        quote! {
            std::format!("invalid {}: {}", stringify!(#ident), #input_ident)
        }
    } else if cfg!(feature = "alloc") {
        quote! {
            alloc::fmt::format(format_args!("invalid {}: {}", stringify!(#ident), #input_ident))
        }
    } else {
        quote! {
            "invalid value"
        }
    };
    let unicase_input = wrap_unicase(&input_ident);

    TokenStream::from(quote! {
        impl core::str::FromStr for #ident {
            type Err = #error_type;
            fn from_str(#input_ident: &str) -> core::result::Result<Self, Self::Err> {
                let #input_ident = #unicase_input;
                #(#match_variants)*
                Err(#error)
            }
        }

        struct #visitor_ident;

        impl<'de> serde::de::Visitor<'de> for #visitor_ident {
            type Value = #ident;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_fmt(format_args!("a valid {} string value", stringify!(#ident)))
            }

           fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E> where E: serde::de::Error {
            use core::str::FromStr;
            match Self::Value::from_str(&v) {
                Ok(v) => Ok(v),
                Err(_) => Err(E::invalid_value(serde::de::Unexpected::Str(&v), &self)),
            }
           }
        }

        impl<'de> serde::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error> where D: serde::Deserializer<'de> {
                deserializer.deserialize_str(#visitor_ident)
            }
        }
    })
}
