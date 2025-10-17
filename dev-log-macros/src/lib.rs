use proc_macro::TokenStream;
use syn::{parse_macro_input, Item};

mod dtrace;

#[proc_macro_attribute]
pub fn dtrace(args: TokenStream, input: TokenStream) -> TokenStream {
    if !cfg!(debug_assertions) {
        return input
    }
    let item = parse_macro_input!(input as Item);
    parse_macro_input!(args as crate::dtrace::DTraceAttribute).decorate(item)
}