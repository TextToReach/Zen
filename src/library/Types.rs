use super::Array;
use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
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
    Array(Array::Array),
    Bool(Bool),
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
    fn new_enum(value: T) -> ZenType;
    fn new(value: T) -> Self;
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
pub struct Bool {
    pub value: bool,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<ZenNamedParameter>,
    // TODO: After adding zenvm functionality, complete this part.
}

// ------------------------------------------ Implements ------------------------------------------

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

impl FromStr for Number {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<f64>().map(|v| Number { value: v })
    }
}

impl New<f64> for Number {
    fn new_enum(value: f64) -> ZenType {
        ZenType::Number(Self { value })
    }

    fn new(value: f64) -> Self {
        Self { value }
    }
}
impl New<String> for Text {
    fn new_enum(value: String) -> ZenType {
        ZenType::Text(Self { value })
    }

    fn new(value: String) -> Self {
        Self { value }
    }
}

impl New<bool> for Bool {
    fn new_enum(value: bool) -> ZenType {
        ZenType::Bool(Self { value })
    }

    fn new(value: bool) -> Self {
        Self { value }
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

impl Display for Bool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
