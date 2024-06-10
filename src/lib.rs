// proc macro create
mod auto_debug;
mod auto_deref;
mod enum_from;
mod enum_from_darling;
use auto_debug::process_auto_debug;
use auto_deref::process_auto_deref;
use enum_from::process_enum_from;
use enum_from_darling::process_enum_from_darling;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

// for enum, we'd like to generate From impls for each variant
#[proc_macro_derive(EnumFrom)]
pub fn enum_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    println!("{:#?}", input);
    process_enum_from(input).into()
}

#[proc_macro_derive(EnumFromDarling)]
pub fn enum_from_darling(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    println!("{:#?}", input);
    process_enum_from_darling(input).into()
}

#[proc_macro_derive(AutoDeref, attributes(deref))]
pub fn derive_auto_deref(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    process_auto_deref(input).into()
}

#[proc_macro_derive(AutoDebug, attributes(debug))]
pub fn derive_auto_debug(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    process_auto_debug(input).into()
}
