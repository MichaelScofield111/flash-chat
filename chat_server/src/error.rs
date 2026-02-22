use argon2::password_hash;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // #[from] is auto to convert sqlx::Error to  AppError
    // #[error(...)]` 让你能 `eprintln!("{}", err)` 打印出人类可读的错误信息
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("argon2 error: {0}")]
    ArgonError(#[from] password_hash::Error),
}
