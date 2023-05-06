//! Bitflag rust Implementation.
//!
//! This implementation is inspired and taken from
//! the awesome amplify library <https://github.com/rust-amplify/rust-amplify/blob/master/src/collection/flags.rs>
//!
//! Author: Vincenzo Palazzo <vincenzopalazzodev@gmail.com>.
use std::vec::Vec;

use crate::core::{FromWire, ToWire};

#[derive(Debug, Clone)]
pub struct BitFlag {
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

impl ToWire for BitFlag {
    fn to_wire<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let mut feature = self.trim();
        let len = feature.len() as u16;
        feature.reverse();

        len.to_wire(writer)?;
        writer.write_all(&feature)?;
        Ok(())
    }
}

impl FromWire for BitFlag {
    fn from_wire<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let size = u16::from_wire(reader)?;
        let mut buff = vec![0u8; size as usize];
        reader.read_exact(&mut buff)?;
        // Least significant bit first means that the least significant bit
        // will arrive first: hence e.g. the same hexadecimal number 0x12,
        // again `00010010` in binary representation, will arrive as
        // the (reversed) sequence `01001000`.
        buff.reverse();
        Ok(Self { inner: buff })
    }
}
