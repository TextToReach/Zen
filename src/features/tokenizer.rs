#![allow(dead_code)]

use std::{
	default,
	fmt::{Debug, Display, write},
	ops::{Neg, Not, Range},
};

use crate::{
	Input,
	library::Types::{Object, ParameterData, RandomizerType, TimeUnit},
	parsers::Parsers::Expression,
	util::ScopeManager::{ConditionBlock, ScopeAction, ScopeManager},
};
use logos::Logos;
use rand::Rng;

#[derive(Clone, Logos, Debug, PartialEq, PartialOrd, Hash, Eq)]
pub enum TokenTable {
	#[regex(r"\t")]
	Tab,

	#[token(r"//")]
	Comment,

	#[token("?")]
	QuestionMark,
	#[token("!")]
	ExclamationMark,

	#[token("eğer")]
	KeywordEğer,
	#[token(r"ise")]
	Keywordİse,
	#[regex(r"değilse[ \t]+ve")]
	KeywordDeğilseVe,
	#[token("değilse")]
	KeywordDeğilse,

	#[token("sayı")]
	KeywordSayı,
	#[token("metin")]
	KeywordMetin,
	#[token("mantıksal")]
	KeywordMantıksal,
	#[token("ihtimal")]
	KeywordIhtimal,
	#[token("harf")]
	KeywordHarf,

	#[token("salise")]
	KeywordSalise,
	#[token("saniye")]
	KeywordSaniye,
	#[token("dakika")]
	KeywordDakika,
	#[token("saat")]
	KeywordSaat,
	#[token("gün")]
	KeywordGün,
	#[token("hafta")]
	KeywordHafta,
	#[token("ay")]
	KeywordAy,
	#[token("yıl")]
	KeywordYıl,
	#[token("bekle")]
	KeywordBekle,

	#[token("ve")]
	KeywordVe,
	#[token("veya")]
	KeywordVeya,
	#[token("ile")]
	Keywordİle,
	#[token("aralığında")]
	KeywordAralığında,
	#[token("içinde")]
	Keywordİçinde,
	#[token("arasında")]
	KeywordArasında,
	#[token("artarak")]
	KeywordArtarak,
	#[token("dolan")]
	KeywordDolan,
	#[token("değil")]
	KeywordDeğil,

	#[token("yazdır")]
	KeywordYazdır,
	#[token("girdi")]
	KeywordGirdi,
	#[regex(r"sürekli[ \t]+tekrarla")]
	KeywordSürekliTekrarla,
	#[regex(r"(?:defa|kere|kez)[ \t]+tekrarla")]
	KeywordNDefaTekrarla,
	#[token(r"fonksiyon")]
	KeywordFonksiyon,
	#[regex(r"devam[ \t]+et")]
	KeywordDevamEt,
	#[token(r"durdur")]
	KeywordDurdur,
	#[token("döndür")]
	KeywordDöndür,
	#[token("tip")]
	KeywordTip,
	#[token("rastgele")]
	KeywordRastgele,

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
	#[regex(r"\^")]
	MathOperatorPower,
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
	#[token(r"^=")]
	AssignmentOperatorPower,
	#[token(r"/=")]
	AssignmentOperatorDivide,

	#[token("(")]
	LPAREN,
	#[token(")")]
	RPAREN,
	#[token(":")]
	Colon,

	#[token("[")]
	LSQBRACKET,
	#[token("]")]
	RSQBRACKET,

	#[token("{")]
	LCRBRACKET,
	#[token("}")]
	RCRBRACKET,

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
	#[regex(
		r"[abcçdefgğhıijklmnoöprsştuüvyzqwxABCÇDEFGĞHIİJKLMNOÖPRSŞTUÜVYZQWX_][abcçdefgğhıijklmnoöprsştuüvyzqwxABCÇDEFGĞHIİJKLMNOÖPRSŞTUÜVYZQWX0-9_]*"
	)]
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
		write!(f, "{:?}", self)
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
			TokenTable::MathOperatorPower => Expression::Pow,
			TokenTable::ComparisonOperatorEqual => Expression::Equal,
			TokenTable::ComparisonOperatorNotEqual => Expression::NotEqual,
			TokenTable::ComparisonOperatorGreaterThan => Expression::GreaterThan,
			TokenTable::ComparisonOperatorGreaterThanOrEqual => Expression::GreaterThanOrEqual,
			TokenTable::ComparisonOperatorLessThan => Expression::LessThan,
			TokenTable::ComparisonOperatorLessThanOrEqual => Expression::LessThanOrEqual,
			_ => panic!(),
		}
	}

	pub fn asNumberLiteral(&self, isNegative: bool) -> f64 {
		match self.token {
			TokenTable::NumberLiteral => {
				let num = self.slice.parse::<f64>().unwrap_or(0.0); // TODO: Replace all unwrap_or statements with proper errors.
				if isNegative { -num } else { num }
			}
			_ => 0.0,
		}
	}

	pub fn asStringLiteral(&self) -> String {
		match self.token {
			TokenTable::StringLiteral => self.slice.clone(), // TODO: Replace all unwrap_or statements with proper errors.
			_ => String::new(),
		}
	}

	pub fn asIdentifier(&self) -> String {
		match self.token {
			TokenTable::Identifier => self.slice.clone(), // TODO: Replace all unwrap_or statements with proper errors.
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
			TokenTable::NumberLiteral => Object::from(self.asNumberLiteral(false)),
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
		(!self.is_empty()) && self.iter().all(|x| x.isOk)
	}
}

