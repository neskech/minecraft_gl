#![allow(non_snake_case)] //I need to stop changing my naming convenctions ):

extern crate proc_macro;

use proc_macro::TokenStream;
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