// https://blog.jetbrains.com/rust/2022/03/18/procedural-macros-under-the-hood-part-i/
// https://dev.to/dandyvica/rust-procedural-macros-step-by-step-tutorial-36n8
// https://dev.to/jeikabu/rust-derive-macros-o38
// https://users.rust-lang.org/t/how-to-parse-the-value-of-a-macros-helper-attribute/39882
// https://stackoverflow.com/a/64096369/839733
// https://sodocumentation.net/rust/topic/9104/custom-derive---macros-1-1-
// https://blog.logrocket.com/procedural-macros-in-rust/
// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{Meta, MetaNameValue, Lit};

#[proc_macro_derive(Planet, attributes(years))]
pub fn planet_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    if let Some(years) = parse_years_attr(&ast) {
        impl_planet(&ast, years)
    } else {
        panic!("Missing attribute 'years' on Struct '{}', example for Earth: #[years = 1.0]", ast.ident.to_string());
    }
}

fn parse_years_attr(ast: &syn::DeriveInput) -> Option<f64> {
    ast.attrs.iter()
        .filter_map(|attr| {
            match attr.parse_meta().unwrap() {
                // Match '#[ident = lit]' attributes. Match guard makes it '#[years = lit]'
                Meta::NameValue(MetaNameValue{ref path, ref lit, ..}) if path.is_ident("years") => {
                    if let Lit::Float(lit) = lit {
                        lit.base10_parse::<f64>().ok()
                    } else {
                        None
                    }
                },
                _ => None
            }
        })
        .next()
}

const EARTH_YEAR_SECONDS: f64 = 31557600.0;

fn impl_planet(ast: &syn::DeriveInput, years: f64) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Planet for #name {
            fn years_during(d: &Duration) -> f64 {
                (d.0 as f64) / (#EARTH_YEAR_SECONDS * #years)
            }
        }
    };
    gen.into()
}
