use alloc::{
    fmt::format,
    vec::Vec,
};
use proc_macro2::{
    Ident,
    Span,
};
use syn::{
    parse::{
        Parse,
        ParseStream,
        Result,
    },
    Data,
    DeriveInput,
    Error,
    Expr,
    Fields,
    Lit,
    LitStr,
    Meta,
    MetaNameValue,
};

#[derive(Clone)]
pub struct VariantAttrs {
    pub string: Option<LitStr>,
    pub aliases: Vec<LitStr>,
}

impl VariantAttrs {
    pub fn new() -> Self {
        Self {
            string: None,
            aliases: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct Variant {
    pub ident: Ident,
    pub attrs: VariantAttrs,
    pub fields: Fields,
}

pub struct Input {
    pub ident: Ident,
    pub variants: Vec<Variant>,
}

pub struct LabeledStringInput {
    pub ident: Ident,
    pub variants: Vec<Variant>,
}

fn get_string_literal_from_name_value_attr(
    span: Span,
    attribute_name: &str,
    name_value: &MetaNameValue,
) -> Result<LitStr> {
    match &name_value.value {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            Lit::Str(str) => Ok(str.clone()),
            _ => Err(Error::new(
                span,
                format(format_args!(
                    "\"{attribute_name}\" attribute must be a string literal"
                )),
            )),
        },
        _ => Err(Error::new(
            span,
            format(format_args!(
                "\"{attribute_name}\" attribute must be a string literal"
            )),
        )),
    }
}

fn parse_variant_attrs(span: Span, variant: &syn::Variant) -> Result<VariantAttrs> {
    let mut attrs = VariantAttrs::new();
    for attr in &variant.attrs {
        if let Meta::NameValue(name_value) = &attr.meta {
            if name_value.path.is_ident("string") {
                attrs.string = Some(get_string_literal_from_name_value_attr(
                    span, "string", name_value,
                )?)
            } else if name_value.path.is_ident("alias") {
                attrs.aliases.push(get_string_literal_from_name_value_attr(
                    span, "alias", name_value,
                )?)
            }
        }
    }
    Ok(attrs)
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let call_site = Span::call_site();
        let derive_input = DeriveInput::parse(input)?;
        let data = match derive_input.data {
            Data::Enum(data) => data,
            _ => return Err(Error::new(call_site, "input must be an enum")),
        };

        let variants = data
            .variants
            .into_iter()
            .map(|variant| {
                let attrs = parse_variant_attrs(call_site, &variant)?;
                Ok(Variant {
                    ident: variant.ident,
                    attrs,
                    fields: variant.fields,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        if variants.is_empty() {
            return Err(Error::new(call_site, "enum must have at least one variant"));
        }

        Ok(Input {
            ident: derive_input.ident,
            variants,
        })
    }
}

impl Parse for LabeledStringInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let call_site = Span::call_site();
        let input = Input::parse(input)?;

        if !input.variants.iter().all(|variant| match variant.fields {
            Fields::Unit => true,
            _ => false,
        }) {
            return Err(Error::new(call_site, "all variants must be a unit variant"));
        }

        if !input
            .variants
            .iter()
            .all(|variant| variant.attrs.string.is_some())
        {
            return Err(Error::new(
                call_site,
                "all variants must have \"string\" attribute",
            ));
        }

        Ok(LabeledStringInput {
            ident: input.ident,
            variants: input.variants,
        })
    }
}
