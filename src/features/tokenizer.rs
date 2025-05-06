use std::{
	fmt::{Debug, Display, write},
	ops::{Deref, Range},
	slice::SliceIndex,
};

use crate::{library::Types::{Object, Operator}, parsers::Collection::{self, Expression}};
use chumsky::{error::Simple, Error, Parser};
use colored::Colorize;
use const_format::concatcp;
use logos::Logos;
use tabled::{
	Table, Tabled,
	assert::assert_table,
	settings::{
		Alignment, Color, Format, Modify, Style, Width,
		object::{Columns, ObjectIterator, Rows, Segment},
	},
};

#[derive(Clone, Logos, Debug, PartialEq, PartialOrd, Hash, Eq)]
pub enum TokenTable {
	#[regex(r"\t")]
	Tab,

	#[token("eğer")]
	KeywordEğer,
	#[token("ise")]
	Keywordİse,
	#[regex(r"değilse[ \t]+ve")]
	KeywordDeğilseVe,
	#[token("değilse")]
	KeywordDeğilse,

	#[token("ve")]
	KeywordVe,
	#[token("veya")]
	KeywordVeya,

	#[token("yazdır")]
	KeywordYazdır,
	#[regex(r"sürekli[ \t]+tekrarla")]
	KeywordSürekliTekrarla,
	#[regex(r"(?:defa|kere|kez)[ \t]+tekrarla")]
	KeywordNDefaTekrarla,

	#[regex("==|!=|<=|>=|<|>")]
	ComparisonOperator,

	#[regex(r"=|\+=|-=|\*=|/=|%=|")]
	AssignmentOperator,

	#[regex(r"\+")]
	OperatorAdd,
	#[regex(r"\-")]
	OperatorSubtract,
	#[regex(r"\*")]
	OperatorMultiply,
	#[regex(r"\/")]
	OperatorDivide,
	#[regex(r"%")]
	OperatorMod,

	#[token("(")]
	RPAREN,
	#[token(")")]
	LPAREN,
	#[token("()")]
	EmptyParens,

	#[token("[")]
	RSQBRACKET,
	#[token("]")]
	LSQBRACKET,
	#[token("[]")]
	EmptySqBrackets,

	#[token("{")]
	RCRBRACKET,
	#[token("}")]
	LCRBRACKET,
	#[token("{}")]
	EmptyCrBrackets,

	#[token(";")]
	Semicolon,
	#[token(",")]
	Comma,
	#[token(".")]
	Dot,

	#[regex("true|doğru|evet|yes|false|yanlış|hayır|no")]
	BooleanLiteral,
	#[regex(r#""[^"\\]*(?:\\.[^"\\]*)*""#)]
	StringLiteral,
	#[regex(r"-?(0|[1-9][0-9]*)(\.[0-9]+)?")]
	NumberLiteral,

	#[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
	Identifier,

	#[regex(r"[ \n\r]+", logos::skip)]
	Error,
}

#[derive(Debug)]
enum Expr {
	Num(f64),
	Neg(Box<Expr>),
	Add(Box<Expr>, Box<Expr>),
	Sub(Box<Expr>, Box<Expr>),
	Mul(Box<Expr>, Box<Expr>),
	Div(Box<Expr>, Box<Expr>),
}

impl Expr {
	fn eval(&self) -> f64 {
		match self {
			Expr::Num(n) => *n,
			Expr::Neg(rhs) => -rhs.eval(),
			Expr::Add(lhs, rhs) => lhs.eval() + rhs.eval(),
			Expr::Sub(lhs, rhs) => lhs.eval() - rhs.eval(),
			Expr::Mul(lhs, rhs) => lhs.eval() * rhs.eval(),
			Expr::Div(lhs, rhs) => lhs.eval() / rhs.eval(),
		}
	}
}

impl Display for TokenTable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TokenTable::Tab => write!(f, "\t"),
			TokenTable::KeywordEğer => write!(f, "KeywordEğer"),
			TokenTable::Keywordİse => write!(f, "Keywordİse"),
			TokenTable::KeywordDeğilseVe => write!(f, "KeywordDeğilseVe"),
			TokenTable::KeywordDeğilse => write!(f, "KeywordDeğilse"),
			TokenTable::KeywordVe => write!(f, "KeywordVe"),
			TokenTable::KeywordVeya => write!(f, "KeywordVeya"),
			TokenTable::KeywordYazdır => write!(f, "KeywordYazdır"),
			TokenTable::KeywordSürekliTekrarla => write!(f, "KeywordSürekliTekrarla"),
			TokenTable::KeywordNDefaTekrarla => write!(f, "KeywordNDefaTekrarla"),
			TokenTable::ComparisonOperator => write!(f, "ComparisonOperator"),
			TokenTable::AssignmentOperator => write!(f, "AssignmentOperator"),
			TokenTable::OperatorAdd => write!(f, "OperatorAdd"),
			TokenTable::OperatorSubtract => write!(f, "OperatorSubtract"),
			TokenTable::OperatorMultiply => write!(f, "OperatorMultiply"),
			TokenTable::OperatorDivide => write!(f, "OperatorDivide"),
			TokenTable::OperatorMod => write!(f, "OperatorMod"),
			TokenTable::RPAREN => write!(f, "RPAREN"),
			TokenTable::LPAREN => write!(f, "LPAREN"),
			TokenTable::EmptyParens => write!(f, "EmptyParens"),
			TokenTable::RSQBRACKET => write!(f, "RSQBRACKET"),
			TokenTable::LSQBRACKET => write!(f, "LSQBRACKET"),
			TokenTable::EmptySqBrackets => write!(f, "EmptySqBrackets"),
			TokenTable::RCRBRACKET => write!(f, "RCRBRACKET"),
			TokenTable::LCRBRACKET => write!(f, "LCRBRACKET"),
			TokenTable::EmptyCrBrackets => write!(f, "EmptyCrBrackets"),
			TokenTable::Semicolon => write!(f, "Semicolon"),
			TokenTable::Comma => write!(f, "Comma"),
			TokenTable::Dot => write!(f, "Dot"),
			TokenTable::BooleanLiteral => write!(f, "BooleanLiteral"),
			TokenTable::StringLiteral => write!(f, "StringLiteral"),
			TokenTable::NumberLiteral => write!(f, "NumberLiteral"),
			TokenTable::Identifier => write!(f, "Identifier"),
			TokenTable::Error => write!(f, "Error"),
		}
	}
}

