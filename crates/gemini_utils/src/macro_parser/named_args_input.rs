use syn::{parse::{Parse, ParseStream}, LitStr, Token, ExprAssign, punctuated::Punctuated};
use quote::ToTokens;

pub struct Input {
    pub format_string: LitStr,
    pub named_args: Option<Punctuated<ExprAssign, Token![,]>>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let format_string: LitStr = input.parse()?;

        let named_args = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?; // Consume the comma
            Some(Punctuated::parse_terminated(input)?)
        } else {
            None
        };

        Ok(Input { format_string, named_args })
    }
}
