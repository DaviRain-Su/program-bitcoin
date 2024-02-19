use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Custom Error({0})")]
    Custom(String),
}
