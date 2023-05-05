//! lnspec basic block to encode and decode the
//! basics types with rust.
pub mod core;
pub mod primitives;
pub mod types;

pub mod prelude {
    #[macro_export]
    macro_rules! error {
    ($($msg:tt)*) => {{
        let msg = format!($($msg)*);
        Err(std::io::Error::new(std::io::ErrorKind::Other, msg))
    }};
}

    pub use error;
}
