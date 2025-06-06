#![allow(non_snake_case, dead_code)]

use chumsky::chain::Chain;
use chumsky::prelude::*;
use chumsky::text::whitespace;
use chumsky::{Parser, error::Simple};
use colored::Colorize;
use miette::{NamedSource, SourceSpan};
use num::iter::Range;
use num::pow::Pow;

use crate::features::tokenizer::{RemoveQuotes, TokenData, TokenTable};
use crate::library::Methods::Throw;
use crate::parsers::Parsers::Expression;
use crate::util::ScopeManager::ScopeManager;

use std::cell::RefCell;
use std::ops::{Add, Div, Index, Mul, Rem, Sub};
use std::rc::Rc;
use std::{fmt::Display, num::ParseFloatError, str::FromStr};

use super::Error::TipHatası;

static LETTERARRAY: &'static str = "abcçdefgğhıijklmnoöprsştuüvyzABCÇDEFGĞHIİJKLMNOÖPRSŞTUÜVYZ";

/// The exact equals:
/// - Number = f64
/// - Text = String
/// - Boolean = bool
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
pub struct ParameterData {
	pub name: String,
	pub data_type: Option<TokenData>,
	pub default_value: Option<Expression>,
}

/// This type just says that the ParameterData is resolved and evaluated and is ready to be used inside the default_value attribute.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedParameterData {
	pub name: String,
	pub data_type: Option<ObjectType>,
	pub default_value: Option<Object>,
}

