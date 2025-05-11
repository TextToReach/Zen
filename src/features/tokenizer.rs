#![allow(dead_code)]

use std::{
	fmt::{write, Debug, Display},
	ops::{Not, Range},
};

use crate::{library::Types::Object, parsers::Parsers::Expression, util::ScopeManager::ScopeAction};
use logos::Logos;

#[derive(Clone, Logos, Debug, PartialEq, PartialOrd, Hash, Eq)]
pub enum TokenTable {
	#[regex(r"\t")]
	Tab,

	#[token(r"//")]
	Comment,

	#[token("eğer")]
	KeywordEğer,
	#[token(r"ise")]
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

	#[token("==")]
	ComparisonOperatorEqual,
	#[token("!=")]
	ComparisonOperatorNotEqual,
	#[token("<<")]
	ComparisonOperatorLessThan,
	#[token(">>")]
	ComparisonOperatorGreaterThan,
	#[token("<=")]
	ComparisonOperatorLessThanOrEqual,
	#[token(">=")]
	ComparisonOperatorGreaterThanOrEqual,
	
	#[regex(r"\+")]
	MathOperatorAdd,
	#[regex(r"\-")]
	MathOperatorSubtract,
	#[regex(r"\*")]
	MathOperatorMultiply,
	#[regex(r"\/")]
	MathOperatorDivide,
	#[regex(r"%")]
	MathOperatorMod,

	#[token(r"=")]
	AssignmentOperatorSet,
	#[token(r"+=")]
	AssignmentOperatorAdd,
	#[token(r"-=")]
	AssignmentOperatorSubtract,
	#[token(r"*=")]
	AssignmentOperatorMultiply,
	#[token(r"/=")]
	AssignmentOperatorDivide,

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
	#[regex(r"(0|[1-9][0-9]*)(\.[0-9]+)?")]
	NumberLiteral,
	#[regex(r"-(0|[1-9][0-9]*)(\.[0-9]+)?")]
	NegativeNumberLiteral,

	#[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
	Identifier,

	#[regex(r"[ \n\r]+", logos::skip)]
	Error,

}

impl TokenTable {
	pub fn asTokenData(&self) -> TokenData {
		TokenData::default(self.clone())
	}
}

