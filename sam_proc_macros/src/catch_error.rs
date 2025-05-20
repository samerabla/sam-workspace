use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Procedural macro to log errors on functions returning `Result<T, E>`
#[proc_macro_attribute]
pub fn catch_error(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the function
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident; // Function name
    let fn_body = &input.block; // Function body
    let fn_sig = &input.sig; // Function signature

    // Generate the new function body
    let expanded = quote! {
        #fn_sig {
            let result = (|| #fn_body)();
            match result {
                Ok(val) => Ok(val),
                Err(err) => {
                    tracing::error!("Error in {}: {}", stringify!(#fn_name), err);
                    Err(err.into())
                }
            }
        }
    };

    TokenStream::from(expanded)
}
