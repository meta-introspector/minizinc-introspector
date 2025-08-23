//use proc_macro2::TokenStream as ProcMacro2TokenStream; // This import might become unused
use std::iter::Peekable;
use std::str::Chars;
use std::collections::HashMap;

pub struct ProcessingContext<'a> {
    pub chars: &'a mut Peekable<Chars<'a>>,
    pub current_segment: &'a mut String,
    pub emojis: &'a HashMap<&'static str, &'static str>,
}
