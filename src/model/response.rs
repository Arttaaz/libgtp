use crate::model::types::*;
use crate::model::CommandName;
use alloc::string::String;
use alloc::string::ToString;
use core::fmt::Display;
use core::fmt;
use core::str::FromStr;
use alloc::vec::Vec;
//use log::debug;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum ResponseData {
    Integer(u32),
    String(String), // used only for name, version and showboard commands.
    Bool(Boolean),
    CommandNames(Vec<CommandName>),
    ListVertex(List<Vertex>),
    Move(Move),
    Score(Score),
    VertexLists(Vec<List<Vertex>>),
}

impl Display for ResponseData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::String(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::CommandNames(s) => s.into_iter().try_for_each(|s| writeln!(f, "{}", s)),
            Self::ListVertex(v) => write!(f, "{}", v),
            Self::Move(m) => write!(f, "{}", m),
            Self::Score(s) => write!(f, "{}", s),
            Self::VertexLists(l) => l.into_iter().try_for_each(|v| writeln!(f, "{}", v)),
        }
    }
}

impl FromStr for ResponseData {
    type Err = crate::model::ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
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
        if matches.len() > 1 {
            matches.pop();
            matches.pop();
            if let Ok(_) = matches[0].parse::<CommandName>() {
                return Ok(Self::CommandNames(matches.into_iter().map(|x| x.parse().unwrap()).collect()));
            }

            let lines: Vec<Result<List<Vertex>, Self::Err>> = matches.into_iter().map(|x| x.parse::<List<Vertex>>()).collect();
            let mut multilines = Vec::new();
            for line in lines {
                if let Err(_) = line {
                    return Ok(Self::String(str.to_string()));
                }
                multilines.push(line?);
            }
            Ok(Self::VertexLists(multilines))
        } else {
            Err(Self::Err::WrongResponseData)
        }
    }
}

impl ResponseData {
    pub fn to_int(self) -> Result<u32, Self> {
        match self {
            Self::Integer(i) => Ok(i),
            _ => Err(self),
        }
    }

    pub const fn as_int(&self) -> Option<&u32> {
        match self {
            Self::Integer(i) => Some(i),
            _ => None,
        }
    }

    pub const fn is_int(&self) -> bool {
        match self {
            Self::Integer(_) => true,
            _ => false,
        }
    }

