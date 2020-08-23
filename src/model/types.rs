use crate::model::Entity;
use alloc::borrow::ToOwned;
use alloc::fmt::Display;
use core::fmt;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::iter::FromIterator;
use core::ops::IndexMut;
use core::ops::Index;
use core::str::FromStr;
use super::ParseError;

//use log::debug;

#[cfg(feature = "serde")]
use serde::{ Deserialize, Serialize };

// MACROS
#[macro_export]
macro_rules! collection {
    ($($elem:expr),*) => {
        {
            let mut v : Vec<crate::model::SimpleEntity> = Vec::new();

            $(
                v.push($elem);
            )*
            crate::model::Collection::from_vec(v)
        }
    };
}

#[macro_export]
macro_rules! list {
    ($t:ty; $($elem:expr),*) => {
        {
            let mut v : Vec<$t> = Vec::new();

            $(
                v.push($elem);
            )*
            crate::model::types::List::from_vec(v)
        }
    };
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub enum Boolean {
    True,
    False,
}

impl Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}

impl FromStr for Boolean {
    type Err = crate::model::ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str.to_uppercase().as_str() {
            "TRUE" => Ok(Self::True),
            "FALSE" => Ok(Self::False),
            _ => Err(Self::Err::WrongBool),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Score(String);

impl FromStr for Score {
    type Err = crate::model::ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str.is_empty() {
            return Err(Self::Err::EmptyString);
        } else if str == "0" {
            Ok(Self(str.to_string()))
        } else {
            let split: Vec<&str> = str.split('+').collect();
            if split.len() > 2 {
                return Err(Self::Err::WrongScore);
            }

            match split[0] {
                "W" | "B" => (),
                _ => {return Err(Self::Err::WrongColor);}
            }

            split[1].parse::<f32>()?;

            Ok(Self(str.to_string()))
        }
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vertex {
    Coord(u8, u8),
    Pass,
    Resign,
}

impl Entity for Vertex {}

impl FromStr for Vertex {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str.is_empty() {
            return Err(Self::Err::EmptyString);
        }
        let str = str.to_uppercase();
        if str == "PASS" {
            return Ok(Self::Pass)
        } else if str == "RESIGN" {
            return Ok(Self::Resign)
        } else if str == "" {
            return Err(ParseError::WrongCoordinates)
        }

        let mut c = str.bytes().next().unwrap() - 64;
        if (1..=20).contains(&c) {
            if c > 9 { // We skip I on the goban for readability
                c -= 1;
            }

            if let Ok(number) = str[1..].parse::<u8>() {
                Ok(Self::Coord(c, number))
            } else {
                Err(ParseError::WrongCoordinates)
            }
        } else {
            Err(ParseError::WrongCoordinates)
        }
    }
}

impl Display for Vertex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Coord(x, y) => {
                let mut x : u8 = x.clone();
                if x > 8 { // We skip I on the goban for readability
                    x = x + 1;
                }
                write!(f, "{}{}", ((x+64) as char).to_uppercase(), y)
            },
            Self::Pass => write!(f, "PASS"),
            Self::Resign => write!(f, "RESIGN"),
        }
    }
}

impl From<Vertex> for String {
    fn from(vertex: Vertex) -> Self {
        format!("{}", vertex)
    }
}

impl core::cmp::PartialEq<&str> for Vertex {
    fn eq(&self, rhs: &&str) -> bool {
        format!("{}", self) == *rhs.to_uppercase()
    }
}

impl core::cmp::PartialEq<String> for Vertex {
    fn eq(&self, rhs: &String) -> bool {
        format!("{}", self) == *rhs.to_uppercase()
    }
}

impl Vertex {
    pub fn to_tupple(&self) -> Option<(u8, u8)> {
        match self {
            Self::Coord(x, y) => Some((*x, *y)),
            Self::Pass => None,
            Self::Resign => None,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Black,
    White,
}

impl Entity for Color {}

impl Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Black => write!(f, "B"),
            Self::White => write!(f, "W"),
        }
    }
}

