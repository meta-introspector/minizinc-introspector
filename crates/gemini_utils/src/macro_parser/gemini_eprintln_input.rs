use syn::{parse::{Parse, ParseStream}, LitStr, Token, Ident, Expr, Path, PathSegment};
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
                if input.peek(Ident) {
                    let first_token: Ident = input.parse()?;
                    if input.peek(Token![:]) {
                        input.parse::<Token![:]>()?; // Consume the colon
                        let expr: Expr = input.parse()?;
                        named_args.push((first_token, expr));
                    } else if input.peek(Token![=]) {
                        input.parse::<Token![=]>()?; // Consume the equals
                        let expr: Expr = input.parse()?;
                        named_args.push((first_token, expr));
                    } else {
                        // It's a positional argument that started with an Ident
                        let mut segments = Punctuated::new();
                        segments.push(PathSegment::from(first_token));
                        let path = Path {
                            leading_colon: None,
                            segments,
                        };
                        positional_args.push(Expr::Path(syn::ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path,
                        }));
                    }
                } else {
                    // It's a positional argument that doesn't start with an Ident (e.g., a literal, a function call)
                    let expr: Expr = input.parse()?;
                    positional_args.push(expr);
                }

                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?; // Consume the comma
                } else if !input.is_empty() {
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
