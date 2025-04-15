#![allow(non_snake_case, dead_code)]

use chumsky::{error::Simple, Parser};
use chumsky::prelude::*;

use super::Array::Array;
use std::{
    fmt::Display,
    num::ParseFloatError,
    str::FromStr,
};

/// Exact eq's:
/// - Number = f64
/// - Text = String
/// - Array = Vec<_>

#[derive(Debug, Clone, PartialEq)]
pub enum ZenType {
    Number(Number),
    Text(Text),
    Array(Array),
    Bool(Boolean),
}

#[derive(Debug, Clone)]
pub struct ZenNamedParameter {
    name: String,
    value: ZenType,
}

#[derive(Debug, Clone)]
pub enum ZenError {
    UnknownError,
    GeneralError,
    NotDeclaredError,
}

// ------------------------------------------ Traits ------------------------------------------

pub trait New<T> {
    /// Converts value T to corresponding ZenType.
    /// Has the exact same purpose as ZenType::from
    /// ZenType::from -> ZenType (T)
    /// T::enum_from -> ZenType (T)
    fn enum_from(value: T) -> ZenType;
    fn new() -> Self;
}

pub trait Parsable<'a, I, O, E>
where
    I: 'a + Clone,
    E: chumsky::error::Error<I> + 'a,
{
    fn parser() -> Box<dyn Parser<I, O, Error = E> + 'a>;
}

// ------------------------------------------ Structs ------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Number {
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {

}

impl Object {

}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<ZenNamedParameter>,
    // TODO: After adding zenvm functionality, complete this part.
}

// ------------------------------------------ Parser Implements ------------------------------------------

impl<'a> Parsable<'a, char, ZenType, Simple<char>> for Number {
   fn parser() -> Box<dyn Parser<char, ZenType, Error = Simple<char>> + 'a> {
        Box::new(
            just("-")
                .or_not()
                .then(text::int::<_, Simple<char>>(10))
                .then(just('.').ignore_then(text::digits(10)).or_not())
                .map(|((negative, int), frac)| {
                    ZenType::from(format!("{}{}.{}", negative.unwrap_or("+"), int, frac.unwrap_or("0".to_owned())).parse::<f64>().unwrap())
                })
                .padded()
        )
    }
}

impl<'a> Parsable<'a, char, ZenType, Simple<char>> for Text {
   fn parser() -> Box<dyn Parser<char, ZenType, Error = Simple<char>> + 'a> {
        let single_quoted = just('\'') // Tek tırnakla başla
            .ignore_then(filter(|c| *c != '\'').repeated()) // Tek tırnak bitene kadar karakterleri al
            .then_ignore(just('\'')) // Tek tırnakla bitir
            .collect::<String>(); // Karakterleri string'e çevir

        let double_quoted = just('"') // Çift tırnakla başla
            .ignore_then(filter(|c| *c != '"').repeated()) // Çift tırnak bitene kadar karakterleri al
            .then_ignore(just('"')) // Çift tırnakla bitir
            .collect::<String>(); // Karakterleri string'e çevir

        Box::new(single_quoted.or(double_quoted).map(ZenType::from))
    }
}

impl<'a> Parsable<'a, char, ZenType, Simple<char>> for Boolean {
   fn parser() -> Box<dyn Parser<char, ZenType, Error = Simple<char>> + 'a> {
        Box::new(
            just("true")
                .to(ZenType::from(true))
                .or(just("false").to(ZenType::from(false)))

        )
    }
}

// ------------------------------------------ Trait Implements ------------------------------------------

impl From<f64> for ZenType {
    fn from(value: f64) -> Self {
        ZenType::Number(Number::from(value)) 
    }
}

impl From<String> for ZenType {
    fn from(value: String) -> Self {
        ZenType::Text(Text::from(value)) 
    }
}

impl From<&str> for ZenType {
    fn from(value: &str) -> Self {
        ZenType::Text(Text::from(value.to_owned())) 
    }
}

impl From<bool> for ZenType {
    fn from(value: bool) -> Self {
        ZenType::Bool(Boolean::from(value)) 
    }
}

impl From<Vec<ZenType>> for ZenType {
    fn from(value: Vec<ZenType>) -> Self {
        ZenType::Array(Array::from(value)) 
    }
}


impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self { value }
    }
}
impl From<String> for Text {
    fn from(value: String) -> Self {
        Self { value }
    }
}
impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Self { value }
    }
}

impl Into<f64> for Number {
    fn into(self) -> f64 {
        self.value
    }
}
impl Into<String> for Text {
    fn into(self) -> String {
        self.value
    }
}
impl Into<bool> for Boolean {
    fn into(self) -> bool {
        self.value
    }
}

impl FromStr for Number {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<f64>().map(|v| Number { value: v })
    }
}

impl New<f64> for Number {
    fn enum_from(value: f64) -> ZenType {
        ZenType::Number(Self { value })
    }

    fn new() -> Self {
        Self { value: 0f64 }
    }
}
impl New<String> for Text {
    fn enum_from(value: String) -> ZenType {
        ZenType::Text(Self { value })
    }

    fn new() -> Self {
        Self { value: "".to_owned() }
    }
}

impl New<bool> for Boolean {
    fn enum_from(value: bool) -> ZenType {
        ZenType::Bool(Self { value })
    }

    fn new() -> Self {
        Self { value: false }
    }
}

impl Display for ZenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
