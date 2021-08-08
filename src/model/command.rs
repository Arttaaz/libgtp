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
    ListString(List<String>),
    String(String),
    Float(f32),
    Int(u32),
    Entity(SimpleEntity),
    KataSize(RectSize),
}

impl Display for Args {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Collection(c) => write!(f, "{}", c),
            Self::ListVertex(l) => write!(f, "{}", l),
            Self::ListMove(l) => write!(f, "{}", l),
            Self::ListString(s) => write!(f, "{}", s),
            Self::String(s) => write!(f, "{}", s),
            Self::Float(float) => write!(f, "{}", float),
            Self::Int(i) => write!(f, "{}", i),
            Self::Entity(e) => write!(f, "{}", e),
            Self::KataSize(s) => write!(f, "{}", s),
        }
    }
}

impl Args {
    pub const fn int(i: u32) -> Self {
        Self::Int(i)
    }

    pub fn to_int(self) -> Result<u32, Self> {
        match self {
            Self::Int(i) => Ok(i),
            _ => Err(self),
        }
    }

    pub const fn as_int(&self) -> Option<&u32> {
        match self {
            Self::Int(i) => Some(i),
            _ => None,
        }
    }

    pub const fn float(f: f32) -> Self {
        Self::Float(f)
    }

    pub fn to_float(self) -> Result<f32, Self> {
        match self {
            Self::Float(f) => Ok(f),
            _ => Err(self),
        }
    }

    pub const fn as_float(&self) -> Option<&f32> {
        match self {
            Self::Float(f) => Some(f),
            _ => None,
        }
    }

    pub const fn string(s: String) -> Self {
        Self::String(s)
    }

