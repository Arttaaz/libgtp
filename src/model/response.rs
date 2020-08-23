use crate::model::types::*;
use crate::model::CommandName;
use alloc::string::String;
use alloc::string::ToString;
use core::fmt::Display;
use core::fmt;
use core::str::FromStr;
use alloc::vec::Vec;
use log::debug;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
enum ResponseData {
    Integer(u32),
    String(String), // used only for name, version and showboard commands.
    Bool(Boolean),
    MultiLine(Vec<CommandName>),
    ListVertex(List<Vertex>),
    Move(Move),
    Score(Score),
    MultiLineList(Vec<List<Vertex>>),
}

impl Display for ResponseData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::String(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::MultiLine(s) => s.into_iter().try_for_each(|s| writeln!(f, "{}", s)),
            Self::ListVertex(v) => write!(f, "{}", v),
            Self::Move(m) => write!(f, "{}", m),
            Self::Score(s) => write!(f, "{}", s),
            Self::MultiLineList(l) => l.into_iter().try_for_each(|v| writeln!(f, "{}", v)),
        }
    }
}

impl FromStr for ResponseData {
    type Err = crate::model::ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        debug!("parsing from: {}", str);
        if let Ok(i) = str.parse::<u32>() {
            return Ok(Self::Integer(i));
        } else if let Ok(b) = str.parse::<Boolean>() {
            return Ok(Self::Bool(b));
        } else if let Ok(m) = str.parse::<Move>() {
            return Ok(Self::Move(m));
        } else if let Ok(s) = str.parse::<Score>() {
            return Ok(Self::Score(s));
        } else if let Ok(l) = str.parse::<List<Vertex>>() {
            return Ok(Self::ListVertex(l));
        }

        let mut matches: Vec<&str> = str.split('\n').collect();
        debug!("{:?}", &matches);
        if matches.len() > 1 {
            matches.pop();
            matches.pop();
            if let Ok(_) = matches[0].to_string().parse::<CommandName>() {
                return Ok(Self::MultiLine(matches.into_iter().map(|x| x.to_string().parse().unwrap()).collect()));
            }

            let lines: Vec<Result<List<Vertex>, Self::Err>> = matches.into_iter().map(|x| x.parse::<List<Vertex>>()).collect();
            let mut multilines = Vec::new();
            for line in lines {
                if let Err(_) = line {
                    return Ok(Self::String(str.to_string()));
                }
                multilines.push(line?);
            }
            Ok(Self::MultiLineList(multilines))
        } else {
            Err(Self::Err::WrongResponseData)
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Response {
    id: Option<u32>,
    data: Option<ResponseData>,
}

impl Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "=")?;
        if self.id.is_some() {
           write!(f, "{}", self.id.unwrap())?; 
        }
        if self.data.is_some() {
            write!(f, " {}", self.data.clone().unwrap())?;
            if let Some(ResponseData::MultiLine(_)) = self.data {
                write!(f, "\n")
            } else if let Some(ResponseData::MultiLineList(_)) = self.data {
                write!(f, "\n")
            } else {
                write!(f, "\n\n")
            }
        } else {
            write!(f, "\n\n")
        }
    }
}


impl FromStr for Response {
    type Err = crate::model::ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str.get(0..1) != Some("=") {
            return Err(Self::Err::WrongResponseFormat);
        }
        let str = str.split_at(1);
        let str = str.1;
        if str.is_empty() {
            return Ok(Self {
                id: None,
                data: None,
            });
        }

        let mut split = str.splitn(2, " ");    
        let mut id = None;
        if let Ok(has_id) = split.next().unwrap().trim_end().parse::<u32>() {
            id = Some(has_id); 
        }

        match split.next() {
            Some(data) => {
                
                let data = match ResponseData::from_str(data) {
                    Ok(d) => Some(d),
                    Err(e) => return Err(e),
                };

                Ok(Self {
                    id,
                    data,
                })
            },
            None => return Ok(Self {
                id,
                data: None,
            }),
        }
    }

}

impl Response {
    pub fn mov(mov: Move) -> Self { //TODO: find a better name as move is a rust keyword
        Self {
            id: None,
            data: Some(ResponseData::Move(mov)),
        }    
    }

    pub fn move_with_id(id: u32, mov: Move) -> Self {
        Self {
            id: Some(id),
            data: Some(ResponseData::Move(mov)),
        }
    }

    pub fn empty() -> Self {
        Self {
            id: None,
            data: None,
        }
    }

    pub fn empty_with_id(id: u32) -> Self {
        Self {
            id: Some(id),
            data: None,
        }
    }

    pub fn integer(int: u32) -> Self {
        Self {
            id: None,
            data: Some(ResponseData::Integer(int)),
        }
    }

    pub fn integer_with_id(id: u32, int: u32) -> Self { 
        Self {
            id: Some(id),
            data: Some(ResponseData::Integer(int)),
        }
    }
    
    pub fn name(name: String) -> Self {
        Self {
            id: None,
            data: Some(ResponseData::String(name)),
        }
    }

    pub fn name_with_id(id: u32, name: String) -> Self {
        Self {
            id: Some(id),
            data: Some(ResponseData::String(name)),
        }
    }

    pub fn version(version: String) -> Self {
        Self::name(version)
    }

    pub fn version_with_id(id: u32, version: String) -> Self {
        Self::name_with_id(id, version)
    }

    pub fn showboard<P>(f: P) -> Self
    where P: Fn() -> String {
        Self {
            id: None,
            data: Some(ResponseData::String(f())),
        }
    }

    pub fn bool(boolean: Boolean) -> Self {
        Self {
            id: None,
            data: Some(ResponseData::Bool(boolean)),
        }
    }
 
    pub fn bool_with_id(id: u32, boolean: Boolean) -> Self {
        Self {
            id: Some(id),
            data: Some(ResponseData::Bool(boolean)),
        }
    }

    pub fn command_names(commands: Vec<CommandName>) -> Self {
        Self {
            id: None,
            data: Some(ResponseData::MultiLine(commands)),
        }
    }
    
    pub fn command_names_with_id(id: u32, commands: Vec<CommandName>) -> Self {
        Self {
            id: Some(id),
            data: Some(ResponseData::MultiLine(commands)),
        }
    }

    pub fn list_vertex(vertices: List<Vertex>) -> Self {
        Self {
            id: None,
            data: Some(ResponseData::ListVertex(vertices)),
        }
    }

    pub fn list_vertex_with_id(id: u32, vertices: List<Vertex>) -> Self {
        Self {
            id: Some(id),
            data: Some(ResponseData::ListVertex(vertices)),
        }
    }

    pub fn score(score: Score) -> Self {
        Self {
            id: None,
            data: Some(ResponseData::Score(score)),
        }
    }

    pub fn score_with_id(id: u32, score: Score) -> Self {
        Self {
            id: Some(id),
            data: Some(ResponseData::Score(score)),
        }
    }

    pub fn multiline_list(list: Vec<List<Vertex>>) -> Self {
        Self {
            id: None,
            data: Some(ResponseData::MultiLineList(list)),
        }
    }

    pub fn multiline_list_with_id(id: u32, list: Vec<List<Vertex>>) -> Self {
        Self {
            id: Some(id),
            data: Some(ResponseData::MultiLineList(list)),
        }
    }
}

