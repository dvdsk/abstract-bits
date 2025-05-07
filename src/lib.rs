#![doc = include_str!("../README.md")]

pub use arbitrary_int::{u1, u2, u3, u4, u5, u6, u7};
pub use bitvec;
use bitvec::order::Lsb0;
use bitvec::slice::BitSlice;
pub use abstract_bits_derive::abstract_bits;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum FromBytesError {
    #[error("Got invalid discriminant {got} while deserializing enum {ty}")]
    InvalidDiscriminant { ty: &'static str, got: usize },
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ToBytesError {
    #[error("List too long to fit. Max length {max}, got: {got}")]
    ListTooLong { max: usize, got: usize },
}

pub trait AbstractBits {
    fn needed_bits(&self) -> usize;
    /// To get the amount written use [`BitWriter::bits_written`] 
    /// or [`BitWriter::bytes_written`]
    fn write_abstract_bits(&self, writer: &mut BitWriter) -> Result<(), ToBytesError>;
    /// To get the amount read use [`BitReader::bits_read`] 
    /// or [`BitReader::bytes_read`]
    fn read_abstract_bits(reader: &mut BitReader) -> Result<Self, FromBytesError>
    where
        Self: Sized;

    fn to_abstract_bits(&self) -> Result<Vec<u8>, ToBytesError> {
        let mut buffer = vec![0u8; 100];
        let mut writer = BitWriter::from(buffer.as_mut_slice());
        self.write_abstract_bits(&mut writer)?;
        let bytes = writer.bytes_written();
        buffer.truncate(bytes);
        Ok(buffer)
    }

    fn from_abstract_bits(bytes: &[u8]) -> Result<Self, FromBytesError>
    where
        Self: Sized,
    {
        let mut reader = BitReader::from(bytes);
        Self::read_abstract_bits(&mut reader)
    }
}

macro_rules! impl_abstract_bits_for_UInt {
    ($base_type:ty, $write_method:ident, $read_method: ident) => {
        impl<const N: usize> AbstractBits for arbitrary_int::UInt<$base_type, N> {
            fn needed_bits(&self) -> usize {
                Self::BITS
            }

            fn write_abstract_bits(&self, writer: &mut BitWriter) -> Result<(), ToBytesError> {
                writer.$write_method(self.needed_bits(), self.value());
                Ok(())
            }

            fn read_abstract_bits(reader: &mut BitReader) -> Result<Self, FromBytesError>
            where
                Self: Sized,
            {
                let value = reader.$read_method(Self::BITS);
                Ok(Self::new(value))
            }
        }
    };
}

impl_abstract_bits_for_UInt! {u8, write_u8, read_u8}
impl_abstract_bits_for_UInt! {u16, write_u16, read_u16}
impl_abstract_bits_for_UInt! {u32, write_u32, read_u32}
impl_abstract_bits_for_UInt! {u64, write_u64, read_u64}

macro_rules! impl_abstract_bits_for_core_int {
    ($type:ty, $write_method:ident, $read_method:ident, $bits:literal) => {
        impl AbstractBits for $type {
            fn needed_bits(&self) -> usize {
                const { assert!(core::mem::size_of::<Self>() * 8 == $bits) }
                core::mem::size_of::<Self>() * 8
            }

            fn write_abstract_bits(&self, writer: &mut BitWriter) -> Result<(), ToBytesError> {
                writer.$write_method($bits, *self);
                Ok(())
            }

            fn read_abstract_bits(reader: &mut BitReader) -> Result<Self, FromBytesError>
            where
                Self: Sized,
            {
                Ok(reader.$read_method($bits))
            }
        }
    };
}

impl_abstract_bits_for_core_int! {u8, write_u8, read_u8, 8}
impl_abstract_bits_for_core_int! {u16, write_u16, read_u16, 16}
impl_abstract_bits_for_core_int! {u32, write_u32, read_u32, 32}
impl_abstract_bits_for_core_int! {u64, write_u64, read_u64, 64}

impl AbstractBits for bool {
    fn needed_bits(&self) -> usize {
        1
    }

    fn write_abstract_bits(&self, writer: &mut BitWriter) -> Result<(), ToBytesError> {
        writer.write_bit(*self);
        Ok(())
    }

    fn read_abstract_bits(reader: &mut BitReader) -> Result<Self, FromBytesError>
    where
        Self: Sized,
    {
        Ok(reader.read_bit())
    }
}

impl<const N: usize, T: AbstractBits + Sized> AbstractBits for [T; N] {
    fn needed_bits(&self) -> usize {
        self.iter().map(|item| item.needed_bits()).sum()
    }

    fn write_abstract_bits(&self, writer: &mut BitWriter) -> Result<(), ToBytesError> {
        for element in self.iter() {
            element.write_abstract_bits(writer)?;
        }
        Ok(())
    }
    fn read_abstract_bits(reader: &mut BitReader) -> Result<Self, FromBytesError>
    where
        Self: Sized,
    {
        let mut res = Vec::new();
        for _ in 0..N {
            res.push(T::read_abstract_bits(reader)?);
        }

        res.try_into()
            .map_err(|_| unreachable!("for loop ensures vec length matches array's"))
    }
}

// For now these use owned fixed size arrays. In the future we might want to
// borrow those, that could help minimize stack usage on embedded.
pub struct BitWriter<'a> {
    pos: usize,
    buf: &'a mut BitSlice<u8, Lsb0>,
}
pub struct BitReader<'a> {
    pos: usize,
    buf: &'a BitSlice<u8, Lsb0>,
}

