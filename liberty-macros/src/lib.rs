use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};



#[proc_macro_derive(Group)]
pub fn group_macro(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let gen = quote! {
        impl liberty_parser::LibertyAttri for #name {
            const T: liberty_parser::AttriType = liberty_parser::AttriType::SimpleSingle;
            fn parser<'a>(i: &'a str) -> nom::IResult<&'a str, Self, nom::error::Error<&str>> 
            where 
                Self: Sized {
                nom::combinator::map_res(
                    liberty_parser::SimpleAttri::single,
                    |s: &str| match s.parse::<Self>(){
                        Ok(_self) => Ok(_self),
                        Err(_) => Err(
                            nom::error::Error::new(i, nom::error::ErrorKind::Fail)),
                    },
                )(i)
            }
        }
    };
    gen.into()
}



#[proc_macro_derive(SingleSimple)]
pub fn single_simple_macro(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let gen = quote! {
        impl liberty_parser::LibertyAttri for #name {
            const T: liberty_parser::AttriType = liberty_parser::AttriType::SimpleSingle;
            fn parser<'a>(i: &'a str) -> nom::IResult<&'a str, Self, nom::error::Error<&str>> 
            where 
                Self: Sized {
                nom::combinator::map_res(
                    liberty_parser::SimpleAttri::single,
                    |s: &str| match s.parse::<Self>(){
                        Ok(_self) => Ok(_self),
                        Err(_) => Err(
                            nom::error::Error::new(i, nom::error::ErrorKind::Fail)),
                    },
                )(i)
            }
        }
    };
    gen.into()
}