impl FromStr for Color {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str.is_empty() {
            return Err(Self::Err::EmptyString);
        }
        let str = str.to_uppercase();
        match &*str {
            "B" | "BLACK" => Ok(Self::Black),
            "W" | "WHITE" => Ok(Self::White),
            _ => Err(ParseError::WrongColor),
        }
    }
}

impl From<Color> for String {
    fn from(col: Color) -> Self {
        format!("{}", col)
    }
}

impl core::cmp::PartialEq<&str> for Color {
    fn eq(&self, rhs: &&str) -> bool {
        format!("{}", self) == *rhs.to_uppercase()
    }
}

impl core::cmp::PartialEq<String> for Color {
    fn eq(&self, rhs: &String) -> bool {
        format!("{}", self) == *rhs.to_uppercase()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub color: Color,
    pub vertex: Vertex,
}

impl Entity for Move {}

impl Display for Move {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{} {}", self.color, self.vertex)
    }
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str.is_empty() {
            return Err(ParseError::EmptyString);
        }
        let str = str.to_uppercase();
        let mut str = str.split_ascii_whitespace();
        Ok(Move {
            color: str.next().unwrap().parse()?,
            vertex: str.next().unwrap().parse()?,
        })
    }
}

impl From<Move> for String {
    fn from(mov: Move) -> Self {
        format!("{}", mov)
    }
}

impl core::cmp::PartialEq<&str> for Move {
    fn eq(&self, rhs: &&str) -> bool {
        format!("{}", self) == *rhs.to_uppercase()
    }
}

impl core::cmp::PartialEq<String> for Move {
    fn eq(&self, rhs: &String) -> bool {
        format!("{}", self) == *rhs.to_uppercase()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Hash)]
pub enum SimpleEntity {
    Vertex(Vertex),
    Color(Color),
    Move(Move),
}

impl alloc::fmt::Display for SimpleEntity {
    fn fmt(&self, f: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
        match self {
            SimpleEntity::Vertex(v) => write!(f, "{}", v),
            SimpleEntity::Color(c) => write!(f, "{}", c),
            SimpleEntity::Move(m) => write!(f, "{}", m),
        }
    }
}

impl FromStr for SimpleEntity {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<SimpleEntity, Self::Err> {
        if let Ok(v) = str.parse::<Vertex>() {
            Ok(SimpleEntity::Vertex(v))
        } else if let Ok(c) = str.parse::<Color>() {
            Ok(SimpleEntity::Color(c))
        } else if let Ok(m) = str.parse::<Move>() {
            Ok(SimpleEntity::Move(m))
        } else {
            Err(ParseError::WrongSimpleEntity)
        }
    }
}

impl From<Vertex> for SimpleEntity {
    fn from(v: Vertex) -> Self {
        SimpleEntity::Vertex(v)
    }
}

impl From<Color> for SimpleEntity {
    fn from(c: Color) -> Self {
        SimpleEntity::Color(c)
    }
}

impl From<Move> for SimpleEntity {
    fn from(m: Move) -> Self {
        SimpleEntity::Move(m)
    }
}

