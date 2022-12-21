//! core API of the lnspec crater.
use std::io::{Error, Read, Write};

/// trait that implement the logic
/// to encode a type into the lightning
/// network wire protocol.
pub trait ToWire {
    fn to_wire<W: Write>(&self, buff: &mut W) -> Result<(), Error>;
}

/// trait that implement the logic
/// to decode a type from a network
/// wire protocol.
pub trait FromWire {
    fn from_wire<R: Read>(buff: &mut R) -> Self;
}
