

mod validater;
mod common;
pub mod compare;

pub use validater::validate_email;
pub use validater::{FormatError,DataError,Error,RuleError};
pub use validater::run as validate;