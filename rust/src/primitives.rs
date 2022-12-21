//! lightning network primitives implementation
//! in rust.
use crate::core::{FromWire, ToWire};
use std::io::{Error, Read, Write};
use std::mem::size_of;

/// macros to implement the FromWire and ToWire
/// for the basics types.
///
/// This implementation is inspired from the
/// rust-lightning implementation but it is a little bit
/// improved wit modern rust
macro_rules! to_wire_type {
    ($ty: ty) => {
        impl ToWire for $ty {
            fn to_wire<W: Write>(&self, buff: &mut W) -> Result<(), Error> {
                buff.write_all(&self.to_be_bytes())
            }
        }

        impl FromWire for $ty {
            fn from_wire<R: Read>(buff: &mut R) -> Self {
                const BUFF_SIZE: usize = size_of::<$ty>();
                let mut tmp_buff = [0; BUFF_SIZE];
                // FIXME: return error
                let size = buff.read(&mut tmp_buff).unwrap();
                let res = <$ty>::from_be_bytes(tmp_buff);
                assert_eq!(size, size_of::<$ty>());
                res
            }
        }
    };
}

to_wire_type!(u8);
to_wire_type!(u16);
to_wire_type!(u32);
to_wire_type!(u64);