#[derive(Clone, Debug, Hash)]
pub struct TokenData {
	pub isOk: bool,
	pub token: TokenTable,
	pub slice: String,
	pub span: Range<usize>,
}

impl TokenData {
	pub fn new(token: TokenTable, slice: String, span: Range<usize>) -> Self {
		Self {
			isOk: true,
			token,
			slice,
			span,
		}
	}

	pub fn default(token: TokenTable) -> Self {
		Self {
			isOk: true,
			token,
			slice: "".to_owned(),
			span: 0..0,
		}
	}

	pub fn toOp(&self) -> fn(Box<Collection::Expression>, Box<Collection::Expression>) -> Collection::Expression {
		match self.token {
			TokenTable::OperatorAdd => Collection::Expression::Add,
			TokenTable::OperatorSubtract => Collection::Expression::Sub,
			TokenTable::OperatorMultiply => Collection::Expression::Mul,
			TokenTable::OperatorDivide => Collection::Expression::Div,
			TokenTable::OperatorMod => Collection::Expression::Mod,
			_ => panic!()
		}
	}

	pub fn asNumberLiteral(&self) -> f64 {
		match self.token {
			TokenTable::NumberLiteral => self.slice.parse::<f64>().unwrap_or(0.0), // TODO: Replace all unwrap_or statements with proper errors.
			_ => 0.0,
		}
	}

	pub fn asStringLiteral(&self) -> String {
		match self.token {
			TokenTable::StringLiteral => self.slice.clone(), // TODO: Replace all unwrap_or statements with proper errors.
			_ => String::new(),
		}
	}
	pub fn asBooleanLiteral(&self) -> bool {
		match self.token {
			TokenTable::BooleanLiteral => match self.slice.as_str() {
				"true" | "doğru" | "evet" | "yes" => true,
				"false" | "yanlış" | "hayır" | "no" => false,
				_ => false,
			}, // TODO: Replace all unwrap_or statements with proper errors.
			_ => false,
		}
	}
}

trait RemoveQuotes {
	fn remove_quotes(&self) -> String;
}

impl RemoveQuotes for String {
	fn remove_quotes(&self) -> String {
		if self.starts_with("\"") && self.ends_with("\"") {
			return self[1..self.len() - 1].to_string();
		}
		self.clone()
		//  else {
		// 	panic!("The string slice doesn't start and end with (\") symbol. This is probably the lexer's fault.")
		// }
	}
}

impl Display for TokenData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			String::from(match &self.token {
				TokenTable::StringLiteral => self.slice.remove_quotes(),
				TokenTable::BooleanLiteral => self.slice.clone(),
				TokenTable::NumberLiteral => self.slice.clone(),
				_ => format!("{}", &self.token),
			})
		)
	}
}

impl PartialEq for TokenData {
	fn eq(&self, other: &Self) -> bool {
		match (self.token.clone(), other.token.clone()) {
			(TokenTable::StringLiteral, TokenTable::StringLiteral) => true,
			(TokenTable::NumberLiteral, TokenTable::NumberLiteral) => true,
			(TokenTable::BooleanLiteral, TokenTable::BooleanLiteral) => true,
			_ => self.token == other.token,
		}
	}
	fn ne(&self, other: &Self) -> bool {
		self.token != other.token
	}
}
impl Eq for TokenData {}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionEnum {
	NoOp,
	Print(Vec<TokenData>),
	Input(Vec<TokenData>),
	Repeat(f64),
	WhileTrue(Vec<TokenData>),
	IfBlock(Vec<TokenData>),
	ElifBlock(Vec<TokenData>),
	ElseBlock(Vec<TokenData>),
	VariableDeclaration(Vec<TokenData>),
	Break(Vec<TokenData>),
	Continue(Vec<TokenData>),
}

pub fn tokenize(input: &str) -> Vec<TokenData> {
	let mut lexer = TokenTable::lexer(input);
	let mut tokens = Vec::new();

	while let Some(token) = lexer.next() {
		tokens.push(TokenData {
			isOk: token.is_ok(),
			token: token.clone().unwrap_or(TokenTable::Error),
			slice: lexer.slice().to_string(),
			span: lexer.span(),
		});
	}
	tokens
}
