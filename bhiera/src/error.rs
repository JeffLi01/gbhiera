use anyhow;

pub use anyhow::Error;
pub type Result<T> = anyhow::Result<T, Error>;
