/// Aplication errors and conversions from dependent APIs errors.
///
/// Included in `crate::prelude`
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("DataNotFound: {0}")]
    DataNotFound(String),

    #[error("DataNotFound: {0}. Possible values are [{}]", itertools::join(.1, ", "))]
    DataNotFoundIn(String, Vec<String>),

    #[error("MissingArgument {0}")]
    MissingArgument(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    HyprError(#[from] hyprland::shared::HyprError),
}
