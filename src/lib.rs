


pub mod validater;
pub mod common;
pub mod compare;
pub mod compressor; 

pub use validater::validate_email;
pub use validater::{FormatError,DataError,Error,RuleError};
pub use validater::run as validate;