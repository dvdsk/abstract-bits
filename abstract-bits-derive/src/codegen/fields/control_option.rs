use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::Ident;

pub fn read(controlled: &Ident) -> TokenStream {
    let controller_ident = super::option::is_some_ident(controlled);
    quote_spanned! {controlled.span()=>
        let #controller_ident = bool::read_abstract_bits(reader)?;
    }
}

pub fn write(controlled: &Ident) -> TokenStream {
    quote_spanned! {controlled.span()=>
        if self.#controlled.is_some() {
            true.write_abstract_bits(writer)?;
        } else {
            false.write_abstract_bits(writer)?;
        }
    }
}

pub(crate) fn min_bits() -> TokenStream {
    quote! { 1 }
}

pub(crate) fn max_bits() -> TokenStream {
    quote! { 1 }
}