impl RemoveQuotes for String {
	fn remove_quotes(&self) -> String {
		if self.starts_with("\"") && self.ends_with("\"") {
			return self[1..self.len() - 1].to_string();
		}
		self.clone()
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
pub enum AssignmentMethod {
	Set,
	Add,
	Sub,
	Mul,
	Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionBlockType {
	If,
	Elif,
	Else,
	None,
}

impl ConditionBlockType {
	pub fn is_one_of(&self, types: &[ConditionBlockType]) -> bool {
		types.iter().any(|t| t == self)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
	Expression(Expression),
	YieldInstruction(YieldInstructionEnum),
}

impl From<YieldInstructionEnum> for Box<Atom> {
	fn from(value: YieldInstructionEnum) -> Self {
		Box::new(value.into())
	}
}

impl From<Expression> for Atom {
	fn from(expr: Expression) -> Self {
		Atom::Expression(expr)
	}
}
impl From<YieldInstructionEnum> for Atom {
	fn from(instr: YieldInstructionEnum) -> Self {
		Atom::YieldInstruction(instr)
	}
}

impl Atom {
	pub fn is_expression(&self) -> bool {
		matches!(self, Atom::Expression(_))
	}
	pub fn is_instruction(&self) -> bool {
		matches!(self, Atom::YieldInstruction(_))
	}

	pub fn resolve(&self, currentScope: usize, manager: &mut ScopeManager) -> Expression {
		match self {
			Atom::Expression(expr) => expr.clone(),
			Atom::YieldInstruction(instr) => match instr {
				YieldInstructionEnum::Input { quote, _type } => {
					let out = Object::from(Input!(quote.clone().evaluate(currentScope, manager)));
					if _type.is_none() {
						return Expression::Value(Box::new(out));
					} else {
						match _type.clone().unwrap().token {
							TokenTable::KeywordSayı => Expression::from(Object::Number(out.forceIntoNumber())),
							TokenTable::KeywordMetin => Expression::from(Object::Text(out.forceIntoText())),
							TokenTable::KeywordMantıksal => Expression::from(Object::Bool(out.forceIntoBool())),
							_ => Expression::Value(Box::new(out)),
						}
					}
				}
				YieldInstructionEnum::Random { method, span } => {
					let out = match method {
						RandomizerType::Number => {
							let span = span.clone().unwrap_or((Expression::from(0.0), Expression::from(1.0)));
							let from_val = span.0.evaluate(currentScope, manager).forceIntoNumber().value.floor() as i64;
							let to_val = span.1.evaluate(currentScope, manager).forceIntoNumber().value.floor() as i64;
							let mut rng = rand::rng();
							let rand_num = rng.random_range(from_val..=to_val);
							Expression::from(rand_num as f64)
						}
						RandomizerType::Letter => {
							panic!("Random letter generation is not implemented yet.");
						}
						RandomizerType::Boolean { chance } => {
							let mut rng = rand::rng();
							let rand_bool = rng.random_bool((chance.evaluate(currentScope, manager).forceIntoNumber().value / 100.0).clamp(0.0, 1.0));
							Expression::from(rand_bool)
						}
					};
					// println!("Random instruction: {method:#?} from: {from:#?} to: {to:#?} with result: {out:#?}");
					out
				}
				YieldInstructionEnum::CallFunction { name, args } => {
					let mut args_evaluated = Vec::new();
					for arg in args {
						args_evaluated.push(Expression::from(arg.evaluate(currentScope, manager)));
					}
					match manager.call_function(currentScope, name, args_evaluated) {
						Some(result) => Expression::from(result),
						None => Expression::falsy(),
					}
				}
				YieldInstructionEnum::RandomVar(name) => {
					println!("Random var instruction: {name}");
					Expression::falsy()
				}
				YieldInstructionEnum::Index(name, val) => {
					println!("hi");
					let index_val = val.resolve(currentScope, manager);
					let index = index_val.evaluate(currentScope, manager).forceIntoNumber().value as i64;
					let obj = manager.get_var(currentScope, name.clone());
					if obj.is_none() {
						panic!("Variable {name} not found in scope {currentScope}");
					}
					let obj = obj.unwrap();
					match obj {
						Object::Text(text) => {
							let len = text.value.len() as i64;
							let i = if index < 0 { len + index } else { index };
							if i < 0 || i >= len {
								println!("Index out of bounds: {i} for {len}");
								return Expression::falsy();
							}
							Expression::from(text.value.chars().nth(i as usize).unwrap().to_string())
						}
						Object::Array(list) => {
							let len = list.value.len() as i64;
							let i = if index < 0 { len + index } else { index };
							if i < 0 || i >= len {
								println!("Index out of bounds: {i} for {len}");
								return Expression::falsy();
							}
							Expression::from(list[i as usize].clone())
						}
						_ => {
							println!("Cannot index type: {obj:?}");
							Expression::falsy()
						}
					}
				}
			},
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum YieldInstructionEnum {
	Input {
		quote: Expression,
		_type: Option<TokenData>,
	},
	Random {
		method: RandomizerType,
		span: Option<(Expression, Expression)>,
	},
	RandomVar(String),
	CallFunction {
		name: String,
		args: Vec<Expression>,
	},
	Index(String, Box<Atom>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionEnum {
	NoOp,
	Print(Vec<Atom>),
	Type(Vec<Atom>),
	Wait {
		amount: Atom,
		unit: TimeUnit,
	},

	// BLOCKS
	Repeat {
		scope_pointer: usize,
		repeat_count: Atom,
	},
	For {
		from: Atom,
		to: Atom,
		step: Option<Atom>,
		name: String,
		scope_pointer: usize,
	},
	ForIn {
		name: String,
		step: Option<Atom>,
		varname: String,
		scope_pointer: usize,
	},
	WhileTrue {
		scope_pointer: usize,
	},
	IfBlock {
		scope_pointer: usize,
		condition: Atom,
	},
	ElifBlock {
		scope_pointer: usize,
		condition: Atom,
	},
	ElseBlock {
		scope_pointer: usize,
	},
	Condition(ConditionBlock),
	Function {
		name: String,
		args: Vec<ParameterData>,
		scope_pointer: usize,
	},
	// BLOCKS
	CallFunction {
		name: String,
		args: Vec<Atom>,
	},
	VariableDeclaration(String, Atom, AssignmentMethod),
	Break,
	Continue,
	Return(Atom),
}

impl InstructionEnum {
	pub fn is_block(&self) -> bool {
		matches!(
			self,
			InstructionEnum::IfBlock { .. }
				| InstructionEnum::ElifBlock { .. }
				| InstructionEnum::ElseBlock { .. }
				| InstructionEnum::WhileTrue { .. }
				| InstructionEnum::Repeat { .. }
		)
	}
	pub fn as_block_action(&self) -> ScopeAction {
		match self {
			InstructionEnum::IfBlock { condition, .. } => ScopeAction::Condition(condition.clone()),
			InstructionEnum::ElifBlock { condition, .. } => ScopeAction::Condition(condition.clone()),
			InstructionEnum::ElseBlock { .. } => ScopeAction::Condition(Expression::truthy().into()),
			InstructionEnum::WhileTrue { .. } => ScopeAction::WhileTrue,
			InstructionEnum::Repeat { repeat_count, scope_pointer } => ScopeAction::Repeat(repeat_count.clone()),
			InstructionEnum::For {
				from,
				to,
				step,
				name,
				scope_pointer,
			} => ScopeAction::For(from.clone(), to.clone(), step.clone(), name.clone()),
			InstructionEnum::ForIn {
				name: variable,
				step,
				scope_pointer,
				varname
			} => ScopeAction::ForIn {
				name: variable.clone(),
				step: step.clone(),
			},
			InstructionEnum::Function { name, args, scope_pointer } => ScopeAction::Function {
				name: name.clone(),
				args: args.clone(),
			},
			_ => panic!(),
		}
	}

	pub fn as_expression(&self) -> Atom {
		match self {
			InstructionEnum::IfBlock { condition, .. } => condition.clone(),
			InstructionEnum::ElifBlock { condition, .. } => condition.clone(),
			InstructionEnum::ElseBlock { .. } => Expression::truthy().into(),
			_ => panic!(),
		}
	}

	pub fn set_block_pointer(&mut self, pointer: usize) {
		match self {
			InstructionEnum::IfBlock { scope_pointer, .. }
			| InstructionEnum::ElifBlock { scope_pointer, .. }
			| InstructionEnum::ElseBlock { scope_pointer, .. }
			| InstructionEnum::WhileTrue { scope_pointer }
			| InstructionEnum::Function { scope_pointer, .. }
			| InstructionEnum::For { scope_pointer, .. }
			| InstructionEnum::ForIn { scope_pointer, .. }
			| InstructionEnum::Repeat { scope_pointer, .. } => *scope_pointer = pointer,
			_ => panic!(),
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
