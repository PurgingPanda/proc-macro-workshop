use proc_macro::TokenStream;
use syn::{self, parse_macro_input};
use syn::DeriveInput;
use quote::{quote, ToTokens};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _ = input;
    let t = parse_macro_input!(input as DeriveInput);

    eprintln!("{:#?}", t);

    quote!{}.into()
}
