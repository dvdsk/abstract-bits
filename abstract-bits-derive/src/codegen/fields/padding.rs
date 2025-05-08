use proc_macro2::TokenStream;
use quote::quote;

pub fn read(n_bits: u8) -> TokenStream {
    let n_bits = proc_macro2::Literal::usize_suffixed(n_bits as usize);
    quote! { reader.skip(#n_bits).map_err(::abstract_bits::FromBytesError::SkipPadding)?; }
}

pub fn write(n_bits: u8) -> TokenStream {
    let n_bits = proc_macro2::Literal::usize_suffixed(n_bits as usize);
    quote! { writer.skip(#n_bits).map_err(::abstract_bits::ToBytesError::AddPadding)?; }
}

pub(crate) fn needed_bits(n_bits: u8) -> TokenStream {
    let n_bits = proc_macro2::Literal::usize_unsuffixed(n_bits as usize);
    quote! {
        min += #n_bits;
        max += #n_bits;
    }
}
