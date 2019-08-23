use crate::model::types::List;
use crate::model::types::Collection;
use crate::model::types::Move;
use crate::model::types::Vertex;
use crate::model::types::SimpleEntity;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Display;
use core::str::FromStr;
#[cfg(feature = "serde")]
use serde::{ Deserialize, Serialize };

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum Args {
    Collection(Collection),
    ListVertex(List<Vertex>),
    ListMove(List<Move>),
    String(String),
    Float(f32),
    Int(u32),
    Entity(SimpleEntity),
    None,
}

impl Display for Args {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Collection(c) => write!(f, "{}", c),
            Self::ListVertex(l) => write!(f, "{}", l),
            Self::ListMove(l) => write!(f, "{}", l),
            Self::String(s) => write!(f, "{}", s),
            Self::Float(float) => write!(f, "{}", float),
            Self::Int(i) => write!(f, "{}", i),
            Self::Entity(e) => write!(f, "{}", e),
            _ => Ok(()),
        }
    }
}

impl Args {
    pub fn int(i: u32) -> Self {
        Self::Int(i)
    }

    pub fn to_int(self) -> Option<u32> {
        match self {
            Self::Int(i) => Some(i),
            _ => None,
        }
    }

    pub fn float(f: f32) -> Self {
        Self::Float(f)
    }

    pub fn to_float(self) -> Option<f32> {
        match self {
            Self::Float(f) => Some(f),
            _ => None,
        }
    }

    pub fn string(s: String) -> Self {
        Self::String(s)
    }

    pub fn to_string(self) -> Option<String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn collection(c: Collection) -> Self {
        Self::Collection(c)
    }

    pub fn to_collection(self) -> Option<Collection> {
        match self {
            Self::Collection(c) => Some(c),
            _ => None,
        }
    }

    pub fn list_vertex(l: List<Vertex>) -> Self {
        Self::ListVertex(l)
    }

    pub fn to_list_vertex(self) -> Option<List<Vertex>> {
        match self {
            Self::ListVertex(l) => Some(l),
            _ => None,
        }
    }

    pub fn list_move(l: List<Move>) -> Self {
        Self::ListMove(l)
    }

    pub fn to_list_move(self) -> Option<List<Move>> {
        match self {
            Self::ListMove(l) => Some(l),
            _ => None,
        }
    }

    pub fn entity(e: SimpleEntity) -> Self {
        Self::Entity(e)
    }

    pub fn to_entity(self) -> Option<SimpleEntity> {
        match self {
            Self::Entity(e) => Some(e),
            _ => None,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Command {
    id:     Option<u32>,
    name:   String,
    args:   Args,
}

impl Display for Command {
    fn fmt(&self,  f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.id.is_some() {
            write!(f, "{} ", self.id.unwrap())?;
        }
        write!(f, "{}", self.name)?;
        write!(f, " {}", self.args)?;
        Ok(())
    }
}

impl FromStr for Command {
    type Err = crate::model::ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut matches : Vec<String> = str.split_ascii_whitespace().map(|x| x.to_string()).collect();
        let mut id = None;
        if let Ok(has_id) = matches[0].parse::<u32>() {
            id = Some(has_id);
            matches.remove(0);
        }
        let name = matches.remove(0);
        let args = matches.concat();

        match name.as_str() {
            "protocol_version" |
            "name" |
            "version" |
            "list_commands" |
            "quit" |
            "clear_board" |
            "undo" |
            "final_score" |
            "showboard"
                => Ok(Self {
                        id,
                        name,
                        args: Args::None
                    }),
            _ => Err(crate::model::ParseError::WrongCommandName)
        }
    }
}

impl From<Command> for String {
    fn from(command: Command) -> Self {
        format!("{}", command)
    }
}

impl Command {
    pub fn new(name: String, args: Args) -> Self {
        Self {
            id: None,
            name,
            args,
        }
    }

    pub fn with_id(id: u32, name: String, args: Args) -> Self {
        Self {
            id: Some(id),
            name,
            args,
        }
    }

    pub fn id(&self) -> &Option<u32> {
        &self.id
    }

    pub fn id_mut(&mut self) -> &mut Option<u32> {
        &mut self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn args(&self) -> &Args {
        &self.args
    }

    pub fn args_mut(&mut self) -> &mut Args {
        &mut self.args
    }

}