impl ParameterData {
	pub fn toResolved(&self, currentScope: usize, manager: &mut ScopeManager) -> ResolvedParameterData {
		ResolvedParameterData {
			name: self.name.clone(),
			data_type: self.data_type.clone().map(|token| match token.token {
				TokenTable::KeywordSayı => ObjectType::Number,
				TokenTable::KeywordMetin => ObjectType::Text,
				TokenTable::KeywordMantıksal => ObjectType::Boolean,
				_ => panic!("Unsupported TokenData variant for conversion to ObjectType: {:?}", token),
			}),
			default_value: self.default_value.clone().map(|expr| expr.evaluate(currentScope, manager)),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
	pub name: String,
	pub args: Vec<ResolvedParameterData>,
	pub scope_pointer: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
	Number,
	Text,
	Boolean,
	Variable,
	Null,
	Array,
}

impl Display for ObjectType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let type_str = match self {
			ObjectType::Number => "Sayı",
			ObjectType::Text => "Metin",
			ObjectType::Boolean => "Mantıksal",
			ObjectType::Variable => "Değişken",
			ObjectType::Null => "NIL",
			ObjectType::Array => "Dizi",
		};
		write!(f, "{}", type_str)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
	Number(Number),
	Text(Text),
	Bool(Boolean),
	Variable(String),
	Array(Array),
	Null,
}

impl Object {
	pub fn get_type(&self) -> ObjectType {
		match self {
			Object::Number(_) => ObjectType::Number,
			Object::Text(_) => ObjectType::Text,
			Object::Bool(_) => ObjectType::Boolean,
			Object::Variable(_) => ObjectType::Variable,
			Object::Null => ObjectType::Null,
			Object::Array(_) => ObjectType::Array,
		}
	}

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
			Object::Array(val) => !val.value.is_empty(),
			Object::Variable(_) => true,
			Object::Null => false,
		}
	}

	pub fn expectToBeNumber(&self, src: NamedSource<String>, span: SourceSpan) -> Result<&Number, TipHatası> {
		if let Object::Number(val) = self {
			Ok(val)
		} else {
			Err(TipHatası::expected(
				"Verilen tipin bir sayı olması bekleniyordu.".to_string(),
				format!("{:?}", self),
				src,
				span,
			))
		}
	}

	pub fn expectToBePozitiveNumber(&self, src: NamedSource<String>, span: SourceSpan) -> Result<&Number, TipHatası> {
		if let Object::Number(val) = self {
			if val.value >= 0.0 {
				Ok(val)
			} else {
				Err(TipHatası::expected(
					"Verilen tipin bir pozitif sayı olması bekleniyordu.".to_string(),
					format!("{:?}", self),
					src,
					span,
				))
			}
		} else {
			Err(TipHatası::expected(
				"Verilen tipin bir pozitif sayı olması bekleniyordu.".to_string(),
				format!("{:?}", self),
				src,
				span,
			))
		}
	}

	pub fn expectToBeText(&self, src: NamedSource<String>, span: SourceSpan) -> Result<&Text, TipHatası> {
		if let Object::Text(val) = self {
			Ok(val)
		} else {
			Err(TipHatası::expected(
				"Verilen tipin bir metin olması bekleniyordu.".to_string(),
				format!("{:?}", self),
				src,
				span,
			))
		}
	}

	pub fn expectToBeBool(&self, src: NamedSource<String>, span: SourceSpan) -> Result<&Boolean, TipHatası> {
		if let Object::Bool(val) = self {
			Ok(val)
		} else {
			Err(TipHatası::expected(
				"Verilen tipin br mantıksal olması bekleniyordu.".to_string(),
				format!("{:?}", self),
				src,
				span,
			))
		}
	}

	pub fn expectToBe(&self, expected: ObjectType, src: NamedSource<String>, span: SourceSpan) -> Result<(), TipHatası> {
		match (self, expected.clone()) {
			(Object::Number(_), ObjectType::Number) => Ok(()),
			(Object::Text(_), ObjectType::Text) => Ok(()),
			(Object::Bool(_), ObjectType::Boolean) => Ok(()),
			(Object::Variable(_), ObjectType::Variable) => Ok(()),
			(Object::Null, ObjectType::Null) => Ok(()),
			_ => Err(TipHatası::expected(
				format!("Verilen tipin bir {} olması bekleniyordu.", expected),
				format!("{:?}", self),
				src,
				span,
			)),
		}
	}

	pub fn forceIntoNumber(&self) -> Number {
		match self {
			Object::Number(val) => val.clone(),
			Object::Text(val) => Number::from(val.value.parse::<f64>().unwrap_or(0.0)),
			Object::Bool(val) => Number::from(if val.value { 1.0 } else { 0.0 }),
			Object::Variable(val) => Number::from(val.parse::<f64>().unwrap_or(0.0)),
			Object::Null => Number::from(0.0),
			Object::Array(val) => Number::from(if val.value.is_empty() { 0.0 } else { 1.0 }),
		}
	}

	pub fn forceIntoText(&self) -> Text {
		match self {
			Object::Number(val) => Text::from(val.value.to_string()),
			Object::Text(val) => val.clone(),
			Object::Bool(val) => Text::from(val.value.to_string()),
			Object::Variable(val) => Text::from(val.clone()),
			Object::Null => Text::from("NIL".to_string()),
			Object::Array(val) => Text::from(val.value.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(", ")),
		}
	}

	pub fn forceIntoBool(&self) -> Boolean {
		match self {
			Object::Number(val) => Boolean::from(val.value != 0.0),
			Object::Text(val) => Boolean::from(match val.value.as_str() {
				"true" | "doğru" | "evet" | "yes" => true,
				"false" | "yanlış" | "hayır" | "no" => false,
				_ => false,
			}),
			Object::Bool(val) => val.clone(),
			Object::Variable(val) => Boolean::from(!val.remove_quotes().is_empty()),
			Object::Null => Boolean::from(false),
			Object::Array(val) => Boolean::from(val.value.is_empty()),
		}
	}

	pub fn forceIntoArray(&self) -> Array {
		match self {
			Object::Number(val) => Array::from(vec![Object::Number(val.clone())]),
			Object::Text(val) => Array::from(vec![Object::Text(val.clone())]),
			Object::Bool(val) => Array::from(vec![Object::Bool(val.clone())]),
			Object::Variable(val) => Array::from(vec![Object::Variable(val.clone())]),
			Object::Null => Array::from(vec![]),
			Object::Array(val) => val.clone(),
		}
	}

	pub fn convertToNumber(&self) -> Result<Number, ()> {
		match self {
			Object::Number(val) => Ok(val.clone()),
			Object::Text(val) => match val.value.parse::<f64>() {
				Ok(num) => Ok(Number::from(num)),
				Err(_) => Err(()),
			},
			Object::Bool(val) => Err(()),
			Object::Variable(val) => Err(()), // I mean, if you try to force a variable into a number, you have problems.
			Object::Null => Err(()),
			Object::Array(val) => Err(()),
		}
	}

	pub fn convertToText(&self) -> Result<Text, ()> {
		match self {
			Object::Text(val) => Ok(val.clone()),
			Object::Number(val) => Err(()),
			Object::Bool(val) => Err(()),
			Object::Variable(val) => Err(()),
			Object::Null => Err(()),
			Object::Array(val) => Err(()),
		}
	}

	pub fn convertToBool(&self) -> Result<Boolean, ()> {
		match self {
			Object::Bool(val) => Ok(val.clone()),
			Object::Number(_) => Err(()),
			Object::Text(_) => Err(()),
			Object::Variable(_) => Err(()),
			Object::Null => Err(()),
			Object::Array(val) => Err(()),
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
			ObjectComparison::BothNumber(x, y) => x > y,
			ObjectComparison::BothText(x, y) => x.value.len() > y.value.len(),
			ObjectComparison::BothBoolean(x, y) => x.value & !y.value,
			ObjectComparison::BothVariable(x, y) => x > y,
			ObjectComparison::NumberAndText(x, y) => x.value > y.value.len() as f64,
			ObjectComparison::NumberAndBoolean(x, y) => x.value > Number::from(y).value,
			ObjectComparison::NumberAndVariable(x, y) => x.value.to_string() > y,
			ObjectComparison::TextAndNumber(x, y) => x.value.len() as f64 > y.value,
			ObjectComparison::TextAndBoolean(x, y) => x.value.len() > (if y.value { 1 } else { 0 }),
			ObjectComparison::TextAndVariable(x, y) => x.value > y,
			ObjectComparison::BooleanAndNumber(x, y) => Number::from(x).value > y.value,
			ObjectComparison::BooleanAndText(x, y) => Number::from(x).value as usize > y.value.len(),
			ObjectComparison::BooleanAndVariable(x, y) => x.value.to_string() > y,
			ObjectComparison::VariableAndNumber(x, y) => self > &Object::Number(y.clone()),
			ObjectComparison::VariableAndText(x, y) => self > &Object::Text(y.clone()),
			ObjectComparison::VariableAndBoolean(x, y) => self > &Object::Bool(y.clone()),
		}
	}
	fn lt(&self, other: &Self) -> bool {
		match ObjectComparison::from((self, other)) {
			ObjectComparison::BothNumber(x, y) => x < y,
			ObjectComparison::BothText(x, y) => x.value.len() < y.value.len(),
			ObjectComparison::BothBoolean(x, y) => !x.value & y.value,
			ObjectComparison::BothVariable(x, y) => x < y,
			ObjectComparison::NumberAndText(x, y) => x.value < y.value.len() as f64,
			ObjectComparison::NumberAndBoolean(x, y) => x.value < Number::from(y).value,
			ObjectComparison::NumberAndVariable(x, y) => x.value.to_string() < y,
			ObjectComparison::TextAndNumber(x, y) => (x.value.len() as f64) < y.value,
			ObjectComparison::TextAndBoolean(x, y) => x.value.len() < (if y.value { 1 } else { 0 }),
			ObjectComparison::TextAndVariable(x, y) => x.value < y,
			ObjectComparison::BooleanAndNumber(x, y) => Number::from(x).value < y.value,
			ObjectComparison::BooleanAndText(x, y) => (Number::from(x).value as usize) < y.value.len(),
			ObjectComparison::BooleanAndVariable(x, y) => x.value.to_string() < y,
			ObjectComparison::VariableAndNumber(x, y) => self < &Object::Number(y.clone()),
			ObjectComparison::VariableAndText(x, y) => self < &Object::Text(y.clone()),
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
pub enum ZenError {
	UnknownError,
	GeneralError,
	NotDeclaredError,
	DivisionByZeroError,
	TypeError,
	IndentationError,
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Array {
	pub value: Vec<Object>,
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
		Object::Array(Array { value })
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
		Self {
			value: if value.value { 1.0 } else { 0.0 },
		}
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

impl New<f64> for Array {
	fn enum_from(value: f64) -> Object {
		Object::Array(Self {
			value: vec![Object::from(value)],
		})
	}

	fn new() -> Self {
		Self { value: vec![] }
	}
}

impl New<String> for Array {
	fn enum_from(value: String) -> Object {
		Object::Array(Self {
			value: vec![Object::from(value)],
		})
	}

	fn new() -> Self {
		Self { value: vec![] }
	}
}

impl New<bool> for Array {
	fn enum_from(value: bool) -> Object {
		Object::Array(Self {
			value: vec![Object::from(value)],
		})
	}

	fn new() -> Self {
		Self { value: vec![] }
	}
}

impl From<Number> for Array {
	fn from(value: Number) -> Self {
		Array {
			value: vec![Object::from(value)],
		}
	}
}
impl From<Text> for Array {
	fn from(value: Text) -> Self {
		Array {
			value: vec![Object::from(value)],
		}
	}
}
impl From<Boolean> for Array {
	fn from(value: Boolean) -> Self {
		Array {
			value: vec![Object::from(value)],
		}
	}
}
impl From<Variable> for Array {
	fn from(value: Variable) -> Self {
		Array {
			value: vec![Object::from(value)],
		}
	}
}
impl From<Object> for Array {
	fn from(value: Object) -> Self {
		match value {
			Object::Array(arr) => arr,
			_ => Array::from(vec![value]),
		}
	}
}
impl From<Vec<Object>> for Array {
	fn from(value: Vec<Object>) -> Self {
		Array { value }
	}
}

impl From<Number> for Object {
	fn from(value: Number) -> Self {
		Object::Number(value)
	}
}
impl From<Text> for Object {
	fn from(value: Text) -> Self {
		Object::Text(value)
	}
}
impl From<Boolean> for Object {
	fn from(value: Boolean) -> Self {
		Object::Bool(value)
	}
}
impl From<Variable> for Object {
	fn from(value: Variable) -> Self {
		Object::Variable(value.value)
	}
}
impl From<Array> for Object {
	fn from(value: Array) -> Self {
		Object::Array(value)
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
		let temp = format!("{}", if self.value { "doğru" } else { "yanlış" });
		write!(f, "{}", temp.bright_blue())
	}
}

impl Display for Array {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut elements = String::new();
		for (i, el) in self.value.iter().enumerate() {
			elements.push_str(&format!("{}", el));
			if i < self.value.len() - 1 {
				elements.push_str(", ");
			}
		}
		write!(f, "[{}]", elements)
	}
}

impl Display for Object {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Object::Bool(val) => write!(f, "{}", val),
			Object::Number(val) => write!(f, "{}", val),
			Object::Text(val) => write!(f, "{}", val),
			Object::Variable(val) => write!(f, "{}", val),
			Object::Array(val) => write!(f, "{}", val),
			Object::Null => write!(f, "NIL"),
		}
	}
}

impl IntoIterator for Array {
	type Item = Object;
	type IntoIter = std::vec::IntoIter<Object>;

	fn into_iter(self) -> Self::IntoIter {
		self.value.into_iter()
	}
}

impl Index<usize> for Array {
	type Output = Object;

	fn index(&self, index: usize) -> &Self::Output {
		&self.value[index]
	}
}

// ------------------------------------------ Extras ------------------------------------------

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
		Object::from("")..Object::from("");
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

#[derive(Debug, Clone, PartialEq)]
pub enum RandomizerType {
	Number,
	Letter,
	Boolean { chance: Expression },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeUnit {
	Millisecond,
	Second,
	Minute,
	Hour,
	Day,
	Week,
	Month,
	Year,
}
