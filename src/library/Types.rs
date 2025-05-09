#![allow(non_snake_case, dead_code)]

use chumsky::prelude::*;
use chumsky::text::whitespace;
use chumsky::{Parser, error::Simple};
use colored::Colorize;
use num::pow::Pow;

use crate::features::tokenizer::TokenTable;
use crate::library::Methods::Throw;
use crate::parsers::Parsers::Expression;
use crate::util::ScopeManager::ScopeManager;

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
	Bool(Boolean),
	Variable(String),
	Expression(Expression),
	Token(TokenTable),
	Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BaseTypes {
	Number,
	Text,
	Array,
	Bool,
}

impl BaseTypes {
	pub const VALUES: [&str; 4] = ["yazı", "sayı", "liste", "mantıksal"];

	pub const POSSIBLEBOOLEANVALUES: [[&str; 4]; 2] = [["doğru", "evet", "yes", "true"], ["yanlış", "hayır", "no", "false"]];

	pub fn from_str(s: &str) -> Self {
		match s {
			val if val == Self::VALUES[0] => BaseTypes::Text,
			val if val == Self::VALUES[1] => BaseTypes::Number,
			val if val == Self::VALUES[2] => BaseTypes::Array,
			val if val == Self::VALUES[3] => BaseTypes::Bool,
			_ => panic!("Error while trying to convert string to BaseType: Unknown type."),
		}
	}
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

	pub fn asExpression(self) -> Expression {
		if let Object::Expression(val) = self {
			val
		} else {
			panic!("Error while trying to convert Object to Expression: Object is not an expression expression.")
		}
	}

	pub fn isTruthy(&self) -> bool {
		match self {
			Object::Bool(val) => val.value,
			Object::Number(val) => val.value != 0.0,
			Object::Text(val) => !val.value.is_empty(),
			Object::Variable(_) => true,
			Object::Null => false,
			Object::Expression(_) => true,
			Object::Token(_) => true,
		}
	}
}

impl Add for Object {
	type Output = Object;

	fn add(self, other: Object) -> Self::Output {
		match (&self, &other) {
			(Object::Number(a), Object::Number(b)) => Object::Number(Number::from(a.value + b.value)),
			(Object::Number(n), Object::Expression(x)) | (Object::Expression(x), Object::Number(n)) => {
				if let Expression::Value(val) = x.clone() {
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
			_ => panic!(
				"Error while trying to add two objects: Objects are not compatible. ({:?} and {:?})",
				self, other
			),
		}
	}
}

impl Sub for Object {
	type Output = Object;

	fn sub(self, other: Object) -> Self::Output {
		match (&self, &other) {
			(Object::Number(a), Object::Number(b)) => Object::Number(Number::from(a.value - b.value)),
			(Object::Number(n), Object::Expression(x)) | (Object::Expression(x), Object::Number(n)) => {
				if let Expression::Value(val) = x.clone() {
					if let Object::Number(val) = *val {
						Object::from(n.value - val.value)
					} else {
						panic!("Error while trying to subtract two objects: Objects are not compatible.")
					}
				} else {
					panic!("Error while trying to subtract two objects: Objects are not compatible.")
				}
			}
			_ => panic!(
				"Error while trying to subtract two objects: Objects are not compatible. ({:?} and {:?})",
				self, other
			),
		}
	}
}

impl Mul for Object {
	type Output = Object;

	fn mul(self, other: Object) -> Self::Output {
		match (&self, &other) {
			(Object::Number(a), Object::Number(b)) => Object::Number(Number::from(a.value * b.value)),
			(Object::Number(n), Object::Expression(x)) | (Object::Expression(x), Object::Number(n)) => {
				if let Expression::Value(val) = x.clone() {
					if let Object::Number(val) = *val {
						Object::from(n.value * val.value)
					} else {
						panic!("Error while trying to multiply two objects: Objects are not compatible.")
					}
				} else {
					panic!("Error while trying to multiply two objects: Objects are not compatible.")
				}
			}
			(Object::Text(a), Object::Number(b)) | (Object::Number(b), Object::Text(a)) => Object::Text(Text::from(a.value.repeat(b.value as usize))),
			_ => panic!(
				"Error while trying to multiply two objects: Objects are not compatible. ({:?} and {:?})",
				self, other
			),
		}
	}
}

impl Div for Object {
	type Output = Object;

	fn div(self, other: Object) -> Self::Output {
		match (&self, &other) {
			(Object::Number(a), Object::Number(b)) => Object::Number(Number::from(a.value / b.value)),
			(Object::Number(n), Object::Expression(x)) | (Object::Expression(x), Object::Number(n)) => {
				if let Expression::Value(val) = x.clone() {
					if let Object::Number(val) = *val {
						Object::from(n.value / val.value)
					} else {
						panic!("Error while trying to subtract two objects: Objects are not compatible.")
					}
				} else {
					panic!("Error while trying to subtract two objects: Objects are not compatible.")
				}
			}
			_ => panic!(
				"Error while trying to subtract two objects: Objects are not compatible. ({:?} and {:?})",
				self, other
			),
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

impl Pow<Object> for Object {
	type Output = Object;

	fn pow(self, other: Object) -> Self::Output {
		match (self, other) {
			(Object::Number(a), Object::Number(b)) => Object::Number(Number::from(a.value.pow(b.value))),
			_ => panic!("Error while trying to raise an object to another object's power: Objects are not compatible."),
		}
	}
}

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
	TypeError,
	IndentationError,
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

	#[derive(Debug, Clone, PartialEq)]
	pub enum Comparison {
		Equal,
		NotEqual,
		GreaterThan,
		LessThan,
		GreaterThanOrEqual,
		LessThanOrEqual,
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
	Low = 1,
	Medium = 2,
	High = 3,
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
	fn parser(currentScope: Rc<RefCell<usize>>, manager: &mut ScopeManager) -> Box<dyn Parser<I, O, Error = E> + 'a>;
}

// ------------------------------------------ Structs ------------------------------------------

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Number {
	pub value: f64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Text {
	pub value: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Boolean {
	pub value: bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Variable {
	pub value: String,
}

#[derive(Debug, Clone)]
pub struct Function {
	pub parameters: Vec<ZenNamedParameter>,
	// TODO: After adding zenvm functionality, complete this part.
}

// ------------------------------------------ Parser Implements ------------------------------------------

/* impl<'a> Parsable<'a, char, Object, Simple<char>> for Number {
	fn parser(currentScope: Rc<RefCell<usize>>, manager: &mut ScopeManager) -> Box<dyn Parser<char, Object, Error = Simple<char>> + 'a> {
		
	}
} */

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
		Self { value: "".to_owned() }
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
		let temp = format!("{}", self.value);
		write!(f, "{}", temp.yellow())
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
			Object::Bool(val) => write!(f, "{}", val),
			Object::Number(val) => write!(f, "{}", val),
			Object::Text(val) => write!(f, "{}", val),
			Object::Variable(val) => write!(f, "{}", val),
			Object::Null => write!(f, "NIL"),
			Object::Expression(val) => write!(f, "{:?}", val),
			Object::Token(val) => write!(f, "{:?}", val),
		}
	}
}