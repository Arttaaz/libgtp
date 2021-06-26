use alloc::string::String;
use alloc::string::ToString;
use core::str::FromStr;
use alloc::fmt;
use alloc::fmt::Display;
use alloc::vec::Vec;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Failure {
    id: Option<u32>,
    message: String,
}

impl Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "?")?;
        if self.id.is_some() {
            write!(f, "{}", self.id.unwrap())?;
        }
        write!(f, " {}\n\n", self.message)
    }
}

impl FromStr for Failure {
    type Err = crate::model::ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str.get(0..1) != Some("?") {
            return Err(Self::Err::WrongFailureFormat);
        }

        let str = str.split_at(1);
        let str = str.1;

        let split: Vec<&str> = str.splitn(2, " ").collect();
        let mut id = None;

        if let Ok(has_id) = split[0].parse::<u32>() {
            id = Some(has_id);
        }
        
        Ok(Self{
            id,
            message: split[1].trim_end().to_string(),
        })
    }
}

impl Failure {
    pub fn new(message: String) -> Self {
        Self {
            id: None,
            message,
        }
    }

    pub fn new_with_id(id: u32, message: String) -> Self {
        Self {
            id: Some(id),
            message,
        }
    }

    pub fn id(&self) -> &Option<u32> {
        &self.id
    }

    pub fn id_mut(&mut self) -> &mut Option<u32> {
        &mut self.id
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn message_mut(&mut self) -> &mut String {
        &mut self.message
    }
}


