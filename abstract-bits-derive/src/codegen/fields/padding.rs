use proc_macro2::{Literal, TokenStream};
use quote::quote;

pub fn read(n_bits: u8, struct_name: &Literal) -> TokenStream {
    let n_bits = proc_macro2::Literal::usize_suffixed(n_bits as usize);
    quote! {
        reader.skip(#n_bits)
            .map_err(|cause| ::abstract_bits::ReadErrorCause::NotEnoughInput {
                ty: "-",
                cause
            }).map_err(|cause| ::abstract_bits::FromBytesError::SkipPadding {
                struct_name: #struct_name,
                cause,
            })?;
    }
}

pub fn write(n_bits: u8, struct_name: &Literal) -> TokenStream {
    let n_bits = proc_macro2::Literal::usize_suffixed(n_bits as usize);
    quote! {
        writer.skip(#n_bits)
            .map_err(|cause| ::abstract_bits::ToBytesError::AddPadding {
                cause,
                struct_name: #struct_name,
            })?;
    }
}

pub(crate) fn min_bits(n_bits: u8) -> TokenStream {
    let n_bits = proc_macro2::Literal::usize_unsuffixed(n_bits as usize);
    quote! {
        #n_bits
    }
}

pub(crate) fn max_bits(n_bits: u8) -> TokenStream {
    let n_bits = proc_macro2::Literal::usize_unsuffixed(n_bits as usize);
    quote! {
        #n_bits
    }
}
