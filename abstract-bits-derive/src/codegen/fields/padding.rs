use proc_macro2::TokenStream;
use quote::quote;

pub fn read(n_bits: u8, struct_name: &syn::Ident) -> TokenStream {
    let n_bits = proc_macro2::Literal::usize_suffixed(n_bits as usize);
    let name = proc_macro2::Literal::string(&struct_name.to_string());
    quote! {
        reader.skip(#n_bits)
            .map_err(|cause| ::abstract_bits::FromBytesError::SkipPadding {
                cause,
                in_struct: #name,
            })?;
    }
}

pub fn write(n_bits: u8, struct_name: &syn::Ident) -> TokenStream {
    let n_bits = proc_macro2::Literal::usize_suffixed(n_bits as usize);
    let name = proc_macro2::Literal::string(&struct_name.to_string());
    quote! {
        writer.skip(#n_bits)
            .map_err(|cause| ::abstract_bits::ToBytesError::AddPadding {
                cause,
                in_struct: #name,
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
