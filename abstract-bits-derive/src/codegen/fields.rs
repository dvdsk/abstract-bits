use proc_macro2::TokenStream;
use crate::model::Field;

mod array;
mod control_list;
mod control_option;
mod list;
mod normal;
mod option;
mod padding;

impl Field {
    pub fn read_code(&self) -> TokenStream {
        match self {
            Field::Normal(normal_field) => normal::read(normal_field),
            Field::PaddBits(n_bits) => padding::read(*n_bits),
            Field::ControlList { controlled, bits } => control_list::read(controlled, *bits),
            Field::ControlOption(ident) => control_option::read(ident),
            Field::Option { inner_type, .. } => option::read(inner_type),
            Field::List { inner_type, .. } => list::read(inner_type),
            Field::Array {
                length,
                inner_type,
                field,
            } => array::read(length, inner_type, field),
        }
    }

    pub fn write_code(&self) -> TokenStream {
        match self {
            Field::Normal(normal_field) => normal::write(normal_field),
            Field::PaddBits(n_bits) => padding::write(*n_bits),
            Field::ControlList { controlled, bits } => control_list::write(controlled, *bits),
            Field::ControlOption(controlled) => control_option::write(controlled),
            Field::Option { inner_type, .. } => option::write(inner_type),
            Field::List { inner_type, .. } => list::write(inner_type),
            Field::Array { field, .. } => array::write(field),
        }
    }

    pub fn needed_bits_code(&self) -> TokenStream {
        match self {
            Field::Normal(normal_field) => normal::needed_bits(normal_field),
            Field::PaddBits(n_bits) => padding::needed_bits(*n_bits),
            Field::ControlList { bits, .. } => control_list::needed_bits(*bits),
            Field::ControlOption(_) => control_option::needed_bits(),
            Field::Option { inner_type, .. } => option::needed_bits(inner_type),
            Field::List { inner_type, max_len, .. } => list::needed_bits(inner_type, *max_len),
            Field::Array { inner_type, length, .. } => array::needed_bits(inner_type, length),
        }
    }
}
