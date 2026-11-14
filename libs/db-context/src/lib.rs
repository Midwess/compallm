use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use quote::{format_ident, quote};
use syn::{ItemFn, parse_quote};

/// Run an async function in a new root context, restoring its caller's scope afterward.
/// The function must return a Result whose error accepts ConnectionError.
#[proc_macro_attribute]
pub fn root(arguments: TokenStream, function: TokenStream) -> TokenStream {
    scoped(arguments, function, "root")
}

/// Run an async function in its caller's current context, preserving its transaction.
/// The function must return a Result whose error accepts ConnectionError.
#[proc_macro_attribute]
pub fn current(arguments: TokenStream, function: TokenStream) -> TokenStream {
    scoped(arguments, function, "current")
}

fn scoped(arguments: TokenStream, function: TokenStream, mode: &str) -> TokenStream {
    expand(arguments, function, mode)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand(
    arguments: TokenStream,
    function: TokenStream,
    mode: &str,
) -> syn::Result<proc_macro2::TokenStream> {
    if !arguments.is_empty() {
        return Err(syn::Error::new_spanned(
            proc_macro2::TokenStream::from(arguments),
            "database context attributes take no arguments",
        ));
    }
    let mut function: ItemFn = syn::parse(function)?;
    if function.sig.asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            &function.sig,
            "database context attributes require an async function",
        ));
    }
    let runtime = match crate_name("compallm-core") {
        Ok(FoundCrate::Itself) => quote!(crate),
        Ok(FoundCrate::Name(name)) => {
            let name = format_ident!("{name}");
            quote!(::#name)
        }
        Err(error) => return Err(syn::Error::new_spanned(&function.sig, error)),
    };
    let mode = format_ident!("{mode}");
    let body = function.block;
    function.block = parse_quote!({
        #runtime::persistent::DbContext::#mode()?.scope(async #body).await
    });
    Ok(quote!(#function))
}
