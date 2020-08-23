mod command;
mod failure;
mod response;
mod types;
use core::str::FromStr;
use core::fmt::Debug;
use core::fmt::Display;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

pub use types::*;
pub use command::*;
pub use response::*;
pub use failure::*;


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum Answer {
    Response(Response),
    Failure(Failure),
}

impl Answer {
    pub fn parse_answer(answer: &str) -> Result<Answer, ParseError> {
        if let Ok(response) = answer.parse::<Response>() {
         Ok(Answer::Response(response))
        } else if let Ok(failure) = answer.parse::<Failure>() {
         Ok(Answer::Failure(failure))
        } else {
         Err(ParseError::WrongAnswerFormat)
        }
    }

    pub fn is_response(&self) -> bool {
        match self {
            Self::Response(_) => true,
            Self::Failure(_) => false,
        }
    }

    pub fn is_failure(&self) -> bool {
        match self {
            Self::Response(_) => false,
            Self::Failure(_) => true,
        }
    }

    pub fn to_response(self) -> Option<Response> {
        match self {
            Self::Response(r) => Some(r),
            Self::Failure(_) => None,
        }
    }

    pub fn to_failure(self) -> Option<Failure> {
        match self {
            Self::Failure(f) => Some(f),
            Self::Response(_) => None,
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
