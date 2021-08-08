mod command;
mod failure;
mod info;
mod response;
mod types;
use core::str::FromStr;
use core::fmt::Debug;
use core::fmt::Display;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

pub use command::*;
pub use failure::*;
pub use info::*;
pub use response::*;
pub use types::*;


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum Answer {
    Failure(Failure),
    Info(Info),
    Response(Response),
}

impl Answer {
    pub fn parse_answer(answer: &str) -> Result<Answer, ParseError> {
        if let Ok(response) = answer.parse::<Response>() {
            Ok(Answer::Response(response))
        } else if let Ok(info) = answer.parse::<Info>() {
            Ok(Answer::Info(info))
        } else if let Ok(failure) = answer.parse::<Failure>() {
            Ok(Answer::Failure(failure))
        } else {
            Err(ParseError::WrongAnswerFormat)
        }
    }

    pub const fn is_response(&self) -> bool {
        match self {
            Self::Response(_) => true,
            _ => false,
        }
    }

    pub const fn is_failure(&self) -> bool {
        match self {
            Self::Failure(_) => true,
            _ => false,
        }
    }

    pub const fn is_info(&self) -> bool {
        match self {
            Self::Info(_) => true,
            _ => false,
        }
    }

    pub fn to_response(self) -> Result<Response, Self> {
        match self {
            Self::Response(r) => Ok(r),
            _ => Err(self),
        }
    }

    pub fn to_failure(self) -> Result<Failure, Self> {
        match self {
            Self::Failure(f) => Ok(f),
            _ => Err(self),
        }
    }

    pub fn to_info(self) -> Result<Info, Self> {
        match self {
            Self::Info(i) => Ok(i),
            _ => Err(self),
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum ParseError {
    WrongAlternative,
    WrongColor,
    WrongCommandName,
    WrongCoordinates,
    WrongSimpleEntity,
    WrongArgs,
    WrongBool,
    WrongResponseData,
    WrongResponseFormat,
    WrongFailureFormat,
    WrongAnswerFormat,
    WrongScore,
    EmptyString,
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

impl Entity for String {}