impl Display for TokenTable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TokenTable::Tab => write!(f, "\t"),
			TokenTable::Comment => write!(f, "Comment"),
			TokenTable::KeywordEğer => write!(f, "KeywordEğer"),
			TokenTable::Keywordİse => write!(f, "Keywordİse"),
			TokenTable::KeywordDeğilseVe => write!(f, "KeywordDeğilseVe"),
			TokenTable::KeywordDeğilse => write!(f, "KeywordDeğilse"),
			TokenTable::KeywordVe => write!(f, "KeywordVe"),
			TokenTable::KeywordVeya => write!(f, "KeywordVeya"),
			TokenTable::KeywordYazdır => write!(f, "KeywordYazdır"),
			TokenTable::KeywordSürekliTekrarla => write!(f, "KeywordSürekliTekrarla"),
			TokenTable::KeywordNDefaTekrarla => write!(f, "KeywordNDefaTekrarla"),
			TokenTable::MathOperatorAdd => write!(f, "OperatorAdd"),
			TokenTable::MathOperatorSubtract => write!(f, "OperatorSubtract"),
			TokenTable::MathOperatorMultiply => write!(f, "OperatorMultiply"),
			TokenTable::MathOperatorDivide => write!(f, "OperatorDivide"),
			TokenTable::MathOperatorMod => write!(f, "OperatorMod"),
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
			TokenTable::NegativeNumberLiteral => write!(f, "NegativeNumberLiteral"),
			TokenTable::Identifier => write!(f, "Identifier"),
			TokenTable::ComparisonOperatorEqual => write!(f, "ComparisonOperatorEqual"),
			TokenTable::ComparisonOperatorNotEqual => write!(f, "ComparisonOperatorNotEqual"),
			TokenTable::ComparisonOperatorLessThan => write!(f, "ComparisonOperatorLessThan"),
			TokenTable::ComparisonOperatorGreaterThan => write!(f, "ComparisonOperatorGreaterThan"),
			TokenTable::ComparisonOperatorLessThanOrEqual => write!(f, "ComparisonOperatorLessThanOrEqual"),
			TokenTable::ComparisonOperatorGreaterThanOrEqual => write!(f, "ComparisonOperatorGreaterThanOrEqual"),
			TokenTable::AssignmentOperatorSet => write!(f, "AssignmentOperatorSet"),
			TokenTable::AssignmentOperatorAdd => write!(f, "AssignmentOperatorAdd"),
			TokenTable::AssignmentOperatorSubtract => write!(f, "AssignmentOperatorSubtract"),
			TokenTable::AssignmentOperatorMultiply => write!(f, "AssignmentOperatorMultiply"),
			TokenTable::AssignmentOperatorDivide => write!(f, "AssignmentOperatorDivide"),
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

	pub fn toOp(&self) -> fn(Box<Expression>, Box<Expression>) -> Expression {
		match self.token {
			TokenTable::MathOperatorAdd => Expression::Add,
			TokenTable::MathOperatorSubtract => Expression::Sub,
			TokenTable::MathOperatorMultiply => Expression::Mul,
			TokenTable::MathOperatorDivide => Expression::Div,
			TokenTable::MathOperatorMod => Expression::Mod,
			TokenTable::ComparisonOperatorEqual => Expression::Equal,
			TokenTable::ComparisonOperatorNotEqual => Expression::NotEqual,
			TokenTable::ComparisonOperatorGreaterThan => Expression::GreaterThan,
			TokenTable::ComparisonOperatorGreaterThanOrEqual => Expression::GreaterThanOrEqual,
			TokenTable::ComparisonOperatorLessThan => Expression::LessThan,
			TokenTable::ComparisonOperatorLessThanOrEqual => Expression::LessThanOrEqual,
			_ => panic!()
		}

		
	}

	pub fn asNumberLiteral(&self) -> f64 {
		match self.token {
			TokenTable::NumberLiteral | TokenTable::NegativeNumberLiteral => self.slice.parse::<f64>().unwrap_or(0.0), // TODO: Replace all unwrap_or statements with proper errors.
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

	pub fn asObject(&self) -> Object {
		match self.token {
			TokenTable::StringLiteral => Object::from(self.asStringLiteral()),
			TokenTable::NumberLiteral => Object::from(self.asNumberLiteral()),
			TokenTable::NegativeNumberLiteral => Object::from(self.asNumberLiteral()),
			TokenTable::BooleanLiteral => Object::from(self.asBooleanLiteral()),
			TokenTable::Identifier => Object::Variable(self.slice.clone()),
			_ => panic!("Unsupported token type for conversion to Object."),
		}
	}
}

pub trait RemoveQuotes {
	fn remove_quotes(&self) -> String;
}
pub trait CheckTokenVec {
	fn is_all_ok(&self) -> bool;
}

impl CheckTokenVec for Vec<TokenData> {
	fn is_all_ok(&self) -> bool {
		self.is_empty().not() || self.iter().all(|x| x.isOk)
	}
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
pub enum AssignmentMethod { Set, Add, Sub, Mul, Div }

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionEnum {
	NoOp,
	Print(Vec<Expression>),
	Input(Vec<TokenData>),
	
	// BLOCKS
	Repeat 		{ scope_pointer: usize, repeat_count: f64 },
	WhileTrue 	{ scope_pointer: usize },
	IfBlock 	{ scope_pointer: usize, condition: Expression },
	ElifBlock 	{ scope_pointer: usize, condition: Expression },
	ElseBlock 	{ scope_pointer: usize },
	// BLOCKS

	VariableDeclaration(String, Expression, AssignmentMethod),
	Break(Vec<TokenData>),
	Continue(Vec<TokenData>)
}

impl InstructionEnum {
	pub fn is_block(&self) -> bool {
		matches!(
			self,
			InstructionEnum::IfBlock { .. } |
			InstructionEnum::ElifBlock { .. } |
			InstructionEnum::ElseBlock { .. } |
			InstructionEnum::WhileTrue { .. } |
			InstructionEnum::Repeat { .. }
		)
	}
	pub fn as_block_action(&self) -> ScopeAction {
		match self {
			InstructionEnum::IfBlock { condition, scope_pointer } => ScopeAction::IfBlock{ condition: condition.clone() },
			InstructionEnum::ElifBlock { condition, scope_pointer } => ScopeAction::ElifBlock{ condition: condition.clone() },
			InstructionEnum::ElseBlock { .. } => ScopeAction::ElseBlock,
			InstructionEnum::WhileTrue { .. } => ScopeAction::WhileTrue,
			InstructionEnum::Repeat { repeat_count, scope_pointer } => ScopeAction::Repeat(*repeat_count),
			_ => panic!()
		}
	}
	
	pub fn set_block_pointer(&mut self, pointer: usize) {
		match self {
			InstructionEnum::IfBlock { scope_pointer, .. } |
			InstructionEnum::ElifBlock { scope_pointer, .. } |
			InstructionEnum::ElseBlock { scope_pointer } |
			InstructionEnum::WhileTrue { scope_pointer } |
			InstructionEnum::Repeat { scope_pointer, .. } => {
				*scope_pointer = pointer
			}
			_ => panic!()
		}
	}
}

pub fn tokenize(input: &str) -> Vec<TokenData> {
	let mut lexer = TokenTable::lexer(input);
	let mut tokens = Vec::new();

	while let Some(token) = lexer.next() {
		tokens.push(TokenData {
			isOk: token.is_ok(),
			token: token.clone().unwrap_or(TokenTable::Error),
			slice: match token {
				Ok(TokenTable::StringLiteral) => lexer.slice().to_string().remove_quotes(),
				_ => lexer.slice().to_string(),
			},
			span: lexer.span(),
		});
	}
	// println!("Tokenizer: {tokens:#?}");
	tokens
}
