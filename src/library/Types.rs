#![allow(non_snake_case, dead_code)]

use chumsky::prelude::*;
use chumsky::text::whitespace;
use chumsky::{Parser, error::Simple};

use crate::library::Methods::Throw;

use super::Array::Array;
use super::Environment::Environment;
use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::rc::Rc;
use std::{fmt::Display, num::ParseFloatError, str::FromStr};

/// Exact eq's:
/// - Number = f64
/// - Text = String
/// - Array = Vec<Object>

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Number(Number),
    Text(Text),
    Array(Array),
    Bool(Boolean),
    Variable(String),
    ArithmeticExpression(Arithmetic),
    Nil,
}

impl Object {
    pub fn asNumber(self) -> Number {
        if let Object::Number(val) = self {
            val
        } else {
            panic!("Error while trying to convert Object to Number: Object is not a number.")
        }
    }

    pub fn asText(self) -> Text {
        if let Object::Text(val) = self {
            val
        } else {
            panic!("Error while trying to convert Object to Text: Object is not a text.")
        }
    }

    pub fn asArray(self) -> Array {
        if let Object::Array(val) = self {
            val
        } else {
            panic!("Error while trying to convert Object to Array: Object is not an array.")
        }
    }

    pub fn asBool(self) -> Boolean {
        if let Object::Bool(val) = self {
            val
        } else {
            panic!("Error while trying to convert Object to Bool: Object is not a bool.")
        }
    }

    pub fn asVariable(self) -> String {
        if let Object::Variable(val) = self {
            val
        } else {
            panic!("Error while trying to convert Object to Variable: Object is not a variable.")
        }
    }

    pub fn asArithmetic(self) -> Arithmetic {
        if let Object::ArithmeticExpression(val) = self {
            val
        } else {
            panic!(
                "Error while trying to convert Object to Arithmetic: Object is not an arithmetic expression."
            )
        }
    }
}

impl Add for Object {
    type Output = Object;

    fn add(self, other: Object) -> Self::Output {
        match (&self, &other) {
            (Object::Number(a), Object::Number(b)) => {
                Object::Number(Number::from(a.value + b.value))
            }
            (Object::Number(n), Object::ArithmeticExpression(x)) | (Object::ArithmeticExpression(x), Object::Number(n)) => {
                if let Arithmetic::Value(val) = x.clone() {
                    if let Object::Number(val) = *val {
                        Object::from(n.value + val.value)
                    } else {
                        panic!("Error while trying to add two objects: Objects are not compatible.")
                    }
                } else {
                    panic!("Error while trying to add two objects: Objects are not compatible.")
                    
                }
            }
            (Object::Text(a), Object::Text(b)) => Object::Text(Text::from(a.value.clone() + &b.value)),
            _ => panic!("Error while trying to add two objects: Objects are not compatible. ({:?} and {:?})", self, other),
        }
    }
}

impl Sub for Object {
    type Output = Object;

    fn sub(self, other: Object) -> Self::Output {
        match (&self, &other) {
            (Object::Number(a), Object::Number(b)) => {
                Object::Number(Number::from(a.value - b.value))
            }
            (Object::Number(n), Object::ArithmeticExpression(x)) | (Object::ArithmeticExpression(x), Object::Number(n)) => {
                if let Arithmetic::Value(val) = x.clone() {
                    if let Object::Number(val) = *val {
                        Object::from(n.value - val.value)
                    } else {
                        panic!("Error while trying to subtract two objects: Objects are not compatible.")
                    }
                } else {
                    panic!("Error while trying to subtract two objects: Objects are not compatible.")
                    
                }
            }
            _ => panic!("Error while trying to subtract two objects: Objects are not compatible. ({:?} and {:?})", self, other),
        }
    }
}

impl Mul for Object {
    type Output = Object;

    fn mul(self, other: Object) -> Self::Output {
        match (&self, &other) {
            (Object::Number(a), Object::Number(b)) => {
                Object::Number(Number::from(a.value * b.value))
            }
            (Object::Number(n), Object::ArithmeticExpression(x)) | (Object::ArithmeticExpression(x), Object::Number(n)) => {
                if let Arithmetic::Value(val) = x.clone() {
                    if let Object::Number(val) = *val {
                        Object::from(n.value * val.value)
                    } else {
                        panic!("Error while trying to multiply two objects: Objects are not compatible.")
                    }
                } else {
                    panic!("Error while trying to multiply two objects: Objects are not compatible.")
                    
                }
            }
            (Object::Text(a), Object::Number(b)) | (Object::Number(b), Object::Text(a)) => {
                Object::Text(Text::from(a.value.repeat(b.value as usize)))
            }
            _ => panic!("Error while trying to multiply two objects: Objects are not compatible. ({:?} and {:?})", self, other),
        }
    }
}

