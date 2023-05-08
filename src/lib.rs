

mod validater;
mod common;

pub use validater::{run as validate,validate_email};
pub use validater::{FormatError,DataError,Error,RuleError};
