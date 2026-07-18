use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    AlreadyExists(String),
    NotFound(String),
    InvalidInput(String),
    Other(String),
}

pub type Result<T> = std::result::Result<T, AppError>;

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "Error de I/O: {}", err),
            AppError::AlreadyExists(msg) => write!(f, "Ya existe: {}", msg),
            AppError::NotFound(msg) => write!(f, "No encontrado: {}", msg),
            AppError::InvalidInput(msg) => write!(f, "Entrada inválida: {}", msg),
            AppError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl StdError for AppError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}
