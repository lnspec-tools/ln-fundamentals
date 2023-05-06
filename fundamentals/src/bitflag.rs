//! Bitflag rust Implementation.
//!
//! This implementation is inspired and taken from
//! the awesome amplify library <https://github.com/rust-amplify/rust-amplify/blob/master/src/collection/flags.rs>
//!
//! Author: Vincenzo Palazzo <vincenzopalazzodev@gmail.com>.
use std::vec::Vec;

use crate::core::{FromWire, ToWire};

#[derive(Clone)]
pub struct BitFlag {
    pub len: u16,
    inner: Vec<u8>,
}

impl BitFlag {
    fn bits_to_bytes(bits: usize) -> usize {
        if bits == 0 {
            0
        } else {
            (bits / 8 + 1) as usize
        }
    }

    fn trim(&self) -> Vec<u8> {
        if self.inner.is_empty() {
            return Vec::new();
        }
        let buff = self.inner.clone();
        let size = buff.len();
        let mut top = 0;
        while top < size && !self.is_set(size - top) {
            top += 1;
        }
        let top = size - top;
        let used = Self::bits_to_bytes(top);
        if used < size {
            let mut sub_buff = vec![0u8; used];
            sub_buff.copy_from_slice(&self.inner[..used]);
            return sub_buff;
        }
        buff
    }

    /// Returns whether a feature flag with `flag_no` is set (`true` or `false`)
    #[inline]
    pub fn is_set(&self, flag_no: usize) -> bool {
        self.byte_at(flag_no)
            .map(|byte| (byte & (1 << (flag_no % 8))) > 0)
            .unwrap_or(false)
    }

    /// Returns reference to the byte responsible for the feature flag
    /// `flag_no`. If the maximum capacity is exceeded, returns
    /// [`Option::None`].
    #[inline]
    fn byte_at(&self, flag_no: usize) -> Option<&u8> {
        if flag_no >= self.inner.len() {
            return None;
        }
        Some(&self.inner[flag_no as usize / 8])
    }
}

impl std::fmt::Debug for BitFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "   len: {:?}", self.len)?;
        writeln!(f, "   hex: {:x}", self)?;
        write!(f, "}}")
    }
}

impl std::fmt::LowerHex for BitFlag {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for byte in self.inner.iter() {
            fmtr.write_fmt(format_args!("{:02x}", byte))?;
        }
        Ok(())
    }
}

impl ToWire for BitFlag {
    fn to_wire<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let feature = self.trim();

        self.len.to_wire(writer)?;
        writer.write_all(&feature)?;
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
