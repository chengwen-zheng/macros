// proc macro create

use proc_macro::TokenStream;
use quote::quote;

// for enum, we'd like to generate From impls for each variant
#[proc_macro_derive(EnumFrom)]
pub fn enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    println!("{:#?}", input);

    let name = &input.ident;
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom can only be derived for enums"),
    };

    // for each variant, generate an impl that converts the enum to the variant
    let from_impls = variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // only support single field variants
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("should have 1 field");
                    let ty = &field.ty;
                    quote! {
                      impl From<#ty> for #name {
                        fn from(v: #ty) -> Self {
                            #name::#variant_name(v)
                        }
                      }
                    }
                }
            }
            syn::Fields::Unit => quote! {},
            syn::Fields::Named(_) => quote! {},
        }
    });

    quote! {
        #(#from_impls)*
    }
    .into()
}