impl BitReader<'_> {
    pub fn bits_read(&self) -> usize {
        self.pos
    }
    /// 12 bits read corrosponds to 2 bytes read
    pub fn bytes_read(&self) -> usize {
        self.pos.div_ceil(8)
    }
    pub fn skip(&mut self, n_bits: usize) {
        self.pos += n_bits;
    }
    fn read_bit(&mut self) -> bool {
        let res = self
            .buf
            .get(self.pos)
            .expect("should not call read after reader is at end of buffer");
        self.pos += 1;
        *res
    }
    fn read_u8(&mut self, n_bits: usize) -> u8 {
        let mut res = 0u8;
        let res_bits = BitSlice::<_, Lsb0>::from_element_mut(&mut res);
        res_bits[0..n_bits].copy_from_bitslice(&self.buf[self.pos..self.pos + n_bits]);
        self.pos += n_bits;
        res
    }
    fn read_u16(&mut self, n_bits: usize) -> u16 {
        let mut res = [0u8; 2];
        let res_bits = BitSlice::<_, Lsb0>::from_slice_mut(&mut res);
        res_bits[0..n_bits].copy_from_bitslice(&self.buf[self.pos..self.pos + n_bits]);
        self.pos += n_bits;
        u16::from_le_bytes(res)
    }
    fn read_u32(&mut self, n_bits: usize) -> u32 {
        let mut res = [0u8; 4];
        let res_bits = BitSlice::<_, Lsb0>::from_slice_mut(&mut res);
        res_bits[0..n_bits].copy_from_bitslice(&self.buf[self.pos..self.pos + n_bits]);
        self.pos += n_bits;
        u32::from_le_bytes(res)
    }
    fn read_u64(&mut self, n_bits: usize) -> u64 {
        let mut res = [0u8; 8];
        let res_bits = BitSlice::<_, Lsb0>::from_slice_mut(&mut res);
        res_bits[0..n_bits].copy_from_bitslice(&self.buf[self.pos..self.pos + n_bits]);
        self.pos += n_bits;
        u64::from_le_bytes(res)
    }
}

impl<'a> From<&'a [u8]> for BitReader<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self {
            pos: 0,
            buf: BitSlice::from_slice(bytes),
        }
    }
}

impl BitWriter<'_> {
    pub fn bits_written(&self) -> usize {
        self.pos
    }
    /// 12 bits read corrosponds to 2 bytes read
    pub fn bytes_written(&self) -> usize {
        self.pos.div_ceil(8)
    }
    pub fn skip(&mut self, n_bits: usize) {
        self.pos += n_bits;
    }
    fn write_bit(&mut self, bit: bool) {
        self.buf.set(self.pos, bit);
        self.pos += 1;
    }
    fn write_u8(&mut self, n_bits: usize, val: u8) {
        let val = BitSlice::<_, Lsb0>::from_element(&val);
        self.buf[self.pos..self.pos + n_bits].copy_from_bitslice(&val[..n_bits]);
        self.pos += n_bits;
    }
    fn write_u16(&mut self, n_bits: usize, val: u16) {
        let val = val.to_le_bytes();
        let val = BitSlice::<_, Lsb0>::from_slice(&val);
        self.buf[self.pos..self.pos + n_bits].copy_from_bitslice(&val[..n_bits]);
        self.pos += n_bits;
    }
    fn write_u32(&mut self, n_bits: usize, val: u32) {
        let val = val.to_le_bytes();
        let val = BitSlice::<_, Lsb0>::from_slice(&val);
        self.buf[self.pos..self.pos + n_bits].copy_from_bitslice(&val[..n_bits]);
        self.pos += n_bits;
    }
    fn write_u64(&mut self, n_bits: usize, val: u64) {
        let val = val.to_le_bytes();
        let val = BitSlice::<_, Lsb0>::from_slice(&val);
        self.buf[self.pos..self.pos + n_bits].copy_from_bitslice(&val[..n_bits]);
        self.pos += n_bits;
    }
}

impl<'a> From<&'a mut [u8]> for BitWriter<'a> {
    fn from(buf: &'a mut [u8]) -> Self {
        Self {
            pos: 0,
            buf: BitSlice::from_slice_mut(buf),
        }
    }
}
