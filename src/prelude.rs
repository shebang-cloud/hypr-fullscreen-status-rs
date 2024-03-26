/// Crate Error
pub use crate::error::Error;

/// Fullscreen Status
pub use crate::status::Status;

/// Crate Result
pub type Result<T> = core::result::Result<T, Error>;
