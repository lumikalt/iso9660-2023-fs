use thiserror::Error;

#[derive(Debug, Error)]
pub enum IsoError {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Invalid ISO9660 signature")]
    InvalidSignature,

    #[error("Invalid volume descriptor")]
    InvalidVolume,

    #[error("File not found: {0}")]
    NotFound(String),

    #[error("Not a directory: {0}")]
    NotADirectory(String),
}
