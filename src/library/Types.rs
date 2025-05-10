#![allow(non_snake_case, dead_code)]

use chumsky::chain::Chain;
use chumsky::prelude::*;
use chumsky::text::whitespace;
use chumsky::{Parser, error::Simple};
use colored::Colorize;
use num::pow::Pow;

use crate::features::tokenizer::{TokenData, TokenTable};
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
pub enum ObjectComparison {
	BothNumber(Number, Number),
	BothText(Text, Text),
	BothBoolean(Boolean, Boolean),
	BothVariable(String, String),
	NumberAndText(Number, Text),
	NumberAndBoolean(Number, Boolean),
	NumberAndVariable(Number, String),
	TextAndNumber(Text, Number),
	TextAndBoolean(Text, Boolean),
	TextAndVariable(Text, String),
	BooleanAndNumber(Boolean, Number),
	BooleanAndText(Boolean, Text),
	BooleanAndVariable(Boolean, String),
	VariableAndNumber(String, Number),
	VariableAndText(String, Text),
	VariableAndBoolean(String, Boolean),
}

impl From<(&Object, &Object)> for ObjectComparison {
	fn from(value: (&Object, &Object)) -> Self {
		match value {
			(Object::Number(a), Object::Number(b)) => ObjectComparison::BothNumber(a.clone(), b.clone()),
			(Object::Text(a), Object::Text(b)) => ObjectComparison::BothText(a.clone(), b.clone()),
			(Object::Bool(a), Object::Bool(b)) => ObjectComparison::BothBoolean(a.clone(), b.clone()),
			(Object::Variable(a), Object::Variable(b)) => ObjectComparison::BothVariable(a.clone(), b.clone()),
			(Object::Number(a), Object::Text(b)) => ObjectComparison::NumberAndText(a.clone(), b.clone()),
			(Object::Number(a), Object::Bool(b)) => ObjectComparison::NumberAndBoolean(a.clone(), b.clone()),
			(Object::Number(a), Object::Variable(b)) => ObjectComparison::NumberAndVariable(a.clone(), b.clone()),
			(Object::Text(a), Object::Number(b)) => ObjectComparison::TextAndNumber(a.clone(), b.clone()),
			(Object::Text(a), Object::Bool(b)) => ObjectComparison::TextAndBoolean(a.clone(), b.clone()),
			(Object::Text(a), Object::Variable(b)) => ObjectComparison::TextAndVariable(a.clone(), b.clone()),
			(Object::Bool(a), Object::Number(b)) => ObjectComparison::BooleanAndNumber(a.clone(), b.clone()),
			(Object::Bool(a), Object::Text(b)) => ObjectComparison::BooleanAndText(a.clone(), b.clone()),
			(Object::Bool(a), Object::Variable(b)) => ObjectComparison::BooleanAndVariable(a.clone(), b.clone()),
			(Object::Variable(a), Object::Number(b)) => ObjectComparison::VariableAndNumber(a.clone(), b.clone()),
			(Object::Variable(a), Object::Text(b)) => ObjectComparison::VariableAndText(a.clone(), b.clone()),
			(Object::Variable(a), Object::Bool(b)) => ObjectComparison::VariableAndBoolean(a.clone(), b.clone()),
			_ => panic!("Unsupported Object combination for comparison: ({:?}, {:?})", value.0, value.1),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
	Number(Number),
	Text(Text),
	Bool(Boolean),
	Variable(String),
	Null,
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

	pub fn isTruthy(&self) -> bool {
		match self {
			Object::Bool(val) => val.value,
			Object::Number(val) => val.value != 0.0,
			Object::Text(val) => !val.value.is_empty(),
			Object::Variable(_) => true,
			Object::Null => false,
		}
	}
}

impl Add for Object {
	type Output = Object;

	fn add(self, other: Object) -> Self::Output {
		match (&self, &other) {
			(Object::Number(a), Object::Number(b)) => Object::Number(Number::from(a.value + b.value)),
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

impl PartialOrd for Object {
	fn gt(&self, other: &Self) -> bool {
		match ObjectComparison::from((self, other)) {
			ObjectComparison::BothNumber(x, y)         => x > y,
			ObjectComparison::BothText(x, y)           => x.value.len() > y.value.len(),
			ObjectComparison::BothBoolean(x, y)        => x.value & !y.value,
			ObjectComparison::BothVariable(x, y)       => x > y,
			ObjectComparison::NumberAndText(x, y)      => x.value > y.value.len() as f64,
			ObjectComparison::NumberAndBoolean(x, y)   => x.value > Number::from(y).value,
			ObjectComparison::NumberAndVariable(x, y)  => x.value.to_string() > y,
			ObjectComparison::TextAndNumber(x, y)      => x.value.len() as f64 > y.value,
			ObjectComparison::TextAndBoolean(x, y)     => x.value.len() > (if y.value { 1 } else { 0 }),
			ObjectComparison::TextAndVariable(x, y)    => x.value > y,
			ObjectComparison::BooleanAndNumber(x, y)   => Number::from(x).value > y.value,
			ObjectComparison::BooleanAndText(x, y)     => Number::from(x).value as usize > y.value.len(),
			ObjectComparison::BooleanAndVariable(x, y) => x.value.to_string() > y,
			ObjectComparison::VariableAndNumber(x, y)  => self > &Object::Number(y.clone()),
			ObjectComparison::VariableAndText(x, y)    => self > &Object::Text(y.clone()),
			ObjectComparison::VariableAndBoolean(x, y) => self > &Object::Bool(y.clone()),
		}
	}
	fn lt(&self, other: &Self) -> bool {
		match ObjectComparison::from((self, other)) {
			ObjectComparison::BothNumber(x, y)         => x < y,
			ObjectComparison::BothText(x, y)           => x.value.len() < y.value.len(),
			ObjectComparison::BothBoolean(x, y)        => !x.value & y.value,
			ObjectComparison::BothVariable(x, y)       => x < y,
			ObjectComparison::NumberAndText(x, y)      => x.value < y.value.len() as f64,
			ObjectComparison::NumberAndBoolean(x, y)   => x.value < Number::from(y).value,
			ObjectComparison::NumberAndVariable(x, y)  => x.value.to_string() < y,
			ObjectComparison::TextAndNumber(x, y)      => (x.value.len() as f64) < y.value,
			ObjectComparison::TextAndBoolean(x, y)     => x.value.len() < (if y.value { 1 } else { 0 }),
			ObjectComparison::TextAndVariable(x, y)    => x.value < y,
			ObjectComparison::BooleanAndNumber(x, y)   => Number::from(x).value < y.value,
			ObjectComparison::BooleanAndText(x, y)     => (Number::from(x).value as usize) < y.value.len(),
			ObjectComparison::BooleanAndVariable(x, y) => x.value.to_string() < y,
			ObjectComparison::VariableAndNumber(x, y)  => self < &Object::Number(y.clone()),
			ObjectComparison::VariableAndText(x, y)    => self < &Object::Text(y.clone()),
			ObjectComparison::VariableAndBoolean(x, y) => self < &Object::Bool(y.clone()),
		}
	}

	fn ge(&self, other: &Self) -> bool {
		!self.lt(other)
	}

	fn le(&self, other: &Self) -> bool {
		!self.gt(other)
	}

	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match ObjectComparison::from((self, other)) {
			ObjectComparison::BothNumber(x, y) => x.partial_cmp(&y),
			ObjectComparison::BothText(x, y) => x.value.len().partial_cmp(&y.value.len()),
			ObjectComparison::BothBoolean(x, y) => x.value.partial_cmp(&y.value),
			ObjectComparison::BothVariable(x, y) => x.partial_cmp(&y),
			ObjectComparison::NumberAndText(x, y) => x.value.partial_cmp(&(y.value.len() as f64)),
			ObjectComparison::NumberAndBoolean(x, y) => x.value.partial_cmp(&Number::from(y).value),
			ObjectComparison::NumberAndVariable(x, y) => x.value.to_string().partial_cmp(&y),
			ObjectComparison::TextAndNumber(x, y) => (x.value.len() as f64).partial_cmp(&y.value),
			ObjectComparison::TextAndBoolean(x, y) => x.value.len().partial_cmp(&(if y.value { 1 } else { 0 })),
			ObjectComparison::TextAndVariable(x, y) => x.value.partial_cmp(&y),
			ObjectComparison::BooleanAndNumber(x, y) => Number::from(x).value.partial_cmp(&y.value),
			ObjectComparison::BooleanAndText(x, y) => (if x.value { 1 } else { 0 }).partial_cmp(&y.value.len()),
			ObjectComparison::BooleanAndVariable(x, y) => x.value.to_string().partial_cmp(&y),
			ObjectComparison::VariableAndNumber(_, y) => self.partial_cmp(&Object::Number(y.clone())),
			ObjectComparison::VariableAndText(_, y) => self.partial_cmp(&Object::Text(y.clone())),
			ObjectComparison::VariableAndBoolean(_, y) => self.partial_cmp(&Object::Bool(y.clone())),
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

impl From<Boolean> for Number {
	fn from(value: Boolean) -> Self {
		Self { value: if value.value {1.0} else {0.0} }
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
		}
	}
}


pub trait CutFromStart<T> {
    fn cut_from_start(&self, whr: fn(&T) -> bool, amount: usize) -> Self;
    fn count_from_start(&self, whr: fn(&T) -> bool) -> usize;
}
impl CutFromStart<TokenData> for Vec<TokenData> {
    fn cut_from_start(&self, whr: fn(&TokenData) -> bool, amount: usize) -> Self {
        let mut inner_self = self.clone();
        let amount = usize::min(amount, inner_self.len());
        for i in (0..amount).rev() {
            if whr(&inner_self[i]) {
                inner_self.remove(i);
            }
        }
        inner_self
    }

    fn count_from_start(&self, whr: fn(&TokenData) -> bool) -> usize {
        let mut count = 0;
        for el in self.clone() {
            if whr(&el) {
                count += 1;
            }
        }

        count
    }
}