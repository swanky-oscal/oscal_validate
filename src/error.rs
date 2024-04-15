use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("General error: {0}")]
    General(&'static str),
    #[error("Failed to cast Any to object: {0}")]
    FailedDowncast(&'static str),
}
