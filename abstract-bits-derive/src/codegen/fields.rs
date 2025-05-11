use crate::model::Field;
use proc_macro2::TokenStream;

mod array;
mod control_list;
mod control_option;
mod list;
mod normal;
mod option;
mod padding;

impl Field {
    pub fn read_code(&self, struct_ident: &syn::Ident) -> TokenStream {
        let struct_name = proc_macro2::Literal::string(&struct_ident.to_string());
        match self {
            Field::Normal(normal_field) => normal::read(normal_field, &struct_name),
            Field::PaddBits(n_bits) => padding::read(*n_bits, &struct_name),
            Field::ControlList { controlled, bits } => {
                control_list::read(controlled, *bits, &struct_name)
            }
            Field::ControlOption(ident) => control_option::read(ident, &struct_name),
            Field::Option { inner_type, .. } => option::read(inner_type, &struct_name),
            Field::List { inner_type, .. } => list::read(inner_type, &struct_name),
            Field::Array {
                length,
                inner_type,
                field,
            } => array::read(length, inner_type, field, &struct_name),
        }
    }

    pub fn write_code(&self, struct_ident: &syn::Ident) -> TokenStream {
        let struct_name = proc_macro2::Literal::string(&struct_ident.to_string());
        match self {
            Field::Normal(normal_field) => normal::write(normal_field),
            Field::PaddBits(n_bits) => padding::write(*n_bits, &struct_name),
            Field::ControlList { controlled, bits } => control_list::write(controlled, *bits),
            Field::ControlOption(controlled) => control_option::write(controlled),
            Field::Option { inner_type, .. } => option::write(inner_type),
            Field::List { inner_type, .. } => list::write(inner_type),
            Field::Array { field, .. } => array::write(field),
        }
    }

    pub fn min_bits_code(&self) -> TokenStream {
        match self {
            Field::Normal(normal_field) => normal::min_bits(normal_field),
            Field::PaddBits(n_bits) => padding::min_bits(*n_bits),
            Field::ControlList { bits, .. } => control_list::min_bits(*bits),
            Field::ControlOption(_) => control_option::min_bits(),
            Field::Option { inner_type, .. } => option::min_bits(inner_type),
            Field::List { inner_type, .. } => list::min_bits(inner_type),
            Field::Array {
                inner_type, length, ..
            } => array::min_bits(inner_type, length),
        }
    }

    pub fn max_bits_code(&self) -> TokenStream {
        match self {
            Field::Normal(normal_field) => normal::max_bits(normal_field),
            Field::PaddBits(n_bits) => padding::max_bits(*n_bits),
            Field::ControlList { bits, .. } => control_list::max_bits(*bits),
            Field::ControlOption(_) => control_option::max_bits(),
            Field::Option { inner_type, .. } => option::max_bits(inner_type),
            Field::List {
                inner_type,
                max_len,
                ..
            } => list::max_bits(inner_type, *max_len),
            Field::Array {
                inner_type, length, ..
            } => array::max_bits(inner_type, length),
        }
    }
}
