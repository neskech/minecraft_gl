#![allow(non_snake_case)] //I need to stop changing my naming convenctions ):

extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::quote;
use syn;

#[proc_macro_derive(ComponentObj)]
pub fn CompObjDerive(input: TokenStream) -> TokenStream{
    let ast = syn::parse(input).unwrap();
    ImplCompObj(&ast)
}

fn ImplCompObj(ast: &syn::DeriveInput) -> TokenStream{
    let name = &ast.ident;
    let result = quote! {
        impl CompObj for #name {
            fn AsAny(&self) -> &dyn std::any::Any {
                self
            }
            fn AsAnyMut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
    result.into()
}

// #[proc_macro_attribute]
// pub fn Bind(_meta: TokenStream, code: TokenStream) -> TokenStream {
//      let input = parse_macro_input!(code as ItemImpl);
//      let result = quote! {
//         #input

//      };
//     let ast = Impl(&input);

//      let a = parse_macro_input!(ast as ItemImpl);
//      let b = parse_macro_input!(result as ItemImpl);
//     //let t = parse_macro_input!(a as ItemImpl);
//     let res = quote! {
//         #b
//     };
//      rres.into()
//     //TokenStream::from(quote!(#input))
    
//     // ...

//     //TokenStream::from(quote!(#input))
// }

// #[proc_macro_attribute]
// pub fn Bind(input: TokenStream, item: TokenStream) -> TokenStream{
//     let ast = syn::parse(item).unwrap();
//     Impl(&ast)
// }

// fn Impl(ast: &syn::ItemImpl) -> TokenStream{
//     let mut funcs: Vec<&syn::Ident> = Vec::new();
//     for item in &ast.items {
//         if let syn::ImplItem::Method(syn::ImplItemMethod { sig: Signature { ident, .. }, .. }) = item {
//             //funcs.push(syn::Ident::new(ident.to_string().as_str(), syn::Span::call_site()));
//             funcs.push(ident);
//         }
//     }

//     let name = &ast.self_ty;
//     let result = quote! {
//         pub fn Bind(nameToID: &HashMap<&str, u8>, registry: &mut BlockRegistry, binding: & #name) {
//             #(
//                  #name :: #funcs (nameToID, registry)
//             );*
      
//         }
//     };
//     result.into()
// }

