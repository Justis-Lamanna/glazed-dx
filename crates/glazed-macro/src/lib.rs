extern crate proc_macro;

use proc_macro::TokenStream;
use std::env::var;
use quote::quote;
use syn;
use syn::{Data, Fields, Type, Variant};
use glazed_core::Id;

#[proc_macro_derive(Id, attributes(tuple))]
pub fn id_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_id_macro(&ast)
}

fn impl_id_macro(ast: &syn::DeriveInput) -> TokenStream {
    let tuple = ast.attrs.iter().filter(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "tuple")
        .any(|_| true);
    if tuple {
        return impl_id_macro_tuple(&ast);
    }
    let name = &ast.ident;
    match &ast.data {
        Data::Enum(data) => {
            let mut match_innards = quote!();
            for (idx, variant) in data.variants.iter().enumerate() {
                let idx = idx + 1; //0 should indicate a lack of value
                match_innards.extend(quote! {
                    #name::#variant => #idx,
                });
            }
            let gen = quote! {
                impl Id<usize> for #name {
                    fn get_id(&self) -> usize {
                        match self {
                            #match_innards
                        }
                    }
                }
            };
            gen.into()
        },
        _ => unimplemented!()
    }
}

fn is_variant_tupleable(variant: &Variant) -> bool {
    match &variant.fields {
        Fields::Unnamed(data) => true,
        Fields::Unit | Fields::Named(_) => false
    }
}

fn impl_id_macro_tuple(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    match &ast.data {
        Data::Enum(data) => {
            let mut match_innards = quote!();
            for (idx, variant) in data.variants.iter().enumerate() {
                let idx = idx + 1; //0 should indicate a lack of value
                let var_name = &variant.ident;
                if is_variant_tupleable(variant) {
                    match_innards.extend(quote! {
                        #name::#var_name(a) => (#idx, a.get_id()),
                    });
                } else {
                    match_innards.extend(quote! {
                        #name::#var_name => (#idx, 0),
                    });
                }
            }
            let gen = quote! {
                impl Id<(usize, usize)> for #name {
                    fn get_id(&self) -> (usize, usize) {
                        match self {
                            #match_innards
                        }
                    }
                }
            };
            gen.into()
        },
        _ => unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}