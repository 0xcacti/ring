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

#[derive(Debug, Clone, Error)]
#[error("{msg}")]
pub struct IPError {
    pub msg: String,
}

impl IPError {
    pub fn new(msg: String) -> Self {
        IPError { msg }
    }
}

#[derive(Debug, Clone, Error)]
#[error("{msg}")]
pub struct SocketError {
    pub msg: String,
}

impl SocketError {
    pub fn new(msg: String) -> Self {
        SocketError { msg }
    }
}
