use thiserror::Error;

/// A specialized `Result` type for `borealis` operations.
pub type BorealisResult<T> = Result<T, BorealisError>;

#[derive(Debug, Error)]
pub enum BorealisError {
    #[error(transparent)]
    HTTPError(#[from] reqwest::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
