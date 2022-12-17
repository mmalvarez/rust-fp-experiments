use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, Block, FnArg, ItemFn, Pat, ReturnType, Type, Receiver};

#[proc_macro_attribute]
pub fn curry(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let parsed = parse_macro_input!(item as ItemFn); 
  generate_curry(parsed).into() 
}

use syn::punctuated::Punctuated;

fn extract_arg_pat(a : FnArg) -> Box<Pat> {
    match a {
        FnArg::Typed(p) => p.pat,
        _ => panic!("Invalid occurrence of `self`"), // TODO
    }
}

fn extract_arg_idents(fn_args :Punctuated<FnArg, syn::token::Comma>) -> (Option<Receiver>, Vec<Box<Pat>>) {
    let first_arg = fn_args[0].clone();

    let (recv, fn_args) = match first_arg {
        FnArg::Typed(p) => (None, fn_args.into_iter().skip(0)),
        FnArg::Receiver(r) => (Some(r), fn_args.into_iter().skip(1)),
    };
    (recv, fn_args.map(extract_arg_pat).collect::<Vec<_>>())
}

fn extract_arg_pat_idents(fn_args : Punctuated<FnArg, syn::token::Comma>) -> Vec<Box<Pat>> {
    return fn_args.into_iter().map(extract_arg_pat).collect::<Vec<_>>();
}

fn generate_body (fn_args : &[Box<Pat>], body : Box<Block>) -> proc_macro2::TokenStream {
    let mut acc = quote! {#body};
    for arg in fn_args.iter().rev() {
        acc = quote! {
            Rc::new(move |#arg| {#acc})
        }
    };
    return quote! { return #acc }
} 

fn extract_type(a : FnArg) -> Box<Type> {
    match a {
        FnArg::Typed(p) => p.ty,
        _ => panic!("Not supported on types with `self`."),
    }
}

fn extract_arg_types(fn_args: Punctuated<FnArg, syn::token::Comma>) -> Vec<Box<Type>> {
    return fn_args.into_iter().map(extract_type).collect::<Vec<_>>();
}

fn extract_return_type(a : ReturnType) -> Box<Type> {
    match a {
        ReturnType::Type(_, p) => p,
        _ => panic!("Not supported on functions without return types!"), //TODO change this?
    }
}

fn fix_type_ident (i : &syn::Ident) -> syn::Ident {
    let i_str = i.to_string();
    let i_str = i_str[0..1].to_uppercase() + &i_str[1..];
    syn::Ident::new(&i_str, i.span())
}

fn curry_fn_name (i : &syn::Ident) -> syn::Ident {
    let i_str = i.to_string();
    let i_str = format!("c_{}", i_str);
    syn::Ident::new(&i_str, i.span())
}

fn generate_types(
    fn_arg_types: &[Box<Type>],
    fn_return_type: Box<Type>,
    fn_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    
    let mut acc = quote! { #fn_return_type };

    for t in fn_arg_types.into_iter().rev() {
        acc = quote! {
            Rc<dyn Fn(#t) -> #acc>
        }
    };
    return acc
}


fn generate_type_aliases(
    fn_arg_types: &[Box<Type>],
    fn_return_type: Box<Type>,
    fn_name: &syn::Ident,
) -> Vec<proc_macro2::TokenStream> {
    
    let type_t0 = format_ident!("C{}XT0", fn_name);
    let mut type_aliases = vec![quote! { type #type_t0 = #fn_return_type }];

    for (i, t) in (1..).zip(fn_arg_types.into_iter().rev()) {
        let p = format_ident!("C{}X{}", fn_name, format!("T{}", i-1));
        let n = format_ident!("C{}X{}", fn_name, format!("T{}", i));

        type_aliases.push(quote! {
            type #n = Rc<dyn Fn(#t) -> #p>
        });
    }

    return type_aliases;
}

fn generate_curry(parsed: ItemFn) -> proc_macro2::TokenStream {
    let fn_body = parsed.block.clone();
    let sig = parsed.sig.clone();
    let vis = parsed.vis.clone();
    let fn_name_fixed = fix_type_ident(&sig.ident);
    let curry_fn_name = curry_fn_name(&sig.ident);
    let fn_args = sig.inputs;
    let fn_return_type = sig.output;

    let (recv, arg_idents) = extract_arg_idents(fn_args.clone());
    match recv {
        None => {            
            let first_ident = arg_idents.first().unwrap();

            let curried_body = generate_body(&arg_idents[1..], fn_body.clone());

            let arg_types = extract_arg_types(fn_args.clone());
            let first_type = &arg_types.first().unwrap();
            let type_aliases = generate_type_aliases(
                &arg_types[1..],
                extract_return_type(fn_return_type),
                &fn_name_fixed,
            );

            let return_type = format_ident!("C{}X{}", &fn_name_fixed, format!("T{}", type_aliases.len() - 1));

            let curry_result = quote! {
                #(#type_aliases);* ;
                #vis fn #curry_fn_name (#first_ident : #first_type) -> #return_type {
                    #curried_body ;
                }
            };

            let result = quote! {
                #parsed
                #curry_result
            };

            return result;
        },
        Some(recv) => {
            let curried_body = generate_body(&arg_idents, fn_body.clone());

            let arg_types = extract_arg_types(fn_args.clone());
            let first_type = &arg_types.first().unwrap();
            let type_aliases = generate_type_aliases(
                &arg_types,
                extract_return_type(fn_return_type),
                &fn_name_fixed,
            );

            let return_type = format_ident!("C{}X{}", &fn_name_fixed, format!("T{}", type_aliases.len() - 1));

            let curry_result = quote! {
                #(#type_aliases);* ;
                #vis fn #curry_fn_name #recv -> #return_type {
                    #curried_body ;
                }
            };

            let result = quote! {
                #parsed
                #curry_result
            };

            return result;

        },
    }
}
