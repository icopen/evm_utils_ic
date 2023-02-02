use std::fmt::{self, Display};

#[derive(Debug)]
pub enum TransactionError {
    InvalidType,
}

impl std::error::Error for TransactionError {}

impl Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{self:?}"))?;
        Ok(())
    }
}
