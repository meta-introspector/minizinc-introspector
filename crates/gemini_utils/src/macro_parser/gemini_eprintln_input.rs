use syn::{parse::{Parse, ParseStream}, LitStr, Token, Ident, Expr};
use syn::punctuated::Punctuated;

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
                let expr: Expr = input.parse()?;
                if let Expr::Assign(assign) = expr {
                    // It's a named argument (key = value)
                    if let Expr::Path(path) = *assign.left {
                        if let Some(ident) = path.path.get_ident() {
                            named_args.push((ident.clone(), *assign.right));
                        } else {
                            return Err(input.error("Expected identifier on left side of assignment"));
                        }
                    } else {
                        return Err(input.error("Expected identifier on left side of assignment"));
                    }
                } else {
                    // It's a positional argument
                    positional_args.push(expr);
                }

                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?; // Consume the comma
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