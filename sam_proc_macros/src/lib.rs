use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ItemFn};

#[proc_macro_attribute]
pub fn catch_error(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract the function signature and body
    let fn_vis = &input_fn.vis; // e.g., `pub`
    let fn_sig = &input_fn.sig; // e.g., `async fn add_user_handler(...) -> Result<Response>`
    let fn_name = &input_fn.sig.ident; // Function name
    let fn_block = &input_fn.block; // The function body

    // Check if the function is async
    let is_async = fn_sig.asyncness.is_some();

    let match_block = quote! {
        Ok(res) => Ok(res),
        Err(err) => {
            tracing::error!("\nModule: {}\nFile: {}\nLine: {}\nFunction: {}\n🚨Error Message: \n-------------\n{}\n-------------", module_path!(),file!(),line!(),stringify!(#fn_name),err);
            return Err(err);
        }
    };
    // Generate the new function body
    let new_block = if is_async {
        quote! {
            {
                let result = async #fn_block;
                match result.await {
                    #match_block
                }
            }
        }
    } else {
        quote! {
            {
                let result = #fn_block;
                match result {
                    #match_block
                }
            }
        }
    };

    // Generate the new function
    let output = quote! {
        #fn_vis #fn_sig {
            #new_block
        }
    };

    // Return the generated code as a TokenStream
    TokenStream::from(output)
}

// #[proc_macro_attribute]
// pub fn catch_error(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     // Parse the input function
//     let input_fn = parse_macro_input!(item as ItemFn);

//     // Extract the function signature and body
//     let fn_vis = &input_fn.vis; // e.g., `pub`
//     let fn_sig = &input_fn.sig; // e.g., `async fn add_user_handler(...) -> Result<Response>`
//     let fn_name = &input_fn.sig.ident; // Function name
//     let fn_block = &input_fn.block; // The function body

//     // Generate the new function body
//     let new_block = quote! {
//         {
//             let result = async #fn_block;
//             match result.await {
//                 Ok(res) => Ok(res),
//                 Err(err) => {
//                     tracing::error!("\nModule: {}\nFile: {}\nLine: {}\nFunction: {}\nError Message: \n-------------\n{}\n-------------", module_path!(),file!(),line!(),stringify!(#fn_name),err);
//                     return Err(err);
//                 }
//             }
//         }
//     };

//     // Generate the new function
//     let output = quote! {
//         #fn_vis #fn_sig {
//             #new_block
//         }
//     };

//     // Return the generated code as a TokenStream
//     TokenStream::from(output)
// }
