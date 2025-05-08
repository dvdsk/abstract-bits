use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};
use syn::spanned::Spanned;
use syn::Ident;

use crate::model::NormalField;

pub fn is_some_ident(controlled: &Ident) -> Ident {
    format_ident!("{controlled}_is_some")
}

pub fn read(field: &NormalField) -> TokenStream {
    let is_some_ident = is_some_ident(&field.ident);
    let field_ident = &field.ident;
    let field_read_code = super::normal::read(field);
    quote_spanned! {field.ident.span()=>
        let #field_ident = if #is_some_ident {
            #field_read_code
            Some(#field_ident)
        } else {
            None
        };
    }
}

pub fn write(field: &NormalField) -> TokenStream {
    let field_ident = &field.ident;
    let write_code = if let Some(bits) = field.bits {
        let utype: syn::Type = syn::parse_str(&format!("::abstract_bits::u{bits}"))
            .expect("should be valid type path");
        quote_spanned! {field.out_ty.span()=>
            let #field_ident = #utype::new(#field_ident);
            #field_ident.write_abstract_bits(writer)?;
        }
    } else {
        quote_spanned! {field.out_ty.span()=>
            #field_ident.write_abstract_bits(writer)?;
        }
    };

    quote_spanned!(field_ident.span()=>
        if let Some(ref #field_ident) = self.#field_ident {
            #write_code
        }
    )
}

pub(crate) fn needed_bits(inner_type: &NormalField) -> TokenStream {
    let ty = &inner_type.out_ty;
    quote_spanned! {inner_type.ident.span()=>
        let range = #ty::needed_bits();
        min += range.start();
        max += range.end();
    }
}
