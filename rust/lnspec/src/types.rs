//! Lightning Network types implementation
use crate::core::{FromWire, ToWire};
use std::io::{Error, Read, Write};

macro_rules! to_wire_type_with_size {
    ($ty: ty, $size: expr) => {
        impl ToWire for $ty {
            fn to_wire<W: Write>(&self, buff: &mut W) -> Result<(), Error> {
                buff.write_all(self)
            }
        }

        impl FromWire for $ty {
            fn from_wire<R: Read>(buff: &mut R) -> Self {
                let mut tmp_buff = [0; $size];
                // FIXME: return error
                let _size = buff.read(&mut tmp_buff).unwrap();
                tmp_buff
            }
        }
    };
}

pub type ChainHash = [u8; 32];
pub type ChannelId = [u8; 32];
pub type Sha256 = [u8; 32];
pub type Signature = [u8; 64];
pub type Point = [u8; 33];
pub type ShortChannelId = [u8; 8];

to_wire_type_with_size!(ChainHash, 32);
to_wire_type_with_size!(Signature, 64);
to_wire_type_with_size!(Point, 33);
to_wire_type_with_size!(ShortChannelId, 8);

/// BigSize type implementation.
pub struct BigSize {
    value: u64,
}

impl FromWire for BigSize {
    fn from_wire<R: Read>(buff: &mut R) -> Self {
        let flag = u8::from_wire(buff);
        match flag {
            0xFF => {
                let value = u64::from_wire(buff);
                if value < 0x100000000 {
                    panic!("invalid encoding")
                } else {
                    return BigSize { value };
                }
            }
            0xFE => {
                let value = u32::from_wire(buff);
                if value < 0x10000 {
                    panic!("invalid")
                } else {
                    return BigSize {
                        value: value as u64,
                    };
                }
            }
            0xFD => {
                let value = u16::from_wire(buff);
                if value < 0xFD {
                    panic!("error")
                } else {
                    return BigSize {
                        value: value as u64,
                    };
                }
            }
            _ => BigSize { value: flag as u64 },
        }
    }
}

impl ToWire for BigSize {
    fn to_wire<W: Write>(&self, buff: &mut W) -> Result<(), crate::core::IOError> {
        match self.value {
            0..=0xFC => (self.value as u8).to_wire(buff),
            0xFD..=0xFFFF => {
                0xFDu8.to_wire(buff)?;
                (self.value as u16).to_wire(buff)
            }
            0x10000..=0xFFFFFFFF => {
                0xFEu8.to_wire(buff)?;
                (self.value as u32).to_wire(buff)
            }
            _ => {
                0xFFu8.to_wire(buff)?;
                (self.value as u64).to_wire(buff)
            }
        }
    }
}
