extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn timer(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let block = &input.block;

    let mut new_stmts = Vec::new();
    new_stmts.push(syn::parse_quote! {
        let mut timer = timer::Timer::new();
    });

    // Populate the function
    for stmt in block.stmts.iter() {
        new_stmts.push(stmt.clone());
    }

    let new_block = syn::Block {
        brace_token: block.brace_token,
        stmts: new_stmts,
    };

    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis;
    let fn_args = &input.sig.inputs;
    let fn_out = &input.sig.output;
    let fn_where = &input.sig.generics.where_clause;

    let expanded = quote! {
        #fn_vis fn #fn_name(#fn_args) #fn_out #fn_where #new_block
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn fn_timer(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let block = &input.block;
    let fn_name_str = &input.sig.ident.to_string();
    let fn_out = &input.sig.output;

    let has_return = match fn_out {
        ReturnType::Default => false, // () is the default return type
        ReturnType::Type(_, ty) => {
            // Check if the type is explicitly ()
            quote!(#ty).to_string() != "()"
        }
    };

    let mut new_stmts = Vec::new();
    new_stmts.push(syn::parse_quote! {
        let mut timer = timer::Timer::new();
    });

    new_stmts.push(syn::parse_quote! {
        timer.start(#fn_name_str.to_string());
    });

    // Populate the function
    for stmt in block.stmts.iter() {
        new_stmts.push(stmt.clone());
    }

    // If function returns something put timer.finish before it
    if let Some(return_stmt) = new_stmts.pop() {
        if has_return {
            new_stmts.push(syn::parse_quote! {
                timer.finish(#fn_name_str.to_string(), module_path!());
            });
            new_stmts.push(return_stmt);
        } else {
            new_stmts.push(return_stmt);
            new_stmts.push(syn::parse_quote! {
                timer.finish(#fn_name_str.to_string(), module_path!());
            });
        }
    }

    let new_block = syn::Block {
        brace_token: block.brace_token,
        stmts: new_stmts,
    };

    let fn_name = &input.sig.ident;
    let fn_args = &input.sig.inputs;
    let fn_vis = &input.vis;
    let fn_where = &input.sig.generics.where_clause;

    let expanded = quote! {
        #fn_vis fn #fn_name(#fn_args) #fn_out #fn_where #new_block
    };

    TokenStream::from(expanded)
}
