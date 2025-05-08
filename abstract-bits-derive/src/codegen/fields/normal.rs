use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;

use crate::codegen::generics_to_fully_qualified;
use crate::model::NormalField;

pub fn read(
    NormalField {
        ident,
        out_ty,
        bits,
        ..
    }: &NormalField,
) -> TokenStream {
    if let Some(bits) = bits {
        let utype: syn::Type = syn::parse_str(&format!("::abstract_bits::u{bits}"))
            .expect("should be valid type path");
        quote_spanned! {out_ty.span()=>
            let #ident = #utype::read_abstract_bits(reader)?;
            let #ident = #ident.value();
        }
    } else {
        let out_ty = generics_to_fully_qualified(out_ty.clone());
        quote_spanned! {out_ty.span()=>
            let #ident = #out_ty::read_abstract_bits(reader)?;
        }
    }
}

pub fn write(
    NormalField {
        ident,
        out_ty,
        bits,
        ..
    }: &NormalField,
) -> TokenStream {
    if let Some(bits) = *bits {
        let utype: syn::Type = syn::parse_str(&format!("::abstract_bits::u{bits}"))
            .expect("should be valid type path");
        quote_spanned! {out_ty.span()=>
            let #ident = #utype::new(self.#ident);
            #ident.write_abstract_bits(writer)?;
        }
    } else {
        quote_spanned! {out_ty.span()=>
            self.#ident.write_abstract_bits(writer)?;
        }
    }
}

pub(crate) fn needed_bits(normal_field: &crate::model::NormalField) -> TokenStream {
    let ty = &normal_field.out_ty;
    quote_spanned! {normal_field.ident.span()=>
        let range = #ty::needed_bits();
        min += range.start();
        max += range.end();
    }
}
