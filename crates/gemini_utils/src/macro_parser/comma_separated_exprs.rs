use syn::{punctuated::Punctuated, token::Comma, Expr};

pub struct CommaSeparatedExprs {
    pub exprs: Punctuated<Expr, Comma>,
}

impl syn::parse::Parse for CommaSeparatedExprs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(CommaSeparatedExprs {
            exprs: Punctuated::parse_terminated(input)?,
        })
    }
}
