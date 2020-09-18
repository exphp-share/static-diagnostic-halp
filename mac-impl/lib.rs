use proc_macro::{TokenStream as TokenStream1};
use proc_macro2::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn attr(_args: TokenStream1, input: TokenStream1) -> TokenStream1 {
    let input: TokenStream = input.into();
    quote!(
        #input
        ::mac::check_smn!{}
    ).into()
}
