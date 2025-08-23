use syn::{parse::{Parse, ParseStream}, LitStr, Token, Ident, Expr};
use syn::punctuated::Punctuated;

pub struct GeminiEprintlnInput {
    pub format_string: LitStr,
    pub args: Vec<(Ident, Expr)>, // Store named arguments as (key, value)
}

impl Parse for GeminiEprintlnInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let format_string: LitStr = input.parse()?;

        let mut args = Vec::new();
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?; // Consume the comma

            let parsed_args = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;

            for expr in parsed_args {
                if let Expr::Assign(assign) = expr {
                    if let Expr::Path(path) = *assign.left {
                        if let Some(ident) = path.path.get_ident() {
                            args.push((ident.clone(), *assign.right));
                        } else {
                            return Err(input.error("Expected identifier on left side of assignment"));
                        }
                    } else {
                        return Err(input.error("Expected identifier on left side of assignment"));
                    }
                } else {
                    return Err(input.error("Expected named argument in `key = value` format"));
                }
            }
        }

        Ok(GeminiEprintlnInput { format_string, args })
    }
}