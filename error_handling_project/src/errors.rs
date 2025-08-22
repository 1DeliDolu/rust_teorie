use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Not found: {0}")]
    NotFound(String),
}
