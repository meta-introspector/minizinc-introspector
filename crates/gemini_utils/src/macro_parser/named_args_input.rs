use syn::{parse::{Parse, ParseStream}, LitStr, Token, ExprAssign, punctuated::Punctuated};
//use quote::ToTokens;

pub struct Input {
    pub format_string: LitStr,
    pub named_args: Option<Punctuated<ExprAssign, Token![,]>>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let format_string: LitStr = input.parse()?;
        // Debug print for format_string
        eprintln!("DEBUG: Parsed format_string: {:?}", format_string.value());

        let named_args = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?; // Consume the comma
            let args = Punctuated::parse_terminated(input)?;
            // Debug print for named_args
            eprintln!("DEBUG: Parsed named_args: {:?}", args);
            Some(args)
        } else {
            // Debug print when no named_args
            eprintln!("DEBUG: No named_args found.");
            None
        };

        Ok(Input { format_string, named_args })
    }
}
