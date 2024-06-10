use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn process_enum_from(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
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
                      // #ty is with the T generic type, so we don't need to use generics
                      impl #generics From<#ty> for #name #generics {
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
}
