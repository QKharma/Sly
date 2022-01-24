use std::error::Error;

pub type CommandResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
