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
}

pub trait Entity : Display + Debug + Clone + FromStr {

}
