#![feature(proc_macro_span)]
extern crate proc_macro;

use proc_macro::{Span, TokenStream};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use quote::{quote, TokenStreamExt, ToTokens};
use quote::__private::{Literal, Punct, Spacing};
use syn::{Attribute, DeriveInput, Error, Ident, Lit, LitStr, Meta, MetaNameValue, parse_macro_input, Path, Result};
use syn::__private::TokenStream2;
use syn::group::Group;
use syn::Lit::Str;
use yaml_rust::{ScanError, Yaml, YamlLoader};
use yaml_rust::yaml::Hash;

fn get_source_ident(attrs: Vec<Attribute>) -> Result<Option<Path>> {
    if attrs.len() > 0 {
        let attr = attrs.get(0).unwrap();
        if !attr.path.is_ident("source") {
            return Ok(None)
        }

        let tokens = attr.parse_meta().unwrap();
        match tokens {
            Meta::NameValue(MetaNameValue{lit: Str(lit), ..}) => {
                lit.parse().map(Some)
            },
            _ => Err(Error::new_spanned(attr, "expected #[source = \"...\"]"))
        }
    } else {
        panic!("idk");
    }
}

fn read_file_data(source: &Path, data: &Ident) -> Result<TokenStream2> {
    let filename = {
        let f = format!("{}-{}",
                source.get_ident().unwrap().to_string().to_ascii_lowercase(),
                data.to_string().to_ascii_lowercase());
        let source = Span::call_site().source_file().path();
        let resources = source.parent()
            .and_then(|e| e.parent())
            .ok_or(Error::new_spanned(data, ""))?;
        let mut resources = PathBuf::from(resources);
        resources.push("resources");
        resources.push(f);
        resources.set_extension("yml");
        resources
    };

    let mut f = match File::open(filename) {
        Ok(f) => f,
        Err(e) => return Err(Error::new_spanned(data, e.to_string()))
    };

    let mut raw = String::new();
    let raw = match f.read_to_string(&mut raw) {
        Ok(_) => raw,
        Err(e) => return Err(Error::new_spanned(data, e.to_string()))
    };

    let file = match YamlLoader::load_from_str(raw.as_str()) {
        Ok(a) => a,
        Err(e) => return Err(Error::new_spanned(data, e.to_string()))
    };

    Ok(parse_yaml(data, source.get_ident().unwrap(), file.get(0).unwrap()))
}

fn parse_yaml (data: &Ident, e: &Ident, yml: &Yaml) -> TokenStream2  {
    let mut tokens = Vec::new();
    if let Yaml::Array(a) = yml {
        for element in a {
            if let Yaml::Hash(h) = element {
                tokens.push(parse_yaml_2(data, e, h));
            } else {
                panic!("Unexpected token {:?}", element);
            }
        }
    }

    quote! {
        #(#tokens),*
    }
}

fn parse_yaml_2 (data: &Ident, e: &Ident, hash: &Hash) -> TokenStream2 {
    let mut id = None;
    let mut body = Vec::new();

    for (k, v) in hash {
        if let Yaml::String(k) = k {
            if *k == "id" {
                if let Yaml::String(v) = v {
                    id = if *v == "_" {
                        Some(TokenWrapper::Underscore)
                    } else {
                        Some(TokenWrapper::YamlLiteral(v.clone()))
                    };
                } else {
                    panic!("ID type must be string");
                }
            } else {
                let k = TokenWrapper::YamlLiteral(k.clone());
                let v = TokenWrapper::convert(v);
                body.push(quote! {
                    #k: #v
                })
            }
        } else {
            panic!("Unknown key type: {:?}", k);
        }
    }

    if let Some(TokenWrapper::Underscore) = id {
        quote! {
            _ => #data {
                #(#body),*
            }
        }
    } else {
        quote! {
            #e::#id => #data {
                #(#body),*
            }
        }
    }
}

enum TokenWrapper {
    Underscore,
    YamlLiteral(String),
    YamlString(String),
    YamlNumber(i64),
    YamlFraction(f64),
    YamlBoolean(bool)
}
impl TokenWrapper {
    pub fn convert(yaml: &Yaml) -> TokenWrapper {
        match yaml {
            Yaml::Real(a) => TokenWrapper::YamlFraction(a.parse().unwrap()),
            Yaml::Integer(a) => TokenWrapper::YamlNumber(*a),
            Yaml::String(a) => TokenWrapper::YamlString(a.clone()),
            Yaml::Boolean(a) => TokenWrapper::YamlBoolean(*a),
            Yaml::Null => panic!("Bad value"),
            Yaml::Array(a) => panic!("Bad value"),
            Yaml::Hash(_) => panic!("Bad value"),
            Yaml::Alias(_) => panic!("Bad value"),
            Yaml::BadValue => panic!("Bad value")
        }
    }
}
impl ToTokens for TokenWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        use quote::__private::TokenTree;
        use quote::__private::Span;
        match self {
            TokenWrapper::Underscore => tokens.append(TokenTree::Ident(Ident::new("_", Span::call_site()))),
            TokenWrapper::YamlLiteral(l) => tokens.append(TokenTree::Ident(Ident::new(l.as_str(), Span::call_site()))),
            TokenWrapper::YamlString(l) => tokens.append(TokenTree::Literal(Literal::string(l.as_str()))),
            TokenWrapper::YamlNumber(l) => tokens.append(TokenTree::Literal(Literal::i64_unsuffixed(*l))),
            TokenWrapper::YamlFraction(l) => tokens.append(TokenTree::Literal(Literal::f64_unsuffixed(*l))),
            TokenWrapper::YamlBoolean(l) => {
                let text = if *l { "true" } else { "false" };
                tokens.append(TokenTree::Ident(Ident::new(text, Span::call_site())));
            },
        }
    }
}

#[proc_macro_derive(EnumData, attributes(source))]
pub fn enum_data(input: TokenStream) -> TokenStream {
    let i = parse_macro_input!(input as DeriveInput);
    dbg!("hi");
    let source_ident = get_source_ident(i.attrs).unwrap().unwrap();
    let data_ident = i.ident;

    let data = read_file_data(&source_ident, &data_ident).unwrap();

    let test = quote! {
        impl From<#source_ident> for #data_ident {
            fn from(i: #source_ident) -> Self {
                match i {
                    #data
                }
            }
        }
    };
    // dbg!(test.to_string());
    TokenStream::from(test)
}