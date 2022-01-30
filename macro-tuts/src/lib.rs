mod builder;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() { println!(\"hello world\"); }".parse().unwrap()
}

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    println!("{:#?}", input);

    builder::BuilderContext::from(input).render().into()
}
