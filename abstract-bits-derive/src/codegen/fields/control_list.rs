use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::Ident;

use crate::codegen::{is_primitive, list_len_ident};

pub fn read(controlled: &Ident, bits: usize) -> TokenStream {
    let len_ident = list_len_ident(controlled);
    if let Some(ty) = is_primitive(bits) {
        quote_spanned! {controlled.span()=>
            let #len_ident = #ty::read_abstract_bits(reader)?;
        }
    } else {
        let utype: syn::Type =
            syn::parse_str(&format!("::abstract_bits::u{bits}")).expect("valid type path");
        quote_spanned! {controlled.span()=>
            let #len_ident = #utype::read_abstract_bits(reader)?;
            let #len_ident = #len_ident.value();
        }
    }
}

pub fn write(controlled: &Ident, bits: usize) -> TokenStream {
    let len_ident = list_len_ident(controlled);
    if let Some(ty) = is_primitive(bits) {
        quote_spanned! {controlled.span()=>
            let #len_ident: #ty = self.#controlled.len().try_into()
                .map_err(|_| ::abstract_bits::ToBytesError::ListTooLong {
                    max: #ty::MAX as usize,
                    got: self.#controlled.len(),
            })?;
            ::abstract_bits::AbstractBits::write_abstract_bits(&#len_ident, writer)?;
        }
    } else {
        let utype: syn::Type =
            syn::parse_str(&format!("::abstract_bits::u{bits}")).expect("valid type path");
        quote_spanned! {controlled.span()=>
            let #len_ident = #utype::new(self.#controlled.len().try_into()
                .map_err(|_| ::abstract_bits::ToBytesError::ListTooLong {
                    max: 2usize.pow(#utype::BITS as u32) - 1,
                    got: self.#controlled.len(),
                })?);
            ::abstract_bits::AbstractBits::write_abstract_bits(&#len_ident, writer)?;
        }
    }
}

pub(crate) fn needed_bits(n_bits: usize) -> TokenStream {
    let n_bits = proc_macro2::Literal::usize_unsuffixed(n_bits);
    quote! {
        min += #n_bits;
        max += #n_bits;
    }
}