    pub fn to_string(self) -> Result<String, Self> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err(self),
        }
    }

    pub const fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub const fn collection(c: Collection) -> Self {
        Self::Collection(c)
    }

    pub fn to_collection(self) -> Result<Collection, Self> {
        match self {
            Self::Collection(c) => Ok(c),
            _ => Err(self),
        }
    }

    pub const fn as_collection(&self) -> Option<&Collection> {
        match self {
            Self::Collection(c) => Some(c),
            _ => None,
        }
    }

    pub const fn list_vertex(l: List<Vertex>) -> Self {
        Self::ListVertex(l)
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

    pub const fn list_move(l: List<Move>) -> Self {
        Self::ListMove(l)
    }

    pub fn to_list_move(self) -> Result<List<Move>, Self> {
        match self {
            Self::ListMove(l) => Ok(l),
            _ => Err(self),
        }
    }

    pub const fn as_list_move(&self) -> Option<&List<Move>> {
        match self {
            Self::ListMove(l) => Some(l),
            _ => None,
        }
    }

    pub const fn list_string(l: List<String>) -> Self {
        Self::ListString(l)
    }

    pub fn to_list_string(self) -> Result<List<String>, Self> {
        match self {
            Self::ListString(l) => Ok(l),
            _ => Err(self),
        }
    }

    pub const fn as_list_string(&self) -> Option<&List<String>> {
        match self {
            Self::ListString(l) => Some(l),
            _ => None,
        }
    }

    pub const fn entity(e: SimpleEntity) -> Self {
        Self::Entity(e)
    }

    pub fn to_entity(self) -> Result<SimpleEntity, Self> {
        match self {
            Self::Entity(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub const fn as_entity(&self) -> Option<&SimpleEntity> {
        match self {
            Self::Entity(e) => Some(e),
            _ => None,
        }
    }

    pub const fn kata_size(e: RectSize) -> Self {
        Self::KataSize(e)
    }

    pub fn to_kata_size(self) -> Result<RectSize, Self> {
        match self {
            Self::KataSize(e) => Ok(e),
            _ => Err(self),
        }
    }

    pub const fn as_kata_size(&self) -> Option<&RectSize> {
        match self {
            Self::KataSize(e) => Some(e),
            _ => None,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct RectSize((u32, u32));
impl FromStr for RectSize {
    type Err = crate::model::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut matches = s.split_ascii_whitespace();
        let width = if let Some(s) = matches.next() {
            s.parse()?
        } else {
            return Err(Self::Err::WrongAlternative)
        };
        let height = if let Some(s) = matches.next() {
            s.parse()?
        } else {
            return Err(Self::Err::WrongAlternative)
        };
        Ok(Self((width, height)))
    }
}

impl Display for RectSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0.0, self.0.1)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub enum CommandName {
    ProtocolVersion,
    Name,
    Version,
    ListCommands,
    Quit,
    ClearBoard,
    Undo,
    FinalScore,
    Showboard,
    KnownCommand,
    Boardsize,
    Komi,
    FixedHandicap,
    PlaceFreeHandicap,
    SetFreeHandicap,
    Play,
    Genmove,
    TimeSettings,
    TimeLeft,
    FinalStatusList,
    Loadsgf,
    RegGenmove,
    Unknown,
    RectangularBoardsize,
    SetPosition,
    ClearCache,
    Stop,
    KataGetRules,
    KataSetRules,
    KataSetRule,
    KgsRules,
    KgsTimeSettings,
    LzAnalyze,
    KataAnalyze,
    LzGenmoveAnalyze,
    KataGenmoveAnalyze,
    Analyze,
    GenmoveAnalyze,
    KataRawNn,
    KataGetParam,
    Cputime,
    GomillCputime,
    GetKomi,
    KataListTimeSettings,
    KataSetParam,
}

impl FromStr for CommandName {
    type Err = crate::model::ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "protocol_version" => Ok(Self::ProtocolVersion),
            "name" => Ok(Self::Name),
            "version" => Ok(Self::Version),
            "list_commands" => Ok(Self::ListCommands),
            "quit" => Ok(Self::Quit),
            "clear_board" => Ok(Self::ClearBoard),
            "undo" => Ok(Self::Undo),
            "final_score" => Ok(Self::FinalScore),
            "showboard" => Ok(Self::Showboard),
            "known_command" => Ok(Self::KnownCommand),
            "boardsize" => Ok(Self::Boardsize),
            "komi" => Ok(Self::Komi),
            "fixed_handicap" => Ok(Self::FixedHandicap),
            "place_free_handicap" => Ok(Self::PlaceFreeHandicap),
            "set_free_handicap" => Ok(Self::SetFreeHandicap),
            "play" => Ok(Self::Play),
            "genmove" => Ok(Self::Genmove),
            "time_settings" => Ok(Self::TimeSettings),
            "time_left" => Ok(Self::TimeLeft),
            "final_status_list" => Ok(Self::FinalStatusList),
            "loadsgf" => Ok(Self::Loadsgf),
            "reg_genmove" => Ok(Self::RegGenmove),
            "rectangular_boardsize" => Ok(Self::RectangularBoardsize),
            "set_position" => Ok(Self::SetPosition),
            "clear_cache" => Ok(Self::ClearCache),
            "stop" => Ok(Self::Stop),
            "kata-get-rules" => Ok(Self::KataGetRules),
            "kata-set-rules" => Ok(Self::KataSetRules),
            "kata-set-rule" => Ok(Self::KataSetRule),
            "kgs-rules" => Ok(Self::KgsRules),
            "kgs-time_settings" => Ok(Self::KgsTimeSettings),
            "lz-analyze" => Ok(Self::LzAnalyze),
            "kata-analyze" => Ok(Self::KataAnalyze),
            "lz-genmove_analyze" => Ok(Self::LzGenmoveAnalyze),
            "kata-genmove_analyze" => Ok(Self::KataGenmoveAnalyze),
            "analyze" => Ok(Self::Analyze),
            "genmove_analyze" => Ok(Self::GenmoveAnalyze),
            "kata-raw-nn" => Ok(Self::KataRawNn),
            "kata-get-param" => Ok(Self::KataGetParam),
            "cputime" => Ok(Self::Cputime),
            "gomill-cpu_time" => Ok(Self::GomillCputime),
            "get_komi" => Ok(Self::GetKomi),
            "kata-list_time_settings" => Ok(Self::KataListTimeSettings),
            "kata-set-param" => Ok(Self::KataSetParam),
            _ => Err(Self::Err::WrongAlternative),
        }
    }
}

impl From<String> for CommandName {
    fn from(str: String) -> Self {
        match str.as_str() {
            "protocol_version" => Self::ProtocolVersion,
            "name" => Self::Name,
            "version" => Self::Version,
            "list_commands" => Self::ListCommands,
            "quit" => Self::Quit,
            "clear_board" => Self::ClearBoard,
            "undo" => Self::Undo,
            "final_score" => Self::FinalScore,
            "showboard" => Self::Showboard,
            "known_command" => Self::KnownCommand,
            "boardsize" => Self::Boardsize,
            "komi" => Self::Komi,
            "fixed_handicap" => Self::FixedHandicap,
            "place_free_handicap" => Self::PlaceFreeHandicap,
            "set_free_handicap" => Self::SetFreeHandicap,
            "play" => Self::Play,
            "genmove" => Self::Genmove,
            "time_settings" => Self::TimeSettings,
            "time_left" => Self::TimeLeft,
            "final_status_list" => Self::FinalStatusList,
            "loadsgf" => Self::Loadsgf,
            "reg_genmove" => Self::RegGenmove,
            "rectangular_boardsize" => Self::RectangularBoardsize,
            "set_position" => Self::SetPosition,
            "clear_cache" => Self::ClearCache,
            "stop" => Self::Stop,
            "kata-get-rules" => Self::KataGetRules,
            "kata-set-rules" => Self::KataSetRules,
            "kata-set-rule" => Self::KataSetRule,
            "kgs-rules" => Self::KgsRules,
            "kgs-time_settings" => Self::KgsTimeSettings,
            "lz-analyze" => Self::LzAnalyze,
            "kata-analyze" => Self::KataAnalyze,
            "lz-genmove_analyze" => Self::LzGenmoveAnalyze,
            "kata-genmove_analyze" => Self::KataGenmoveAnalyze,
            "analyze" => Self::Analyze,
            "genmove_analyze" => Self::GenmoveAnalyze,
            "kata-raw-nn" => Self::KataRawNn,
            "kata-get-param" => Self::KataGetParam,
            "cputime" => Self::Cputime,
            "gomill-cpu_time" => Self::GomillCputime,
            "get_komi" => Self::GetKomi,
            "kata-list_time_settings" => Self::KataListTimeSettings,
            "kata-set-param" => Self::KataSetParam,
            _ => Self::Unknown,
        }
    }
}

impl Display for CommandName {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProtocolVersion =>      write!(f, "protocol_version"),
            Self::Name =>                 write!(f, "name"),
            Self::Version =>              write!(f, "version"),
            Self::ListCommands =>         write!(f, "list_commands"),
            Self::Quit =>                 write!(f, "quit"),
            Self::ClearBoard =>           write!(f, "clear_board"),
            Self::Undo =>                 write!(f, "undo"),
            Self::FinalScore =>           write!(f, "final_score"),
            Self::Showboard =>            write!(f, "showboard"),
            Self::KnownCommand =>         write!(f, "known_command"),
            Self::Boardsize =>            write!(f, "boardsize"),
            Self::Komi =>                 write!(f, "komi"),
            Self::FixedHandicap =>        write!(f, "fixed_handicap"),
            Self::PlaceFreeHandicap =>    write!(f, "place_free_handicap"),
            Self::SetFreeHandicap =>      write!(f, "set_free_handicap"),
            Self::Play =>                 write!(f, "play"),
            Self::Genmove =>              write!(f, "genmove"),
            Self::TimeSettings =>         write!(f, "time_settings"),
            Self::TimeLeft =>             write!(f, "time_left"),
            Self::FinalStatusList =>      write!(f, "final_status_list"),
            Self::Loadsgf =>              write!(f, "loadsgf"),
            Self::RegGenmove =>           write!(f, "reg_genmove"),
            Self::Unknown =>              write!(f, "unknown"),
            Self::RectangularBoardsize => write!(f, "rectangular_boardsize"),
            Self::SetPosition =>          write!(f, "set_position"),
            Self::ClearCache =>           write!(f, "clear_cache"),
            Self::Stop =>                 write!(f, "stop"),
            Self::KataGetRules =>         write!(f, "kata-get_rules"),
            Self::KataSetRules =>         write!(f, "kata-set-rules"),
            Self::KataSetRule =>          write!(f, "kata-set-rule"),
            Self::KgsRules =>             write!(f, "kgs-rules"),
            Self::KgsTimeSettings =>      write!(f, "kgs-time_settings"),
            Self::LzAnalyze =>            write!(f, "lz-analyze"),
            Self::KataAnalyze =>          write!(f, "kata-analyze"),
            Self::LzGenmoveAnalyze =>     write!(f, "lz-genmove_analyze"),
            Self::KataGenmoveAnalyze =>   write!(f, "kata-genmove_analyze"),
            Self::Analyze =>              write!(f, "analyze"),
            Self::GenmoveAnalyze =>       write!(f, "genmove_analyze"),
            Self::KataRawNn =>            write!(f, "kata-raw-nn"),
            Self::KataGetParam =>         write!(f, "kata-get-param"),
            Self::Cputime =>              write!(f, "cputime"),
            Self::GomillCputime =>        write!(f, "gomill-cpu_time"),
            Self::GetKomi =>              write!(f, "get_komi"),
            Self::KataListTimeSettings => write!(f, "kata-list_time_settings"),
            Self::KataSetParam =>         write!(f, "kata-set-param"),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Command {
    id:     Option<u32>,
    name:   CommandName,
    args:   Option<Args>,
}

impl Display for Command {
    fn fmt(&self,  f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.id.is_some() {
            write!(f, "{} ", self.id.unwrap())?;
        }
        write!(f, "{}", self.name)?;
        if self.args.is_some() {
            write!(f, " {}", self.args.clone().unwrap())?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

impl FromStr for Command {
    type Err = crate::model::ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut matches : Vec<_> = str.split_ascii_whitespace().collect();
        let mut id = None;
        if let Ok(has_id) = matches[0].parse::<u32>() {
            id = Some(has_id);
            matches.remove(0);
        }
        let name = matches.remove(0);
        let args = matches.join(" ");

        match name {
            "protocol_version" |
            "name" |
            "version" |
            "list_commands" |
            "quit" |
            "clear_board" |
            "undo" |
            "final_score" |
            "showboard" |
            "clear_cache" |
            "stop" |
            "kata-get-rules" |
            "cputime" |
            "gomill-cpu_time" |
            "get_komi" |
            "kata-list_time_settings" |
            "kata-list-params"
                => Ok(Self {
                        id,
                        name: CommandName::from_str(name).unwrap(),
                        args: None
                    }),
            "known_command" |
            "final_status_list"
                => Ok(Self {
                        id,
                        name: CommandName::from_str(name).unwrap(),
                        args: Some(Args::string(args)),
                    }),
            "boardsize" |
            "fixed_handicap" |
            "place_free_handicap"
                => Ok(Self {
                        id,
                        name: CommandName::from_str(name).unwrap(),
                        args: Some(Args::int(args.parse()?)),
                    }),
            "komi"
                => Ok(Self{
                        id,
                        name: CommandName::from_str(name).unwrap(),
                        args: Some(Args::float(args.parse()?)),
                    }),
            "set_free_handicap"
                => Ok(Self{
                        id,
                        name: CommandName::from_str(name).unwrap(),
                        args: Some(Args::list_vertex(args.as_str().parse()?)),
                    }),
            "play" |
            "genmove" |
            "reg_genmove"
                => Ok(Self {
                        id,
                        name: CommandName::from_str(name).unwrap(),
                        args: Some(Args::entity(args.as_str().parse()?)),
                    }),
            "time_settings" |
            "time_left" |
            "loadsgf"
                => Ok(Self {
                        id,
                        name: CommandName::from_str(name).unwrap(),
                        args: Some(Args::collection(args.as_str().parse()?)),
                    }),
            "set_position" => Ok(Self {
                        id,
                        name: CommandName::from_str(name).unwrap(),
                        args: Some(Args::list_move(args.as_str().parse()?)),
            }),
            "rectangular_boardsize" => Ok(Self {
                        id,
                        name: CommandName::from_str(name).unwrap(),
                        args: Some(Args::kata_size(args.as_str().parse()?)),
            }),
            "kata-set-rules" |
            "kata-set-rule" |
            "kgs-rules" |
            "kgs-time_settings" |
            "kata-analyze" |
            "lz-analyze" |
            "analyze" |
            "lz-genmove_analyze" |
            "kata-genmove_analyze" |
            "genmove_analyze" |
            "kata-raw-nn" |
            "kata-get-param" |
            "kata-set-param"
                => Ok(Self {
                        id,
                        name: CommandName::from_str(name).unwrap(),
                        args: Some(Args::list_string(args.as_str().parse()?)),
                }),
            _ => Err(crate::model::ParseError::WrongCommandName) //unreachable
        }
    }
}

impl From<&Command> for String {
    fn from(command: &Command) -> Self {
        format!("{}", command)
    }
}

impl Into<String> for Command {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Command {
    pub const fn new(name: CommandName, args: Option<Args>) -> Self {
        Self {
            id: None,
            name,
            args,
        }
    }

    pub const fn with_id(id: u32, name: CommandName, args: Option<Args>) -> Self {
        Self {
            id: Some(id),
            name,
            args,
        }
    }

    pub const fn id(&self) -> &Option<u32> {
        &self.id
    }

    pub fn id_mut(&mut self) -> &mut Option<u32> {
        &mut self.id
    }

    pub const fn name(&self) -> CommandName {
        self.name
    }

    pub const fn args(&self) -> &Option<Args> {
        &self.args
    }

    pub fn args_mut(&mut self) -> &mut Option<Args> {
        &mut self.args
    }

}