impl Div for Object {
    type Output = Object;

    fn div(self, other: Object) -> Self::Output {
        match (&self, &other) {
            (Object::Number(a), Object::Number(b)) => {
                Object::Number(Number::from(a.value / b.value))
            }
            (Object::Number(n), Object::ArithmeticExpression(x)) | (Object::ArithmeticExpression(x), Object::Number(n)) => {
                if let Arithmetic::Value(val) = x.clone() {
                    if let Object::Number(val) = *val {
                        Object::from(n.value / val.value)
                    } else {
                        panic!("Error while trying to subtract two objects: Objects are not compatible.")
                    }
                } else {
                    panic!("Error while trying to subtract two objects: Objects are not compatible.")
                    
                }
            }
            _ => panic!("Error while trying to subtract two objects: Objects are not compatible. ({:?} and {:?})", self, other),
        }
    }
}

impl Rem for Object {
    type Output = Object;

    fn rem(self, other: Object) -> Self::Output {
        match (self, other) {
            (Object::Number(a), Object::Number(b)) => {
                if b.value == 0.0 {
                    panic!("Error while trying to divide two objects: Division by zero.");
                }
                Object::Number(Number::from(a.value % b.value))
            }
            _ => panic!("Error while trying to divide two objects: Objects are not compatible."),
        }
    }
}
/// Fill fields on demand. No need to fill al fields. See: forloop1.rs
#[derive(Debug, Clone, PartialEq)]
pub struct Instruction(pub InstructionEnum);

#[derive(Debug, Clone)]
pub struct ZenNamedParameter {
    name: String,
    value: Object,
}

#[derive(Debug, Clone)]
pub enum ZenError {
    UnknownError,
    GeneralError,
    NotDeclaredError,
    DivisionByZeroError,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionEnum {
    NoOp,
    Yazdır(Vec<Object>),
    Forloop1(i64, Vec<Instruction>),
    VariableDeclaration(String, Object),
}

pub mod Operator {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Arithmetic {
        Plus,
        Minus,
        Multiply,
        Divide,
        Mod,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arithmetic {
    Value(Box<Object>),
    Add(Box<Arithmetic>, Box<Arithmetic>),
    Sub(Box<Arithmetic>, Box<Arithmetic>),
    Mul(Box<Arithmetic>, Box<Arithmetic>),
    Div(Box<Arithmetic>, Box<Arithmetic>),
    Mod(Box<Arithmetic>, Box<Arithmetic>),
}

impl Arithmetic {
    pub fn parser<'a>(currentScope: Rc<RefCell<Environment>>) -> Box<dyn Parser<char, Object, Error = Simple<char>> + 'a> {
        // Arithmetic expression parser with operator precedence and parentheses
        let currentScope_clone = currentScope.clone();
        Box::new(
            recursive(move |arith| {
                let currentScope = currentScope_clone.clone();
                // Parse a value: number, variable, or parenthesized arithmetic
                let value = choice((
                    Number::parser().map(|obj| Arithmetic::Value(Box::new(obj))),
                    Variable::parser().map(|obj| Arithmetic::Value(Box::new(obj))),
                    arith.clone().delimited_by(just('('), just(')')),
                ))
                .boxed();

                // Operator precedence: *, /, % > +, -
                let op_mul = just('*')
                    .to(Arithmetic::Mul as fn(_, _) -> _)
                    .or(just('/').to(Arithmetic::Div as fn(_, _) -> _))
                    .or(just('%').to(Arithmetic::Mod as fn(_, _) -> _))
                    .padded_by(whitespace());
                let op_add = just('+')
                    .to(Arithmetic::Add as fn(_, _) -> _)
                    .or(just('-').to(Arithmetic::Sub as fn(_, _) -> _))
                    .padded_by(whitespace());

                // Multiplicative: value (('*' | '/' | '%') value)*
                let mul = value
                    .clone()
                    .then(op_mul.then(value.clone()).repeated())
                    .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

                // Additive: mul (('+' | '-') mul)*
                let add = mul
                    .clone()
                    .then(op_add.then(mul.clone()).repeated())
                    .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

                add
            })
            .map(|e| Object::from(e)),
        )
    }
}

// ------------------------------------------ Traits ------------------------------------------

pub trait New<T> {
    /// Converts value T to corresponding ZenType.
    /// Has the exact same purpose as ZenType::from
    /// ZenType::from -> ZenType (T)
    /// T::enum_from -> ZenType (T)
    fn enum_from(value: T) -> Object;
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
pub struct Variable {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<ZenNamedParameter>,
    // TODO: After adding zenvm functionality, complete this part.
}

// ------------------------------------------ Parser Implements ------------------------------------------

impl<'a> Parsable<'a, char, Object, Simple<char>> for Number {
    fn parser() -> Box<dyn Parser<char, Object, Error = Simple<char>> + 'a> {
        let out = just("-")
            .or_not()
            .then(text::int::<_, Simple<char>>(10))
            .then(just('.').ignore_then(text::digits(10)).or_not())
            .map(|((negative, int), frac)| {
                Object::from(
                    format!(
                        "{}{}.{}",
                        negative.unwrap_or("+"),
                        int,
                        frac.unwrap_or("0".to_owned())
                    )
                    .parse::<f64>()
                    .unwrap(),
                )
            });

