use syn::{parse::{Parse, ParseStream}, LitStr, Token, Ident, Expr};
//use syn::punctuated::Punctuated;

pub struct GeminiEprintlnInput {
    pub format_string: LitStr,
    pub named_args: Vec<(Ident, Expr)>,
    pub positional_args: Vec<Expr>,
}

impl Parse for GeminiEprintlnInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let format_string: LitStr = input.parse()?;

        let mut named_args = Vec::new();
        let mut positional_args = Vec::new();

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?; // Consume the comma

            while !input.is_empty() {
                // Attempt to parse as a named argument (ident: expr)
                if input.peek(Ident) && input.peek2(Token![:]) {
                    let ident: Ident = input.parse()?;
                    input.parse::<Token![:]>()?; // Consume the colon
                    let expr: Expr = input.parse()?;
                    named_args.push((ident, expr));
                } else if input.peek(Ident) && input.peek2(Token![=]) {
                    // Keep support for ident = expr for backward compatibility if needed, or remove
                    let ident: Ident = input.parse()?;
                    input.parse::<Token![=]>()?; // Consume the equals
                    let expr: Expr = input.parse()?;
                    named_args.push((ident, expr));
                }
                else {
                    // It's a positional argument
                    let expr: Expr = input.parse()?;
                    positional_args.push(expr);
                }

                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?; // Consume the comma
                } else if !input.is_empty() {
                    // If there are more tokens but no comma, it's a syntax error
                    return Err(input.error("Expected comma or end of input"));
                }
            }
        }

        Ok(GeminiEprintlnInput {
            format_string,
            named_args,
            positional_args,
        })
    }
}