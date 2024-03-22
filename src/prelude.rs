/// Crate Error
pub use crate::error::Error;

/// Crate Result
pub type Result<T> = core::result::Result<T, Error>;