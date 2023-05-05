//! lnspec basic block to encode and decode the
//! basics types with rust.
pub mod bolt;
pub mod core;
pub mod primitives;
pub mod tlv;
pub mod types;

pub mod prelude {
    pub use crate::bolt::*;
    pub use crate::primitives::*;
    pub use crate::tlv::*;
    pub use crate::types::*;

    #[macro_export]
    macro_rules! error {
    ($($msg:tt)*) => {{
        let msg = format!($($msg)*);
        Err(std::io::Error::new(std::io::ErrorKind::Other, msg))
    }};
}

    pub use error;
}
