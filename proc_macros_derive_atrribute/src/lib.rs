use builder::BuilderContext;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod builder;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn generate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let context = BuilderContext::new(input);
    context.generate().into()
}