        Box::new(recursive(|prev| {
            prev.clone().delimited_by(just("("), just(")")).or(out)
        }))
    }
}

impl<'a> Parsable<'a, char, Object, Simple<char>> for Text {
    fn parser() -> Box<dyn Parser<char, Object, Error = Simple<char>> + 'a> {
        let single_quoted = just('\'') // Tek tırnakla başla
            .ignore_then(filter(|c| *c != '\'').repeated()) // Tek tırnak bitene kadar karakterleri al
            .then_ignore(just('\'')) // Tek tırnakla bitir
            .collect::<String>(); // Karakterleri string'e çevir

        let double_quoted = just('"') // Çift tırnakla başla
            .ignore_then(filter(|c| *c != '"').repeated()) // Çift tırnak bitene kadar karakterleri al
            .then_ignore(just('"')) // Çift tırnakla bitir
            .collect::<String>(); // Karakterleri string'e çevir

        let out = single_quoted.or(double_quoted).map(Object::from);

        Box::new(recursive(|prev| {
            prev.clone().delimited_by(just("("), just(")")).or(out)
        }))
    }
}

impl<'a> Parsable<'a, char, Object, Simple<char>> for Boolean {
    fn parser() -> Box<dyn Parser<char, Object, Error = Simple<char>> + 'a> {
        let out = choice([just("true"), just("doğru")])
            .to(Object::from(true))
            .or(choice([just("false"), just("yanlış")]).to(Object::from(false)));

        Box::new(recursive(|prev| {
            prev.clone().delimited_by(just("("), just(")")).or(out)
        }))
    }
}

impl<'a> Parsable<'a, char, Object, Simple<char>> for Variable {
    fn parser() -> Box<dyn Parser<char, Object, Error = Simple<char>> + 'a> {
        let out = text::ident();

        Box::new(
            recursive(|prev| prev.clone().delimited_by(just("("), just(")")).or(out))
                .map(Object::Variable),
        )
    }
}

impl Object {
    pub fn parser<'a>(currentScope: Rc<RefCell<Environment>>) -> Box<dyn Parser<char, Object, Error = Simple<char>> + 'a> {
        Box::new(choice([
            Arithmetic::parser(currentScope),
            Number::parser(),
            Text::parser(),
            Boolean::parser(),
        ]))
    }
}

// ------------------------------------------ Trait Implements ------------------------------------------

impl From<f64> for Object {
    fn from(value: f64) -> Self {
        Object::Number(Number::from(value))
    }
}

impl From<String> for Object {
    fn from(value: String) -> Self {
        Object::Text(Text::from(value))
    }
}

impl From<&str> for Object {
    fn from(value: &str) -> Self {
        Object::Text(Text::from(value.to_owned()))
    }
}

impl From<bool> for Object {
    fn from(value: bool) -> Self {
        Object::Bool(Boolean::from(value))
    }
}

impl From<Vec<Object>> for Object {
    fn from(value: Vec<Object>) -> Self {
        Object::Array(Array::from(value))
    }
}

impl From<Arithmetic> for Object {
    fn from(value: Arithmetic) -> Self {
        Object::ArithmeticExpression(value)
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
    fn enum_from(value: f64) -> Object {
        Object::Number(Self { value })
    }

    fn new() -> Self {
        Self { value: 0f64 }
    }
}
impl New<String> for Text {
    fn enum_from(value: String) -> Object {
        Object::Text(Self { value })
    }

    fn new() -> Self {
        Self {
            value: "".to_owned(),
        }
    }
}

impl New<bool> for Boolean {
    fn enum_from(value: bool) -> Object {
        Object::Bool(Self { value })
    }

    fn new() -> Self {
        Self { value: false }
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

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Array(val) => write!(f, "{:?}", val),
            Object::Bool(val) => write!(f, "{}", val),
            Object::Number(val) => write!(f, "{}", val),
            Object::Text(val) => write!(f, "{}", val),
            Object::Variable(val) => write!(f, "{}", val),
            Object::Nil => write!(f, "NIL"),
            Object::ArithmeticExpression(val) => write!(f, "{:?}", val),
        }
    }
}
