use syn::{
    parse::{Parse, ParseStream},
    DeriveInput, Ident, Meta,
};

#[derive(Debug)]
pub struct ProstEnum {
    pub ident: Ident,
    pub repr: Option<Ident>,
}

impl Parse for ProstEnum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let derive_input = DeriveInput::parse(input)?;

        let mut repr = None;
        for attr in derive_input.attrs {
            if attr.path().is_ident("repr") {
                if let Meta::List(_) = &attr.meta {
                    let ty: Ident = attr.parse_args()?;
                    repr = Some(ty);
                    break;
                }
            }
        }

        Ok(Self {
            ident: derive_input.ident,
            repr,
        })
    }
}
