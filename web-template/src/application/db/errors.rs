#[derive(Debug, thiserror::Error)]
#[error("Conn error: {message}")]
pub struct ConnError {
    message: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Begin transaction error: {message}")]
pub struct BeginError {
    message: String,
}

impl BeginError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Commit transaction error: {message}")]
pub struct CommitError {
    message: String,
}

impl CommitError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Rollback transaction error: {message}")]
pub struct RollbackError {
    message: String,
}

impl RollbackError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Transaction not begin")]
pub struct TransactionNotBegin;
