pub mod command;
pub mod types;
use core::str::FromStr;
use core::fmt::Debug;
use core::fmt::Display;
pub use types::*;

#[derive(Debug)]
pub enum ParseError {
    WrongAlternative,
    WrongColor,
    WrongCommandName,
    WrongCoordinates,
    WrongSimpleEntity,
    WrongArgs,
}

impl From<core::num::ParseIntError> for ParseError {
    fn from(_err: core::num::ParseIntError) -> Self {
        Self::WrongArgs
    }
}

impl From<core::num::ParseFloatError> for ParseError {
    fn from(_err: core::num::ParseFloatError) -> Self {
        Self::WrongArgs
    }
}

pub trait Entity : Display + Debug + Clone + FromStr {

}
