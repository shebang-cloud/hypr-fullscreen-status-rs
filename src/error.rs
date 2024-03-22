/// Aplication errors and conversions from dependent APIs errors.
///
/// Included in crate::prelude
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("DataNotFound {0}")]
    DataNotFound(String),

    #[error("MissingArgumen {0}")]
    MissingArgument(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    HyprError(#[from] hyprland::shared::HyprError),
}
