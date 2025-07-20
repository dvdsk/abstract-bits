use proc_macro2::{Literal, TokenStream};
use quote::quote_spanned;
use syn::Ident;

use crate::model::NormalField;

pub(crate) fn write(inner_type: &NormalField, controller: &Option<Ident>) -> TokenStream {
    let field_ident = &inner_type.ident;
    
    match controller {
        Some(controller_ident) => {
            // Validate that controller field matches the vector length
            quote_spanned! {field_ident.span()=>
                // Validation: ensure controller field matches vector length
                if self.#controller_ident as usize != self.#field_ident.len() {
                    return Err(::abstract_bits::ToBytesError::ValidationError(
                        format!("Field '{}' length controller is {} but vector has {} elements", 
                            stringify!(#field_ident), 
                            self.#controller_ident, 
                            self.#field_ident.len())
                    ));
                }
                
                // Write all elements
                for element in &self.#field_ident {
                    ::abstract_bits::AbstractBits::write_abstract_bits(element, writer)?;
                }
            }
        }
        None => {
            panic!("List field without controller")
        }
    }
}

pub(crate) fn read(field: &NormalField, controller: &Option<Ident>, struct_name: &Literal) -> TokenStream {
    let field_name = Literal::string(&field.ident.to_string());
    let field_ident = &field.ident;
    
    match controller {
        Some(controller_ident) => {
            quote_spanned! {field.ident.span()=>
                // Use the controller field that was already read
                let res = (0..#controller_ident).into_iter().map(|_|
                    ::abstract_bits::AbstractBits::read_abstract_bits(reader)
                )
                    .collect::<Result<_, ::abstract_bits::FromBytesError>>()
                    .map_err(|cause| cause.read_list(#struct_name, 
                            #field_name, #controller_ident as usize));
                let #field_ident = res?;
            }
        }
        None => {
            panic!("List field without controller")
        }
    }
}

pub(crate) fn min_bits(inner_type: &NormalField) -> TokenStream {
    quote_spanned! {inner_type.ident.span()=>
        0
    }
}

pub(crate) fn max_bits(inner_type: &NormalField, max_len: usize) -> TokenStream {
    let ty = &inner_type.out_ty;
    quote_spanned! {inner_type.ident.span()=>
        #max_len * <#ty as ::abstract_bits::AbstractBits>::MAX_BITS
    }
}
