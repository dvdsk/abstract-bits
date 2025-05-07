use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::Ident;
use syn::spanned::Spanned;

use crate::codegen::{is_primitive, list_len_ident};
use crate::model::{EmptyVariant, NormalField};

pub fn padding_field_code(n_bits: u8) -> TokenStream {
    let n_bits = proc_macro2::Literal::usize_suffixed(n_bits as usize);
    quote! { reader.skip(#n_bits); }
}

pub fn normal_field_code(
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

pub fn option_field_code(field: &NormalField) -> TokenStream {
    let is_some_ident = is_some_ident(&field.ident);
    let field_ident = &field.ident;
    let field_read_code = normal_field_code(field);
    quote_spanned! {field.ident.span()=>
        let #field_ident = if #is_some_ident {
            #field_read_code
            Some(#field_ident)
        } else {
            None
        };
    }
}

pub fn enum_code(variants: &[EmptyVariant], repr: Ident, bits: usize) -> TokenStream {
    let variants_discriminants = variants
        .iter()
        .map(|v| v.discriminant)
        .map(proc_macro2::Literal::usize_unsuffixed);
    let variant_idents = variants.iter().map(|v| &v.ident);

    let read_discriminant = if is_primitive(bits).is_some() {
        quote_spanned! {repr.span()=>
            let discriminant = #repr::read_abstract_bits(reader)?;
        }
    } else {
        let utype: syn::Type =
            syn::parse_str(&format!("::abstract_bits::u{bits}")).expect("valid type path");
        quote_spanned! {repr.span()=>
            let discriminant = #utype::read_abstract_bits(reader)?;
            let discriminant = discriminant.value();
        }
    };

    quote! {
        #read_discriminant;
        match discriminant {
            #(#variants_discriminants => Ok(Self::#variant_idents)),*,
            invalid => Err(::abstract_bits::FromBytesError::InvalidDiscriminant {
                ty: std::any::type_name::<Self>(),
                got: discriminant as usize,
            }),
        }
    }
}

/// Turns `Type<T>` into `Type::<T>` which is needed for
/// `Type::<T>::read_abstract_bits(reader)`
pub fn generics_to_fully_qualified(mut ty: syn::Type) -> syn::Type {
    if let syn::Type::Path(typath) = &mut ty {
        let syn::Path { segments, .. } = &mut typath.path;
        let first_seg = segments
            .first_mut()
            .expect("type path always has at least one segment");
        let syn::PathArguments::AngleBracketed(args) = &mut first_seg.arguments else {
            return ty;
        };

        args.colon2_token = Some(syn::Token![::](args.span()));
    }
    ty
}

fn is_some_ident(controlled: &Ident) -> Ident {
    format_ident!("{controlled}_is_some")
}

pub fn control_option_code(controlled: &Ident) -> TokenStream {
    let controller_ident = is_some_ident(controlled);
    quote_spanned! {controlled.span()=>
        let #controller_ident = bool::read_abstract_bits(reader)?;
    }
}

pub fn control_list_code(controlled: &Ident, bits: usize) -> TokenStream {
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

pub(crate) fn list_field_code(field: &NormalField) -> TokenStream {
    let len_ident = list_len_ident(&field.ident);
    let field_ident = &field.ident;
    quote_spanned! {field.ident.span()=>
        let res = (0..#len_ident).into_iter().map(|_|
            ::abstract_bits::AbstractBits::read_abstract_bits(reader)
        ).collect::<Result<_, ::abstract_bits::FromBytesError>>();
        let #field_ident = res?;
    }
}

pub(crate) fn array_code(length: &syn::Expr, ty: &syn::Type, field: &syn::Field) -> TokenStream {
    let field_ident = &field.ident;
    quote_spanned! {field.ident.span()=>
        const LEN: usize = #length;

        // Any panic beyond this pint will leak memory
        let mut array: [::core::mem::MaybeUninit<#ty>; LEN] = unsafe {
            // # SAFETY
            // MaybeUninit<T> does not require initialization. It also does not
            // drop the `T`.
            ::core::mem::MaybeUninit::uninit().assume_init()
        };

        for i in 0..LEN {
            match <#ty as ::abstract_bits::AbstractBits>::read_abstract_bits(reader) {
                Ok(val) => {
                    array[i] = ::core::mem::MaybeUninit::new(val);
                }
                Err(e) => {
                    // # SAFETY
                    // `array[0..i]` are initialized, we need to drop those elements
                    unsafe {
                        for j in 0..i {
                            array[j].assume_init_drop();
                        }
                    }
                    return Err(e);
                }
            } // match
        } // for

        // # SAFETY
        // The loop completed, every element is therefore initialized. In memory 
        // arrays of `MaybeUninit<T>` look the same as `T`, therefore the transmute
        // is safe
        let res = unsafe { ::core::mem::transmute::<_, [#ty; LEN]>(array) };
        let #field_ident = res;
    }
}
