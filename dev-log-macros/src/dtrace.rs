use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, Expr, Item, Lit, Meta};

pub(crate) struct DTraceAttribute  {
    pub(crate) name: Option<String>,
}

impl DTraceAttribute {
    pub(crate) fn decorate(self, item: Item) -> TokenStream {
        match item {
            Item::Fn(f) => {
                let name = self.name.unwrap_or_else(|| f.sig.ident.to_string());
                let vis = &f.vis;
                let sig = &f.sig;
                let attrs = &f.attrs;
                let body = &f.block;

                quote! {
                    #(#attrs)*
                    #vis #sig {
                        #[cfg(feature="stack-trace")]
                        let __stack_guard: ::dev_log::stack_trace::StackTrace = #name.into();
                        #body
                    }
                }.into()
            },
            _ => panic!("#[dtrace] only avaliable for fn"),
        }
    }
}

impl Parse for DTraceAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(DTraceAttribute { name: None });
        }
        let meta: Meta = input.parse()?;
        match meta {
            Meta::NameValue(nv) if nv.path.is_ident("name") => match nv.value {
                Expr::Lit(lit) => match lit.lit {
                    Lit::Str(name) => Ok(DTraceAttribute { name: Some(name.value()) }),
                    _ => Err(syn::Error::new_spanned(lit.lit, "name must be a string literal")),
                },
                _ => Err(syn::Error::new_spanned(nv.value, "name must be a string literal")),
            },
            _ => Err(syn::Error::new_spanned(meta, "unkown attribute")),
        }
    }
}
