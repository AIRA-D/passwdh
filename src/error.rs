//! Модуль определений ошибок приложения.
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorT {
    /// Ошибка в переданных аргументах или параметрах маски
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    /// Ошибка при попытке генерации пароля
    #[error("Generation failed: {0}")]
    GenerationFailed(String),
    /// Ошибки ввода-вывода
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ErrorT>;