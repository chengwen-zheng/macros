use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(deref))]
struct AutoDerefInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), AutoDerefFieldsInfo>,
    #[darling(default)]
    mutable: bool,
    #[darling(default)]
    field: Option<syn::Ident>,
}

#[derive(Debug, FromField)]
struct AutoDerefFieldsInfo {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

pub fn process_auto_deref(input: DeriveInput) -> TokenStream {
    let AutoDerefInfo {
        ident,
        generics,
        data: Data::Struct(fields),
        mutable,
        field,
    } = AutoDerefInfo::from_derive_input(&input).unwrap()
    else {
        panic!("Failed to parse AutoDerefInfo")
    };

    let (fd, ty) = if let Some(field) = field {
        match fields.iter().find(|f| f.ident.as_ref() == Some(&field)) {
            Some(fd) => (fd.ident.as_ref().unwrap(), fd.ty.clone()),
            None => panic!("Field {} not found", field),
        }
    } else {
        // if only 1 field, use it
        if fields.len() == 1 {
            let fd = fields.iter().next().unwrap();
            (fd.ident.as_ref().unwrap(), fd.ty.clone())
        } else {
            panic!("Field not specified and struct has more than 1 field");
        }
    };

    let mut code = vec![quote! {
      impl #generics std::ops::Deref for #ident #generics {
        type Target = #ty;

        fn deref(&self) -> &Self::Target {
          &self.#fd
        }
      }
    }];

    if mutable {
        code.push(quote! {
            impl #generics std::ops::DerefMut for #ident #generics {
              fn deref_mut(&mut self) -> &mut Self::Target {
                  &mut self.#fd
              }
          }
        })
    }

    quote! {
      #(#code)*
    }
}
