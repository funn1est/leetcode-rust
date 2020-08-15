use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LcError {
    #[error("fetch error")]
    Fetch,
    #[error("parse error")]
    Parse,
    #[error("can't find rust code")]
    NoRustCode,
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
