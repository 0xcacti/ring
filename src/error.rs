use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[error("{msg}")]
pub struct ICMPError {
    pub msg: String,
}

impl ICMPError {
    pub fn new(msg: String) -> Self {
        ICMPError { msg }
    }
}