    pub fn to_name(self) -> Result<String, Self> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err(self),
        }
    }

    pub const fn as_name(&self) -> Option<&String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub const fn is_name(&self) -> bool {
        match self {
            Self::String(_) => true,
            _ => false,
        }
    }

    pub fn to_bool(self) -> Result<Boolean, Self> {
        match self {
            Self::Bool(b) => Ok(b),
            _ => Err(self),
        }
    }

    pub const fn as_bool(&self) -> Option<&Boolean> {
        match self {
            Self::Bool(b) => Some(b),
            _ => None,
        }
    }

    pub const fn is_bool(&self) -> bool {
        match self {
            Self::Bool(_) => true,
            _ => false,
        }
    }

    pub fn to_command_names(self) -> Result<Vec<CommandName>, Self> {
        match self {
            Self::CommandNames(c) => Ok(c),
            _ => Err(self),
        }
    }

    pub const fn as_command_names(&self) -> Option<&Vec<CommandName>> {
        match self {
            Self::CommandNames(c) => Some(c),
            _ => None,
        }
    }

    pub const fn is_command_names(&self) -> bool {
        match self {
            Self::CommandNames(_) => true,
            _ => false,
        }
    }

    pub fn to_list_vertex(self) -> Result<List<Vertex>, Self> {
        match self {
            Self::ListVertex(l) => Ok(l),
            _ => Err(self),
        }
    }

    pub const fn as_list_vertex(&self) -> Option<&List<Vertex>> {
        match self {
            Self::ListVertex(l) => Some(l),
            _ => None,
        }
    }

    pub const fn is_list_vertex(&self) -> bool {
        match self {
            Self::ListVertex(_) => true,
            _ => false,
        }
    }

    pub fn to_move(self) -> Result<Move, Self> {
        match self {
            Self::Move(m) => Ok(m),
            _ => Err(self),
        }
    }

    pub const fn as_move(&self) -> Option<&Move> {
        match self {
            Self::Move(m) => Some(m),
            _ => None,
        }
    }

    pub const fn is_move(&self) -> bool {
        match self {
            Self::Move(_) => true,
            _ => false,
        }
    }

    pub fn to_score(self) -> Result<Score, Self> {
        match self {
            Self::Score(s) => Ok(s),
            _ => Err(self),
        }
    }

    pub const fn as_score(&self) -> Option<&Score> {
        match self {
            Self::Score(s) => Some(s),
            _ => None,
        }
    }

    pub const fn is_score(&self) -> bool {
        match self {
            Self::Score(_) => true,
            _ => false,
        }
    }

    pub fn to_vertex_lists(self) -> Result<Vec<List<Vertex>>, Self> {
        match self {
            Self::VertexLists(l) => Ok(l),
            _ => Err(self),
        }
    }
    
    pub const fn as_vertex_lists(&self) -> Option<&Vec<List<Vertex>>> {
        match self {
            Self::VertexLists(l) => Some(l),
            _ => None,
        }
    }

    pub const fn is_vertex_lists(&self) -> bool {
        match self {
            Self::VertexLists(_) => true,
            _ => false,
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
            if let Some(ResponseData::CommandNames(_)) = self.data {
                write!(f, "\n")
            } else if let Some(ResponseData::VertexLists(_)) = self.data {
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
    pub const fn mov(mov: Move) -> Self { //TODO: find a better name as move is a rust keyword
        Self {
            id: None,
            data: Some(ResponseData::Move(mov)),
        }    
    }

    pub const fn move_with_id(id: u32, mov: Move) -> Self {
        Self {
            id: Some(id),
            data: Some(ResponseData::Move(mov)),
        }
    }

    pub const fn empty() -> Self {
        Self {
            id: None,
            data: None,
        }
    }

    pub const fn empty_with_id(id: u32) -> Self {
        Self {
            id: Some(id),
            data: None,
        }
    }

    pub const fn integer(int: u32) -> Self {
        Self {
            id: None,
            data: Some(ResponseData::Integer(int)),
        }
    }

    pub const fn integer_with_id(id: u32, int: u32) -> Self { 
        Self {
            id: Some(id),
            data: Some(ResponseData::Integer(int)),
        }
    }
    
    pub const fn name(name: String) -> Self {
        Self {
            id: None,
            data: Some(ResponseData::String(name)),
        }
    }

    pub const fn name_with_id(id: u32, name: String) -> Self {
        Self {
            id: Some(id),
            data: Some(ResponseData::String(name)),
        }
    }

    pub const fn version(version: String) -> Self {
        Self::name(version)
    }

    pub const fn version_with_id(id: u32, version: String) -> Self {
        Self::name_with_id(id, version)
    }

    pub fn showboard<P>(f: P) -> Self
    where P: Fn() -> String {
        Self {
            id: None,
            data: Some(ResponseData::String(f())),
        }
    }

    pub const fn bool(boolean: Boolean) -> Self {
        Self {
            id: None,
            data: Some(ResponseData::Bool(boolean)),
        }
    }
 
    pub const fn bool_with_id(id: u32, boolean: Boolean) -> Self {
        Self {
            id: Some(id),
            data: Some(ResponseData::Bool(boolean)),
        }
    }

    pub const fn command_names(commands: Vec<CommandName>) -> Self {
        Self {
            id: None,
            data: Some(ResponseData::CommandNames(commands)),
        }
    }
    
    pub const fn command_names_with_id(id: u32, commands: Vec<CommandName>) -> Self {
        Self {
            id: Some(id),
            data: Some(ResponseData::CommandNames(commands)),
        }
    }

    pub const fn list_vertex(vertices: List<Vertex>) -> Self {
        Self {
            id: None,
            data: Some(ResponseData::ListVertex(vertices)),
        }
    }

    pub const fn list_vertex_with_id(id: u32, vertices: List<Vertex>) -> Self {
        Self {
            id: Some(id),
            data: Some(ResponseData::ListVertex(vertices)),
        }
    }

    pub const fn score(score: Score) -> Self {
        Self {
            id: None,
            data: Some(ResponseData::Score(score)),
        }
    }

    pub const fn score_with_id(id: u32, score: Score) -> Self {
        Self {
            id: Some(id),
            data: Some(ResponseData::Score(score)),
        }
    }

    pub const fn vertex_lists(list: Vec<List<Vertex>>) -> Self {
        Self {
            id: None,
            data: Some(ResponseData::VertexLists(list)),
        }
    }

    pub const fn vertex_lists_with_id(id: u32, list: Vec<List<Vertex>>) -> Self {
        Self {
            id: Some(id),
            data: Some(ResponseData::VertexLists(list)),
        }
    }

    pub const fn id(&self) -> &Option<u32> {
        &self.id
    }

    pub fn id_mut(&mut self) -> &mut Option<u32> {
        &mut self.id
    }

    pub const fn data(&self) -> &Option<ResponseData> {
        &self.data
    }
    
    pub fn data_mut(&mut self) -> &mut Option<ResponseData> {
        &mut self.data
    }
}

