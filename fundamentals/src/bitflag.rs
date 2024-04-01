//! Bitflag rust Implementation.
//!
//! This implementation is inspired and taken from
//! the awesome amplify library <https://github.com/rust-amplify/rust-amplify/blob/master/src/collection/flags.rs>
//!
//! Author: Vincenzo Palazzo <vincenzopalazzodev@gmail.com>.
use std::vec::Vec;

use crate::core::{FromWire, ToWire};

#[derive(Clone, Debug)]
pub struct BitFlag {
    pub len: u16,
    inner: Vec<u8>,
}

impl BitFlag {
    pub fn to_slice(&self) -> &[u8] {
        &self.inner
    }
}

impl ToWire for BitFlag {
    fn to_wire<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.len.to_wire(writer)?;
        writer.write_all(&self.inner)?;
        Ok(())
    }
}

impl FromWire for BitFlag {
    fn from_wire<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let size = u16::from_wire(reader)?;
        let mut buff = vec![0u8; size as usize];
        reader.read_exact(&mut buff)?;
        Ok(Self {
            len: size,
            inner: buff,
        })
    }
}