impl SimpleEntity {
    pub fn as_vertex(self) -> Option<Vertex> {
        match self {
            Self::Vertex(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_color(self) -> Option<Color> {
        match self {
            Self::Color(c) => Some(c),
            _ => None,
        }
    }

    pub fn as_move(self) -> Option<Move> {
        match self {
            Self::Move(m) => Some(m),
            _ => None,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash)]
pub struct Collection(Vec<SimpleEntity>);

impl Entity for Collection {}

impl Display for Collection {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if !self.0.is_empty() {
            write!(f, "{}", self.0[0])?;
            if self.0.len() > 1 {
                for entity in &self.0[1..] {
                    write!(f, " {}", entity)?;
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }
}

impl FromStr for Collection {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let elems : Vec<SimpleEntity> = str.to_uppercase().split_ascii_whitespace()
            .map(|e| if let Ok(elem) = e.parse::<SimpleEntity>() {
                elem
            } else {
                panic!()
            }).collect();

        Ok(Self(elems))
    }
}

impl Index<usize> for Collection {
    type Output = SimpleEntity;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for Collection {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Self::Output {
        &mut self.0[i]
    }
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            0: Vec::new(),
        }
    }
}

impl From<Vec<SimpleEntity>> for Collection {
    fn from(vec: Vec<SimpleEntity>) -> Self {
        Self(vec)
    }
}

impl FromIterator<SimpleEntity> for Collection {
    fn from_iter<I: IntoIterator<Item=SimpleEntity>>(it: I) -> Self {
        Self(it.into_iter().collect())
    }
}

impl IntoIterator for Collection {
    type Item = SimpleEntity;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Collection {
    type Item = SimpleEntity;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.to_owned().into_iter()
    }
}

impl<'a> IntoIterator for &'a mut Collection {
    type Item = SimpleEntity;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.to_owned().into_iter()
    }
}

impl Collection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_vec(vec: Vec<SimpleEntity>) -> Self {
        Self(vec)
    }

    pub fn push(&mut self, elem: SimpleEntity) {
        self.0.push(elem);
    }

    pub fn remove(&mut self, index: usize) -> SimpleEntity {
        self.0.remove(index)
    }

    pub fn into_vec(self) -> Vec<SimpleEntity> {
        self.0
    }

    pub fn inner(&self) -> &Vec<SimpleEntity> {
        &self.0
    }

    pub fn mut_inner(&mut self) -> &mut Vec<SimpleEntity> {
        &mut self.0
    }

    pub fn iter(&self) -> core::slice::Iter<'_, SimpleEntity> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, SimpleEntity> {
        self.0.iter_mut()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash)]
pub struct List<T : Entity>(Vec<T>);

impl<T: Entity> Entity for List<T> {}

impl<T: Entity> Display for List<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if !self.0.is_empty() {
            write!(f, "{}", self.0[0])?;
            if self.0.len() > 1 {
                for entity in &self.0[1..] {
                    write!(f, " {}", entity)?;
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }
}

impl<T: Entity> FromStr for List<T> {
    type Err = ParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let check_line_return: Vec<&str> = str.split('\n').collect();
        if check_line_return.len() > 2 {
            return Err(Self::Err::WrongArgs);
        }

        let elems : Vec<T> = str.to_uppercase().split_ascii_whitespace()
            .take_while(|e| e.clone() != "".to_string())
            .map(|e| if let Ok(elem) = e.parse::<T>() {
                elem
            } else {
                panic!()
            }).collect();

        Ok(Self(elems))
    }
}

impl<T: Entity> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<T: Entity> IndexMut<usize> for List<T> {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Self::Output {
        &mut self.0[i]
    }
}

impl<T: Entity> Default for List<T> {
    fn default() -> Self {
        Self {
            0: Vec::new(),
        }
    }
}

impl<T: Entity> From<Vec<T>> for List<T> {
    fn from(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

impl<T: Entity> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item=T>>(it: I) -> Self {
        Self(it.into_iter().collect())
    }
}

impl<T: Entity> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T: Entity> IntoIterator for &'a List<T> {
    type Item = T;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.to_owned().into_iter()
    }
}

impl<'a, T: Entity> IntoIterator for &'a mut List<T> {
    type Item = T;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.to_owned().into_iter()
    }
}

impl<T: Entity> List<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        Self(vec)
    }

    pub fn push(&mut self, elem: T) {
        self.0.push(elem);
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.0.remove(index)
    }

    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    pub fn inner(&self) -> &Vec<T> {
        &self.0
    }

    pub fn mut_inner(&mut self) -> &mut Vec<T> {
        &mut self.0
    }

    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.0.iter_mut()
    }
}
