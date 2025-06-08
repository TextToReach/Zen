#![allow(dead_code)]

pub mod Break;
pub mod Continue;
pub mod Define;
pub mod Elif;
pub mod Else;
pub mod For;
pub mod ForIn;
pub mod Function;
pub mod FunctionCall;
pub mod Yield;
use Yield::*;
pub mod If;
pub mod Print;
pub mod Repeat;
pub mod Return;
pub mod Type;
pub mod Wait;
pub mod WhileTrue;

pub mod Parsers {
	use super::{
		Break, Continue, Define, Elif, Else, For, ForIn, Function, FunctionCall, FunctionCallYield, If, Input, Print, Random, Repeat, Return, Type,
		Wait, WhileTrue, Index
	};
	use crate::features::tokenizer::{AssignmentMethod, Atom, InstructionEnum, TokenData, TokenTable, YieldInstructionEnum};
	use crate::library::Types::{Object, ParameterData, RandomizerType};
	use crate::util::ScopeManager::ScopeManager;
	use chumsky::prelude::*;
	use num::pow::Pow;
	use std::fmt::Display;
	use std::rc::Rc;

	type ParserType1 = Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>>;
	type ParserType2 = Box<dyn Parser<TokenData, (ParserOutput, InstructionEnum), Error = Simple<TokenData>>>;

	#[derive(Debug, Clone)]
	pub struct ParserOutput {
		pub indent: bool, // Specifies if the parser requires the lines after it to be indented.
	}

	pub fn tab() -> Box<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>> {
		Box::new(just(TokenTable::Tab.asTokenData()))
	}

	pub fn WithIndentation(inp: ParserType1) -> ParserType2 {
		Box::new(inp.map(|x| (ParserOutput { indent: true }, x)))
	}

	pub fn WithoutIndentation(inp: ParserType1) -> ParserType2 {
		Box::new(inp.map(|x| (ParserOutput { indent: false }, x)))
	}

	pub fn parser() -> Box<dyn Parser<TokenData, (ParserOutput, InstructionEnum), Error = Simple<TokenData>>> {
		Box::new(recursive(|instr_parser| {
			choice([
				WithIndentation(Repeat::parser()),
				WithIndentation(If::parser()),
				WithIndentation(Elif::parser()),
				WithIndentation(Else::parser()),
				WithIndentation(WhileTrue::parser()),
				WithIndentation(Function::parser()),
				WithIndentation(For::parser()),
				WithIndentation(ForIn::parser()),
				WithoutIndentation(FunctionCall::parser()),
				WithoutIndentation(Print::parser()),
				WithoutIndentation(Define::parser()),
				WithoutIndentation(Break::parser()),
				WithoutIndentation(Continue::parser()),
				WithoutIndentation(Return::parser()),
				WithoutIndentation(Type::parser()),
				WithoutIndentation(Wait::parser()),
			])
		}))
	}

	pub fn yield_instruction_parser() -> Box<dyn Parser<TokenData, YieldInstructionEnum, Error = Simple<TokenData>>> {
		Box::new(recursive(|instr_parser| {
			choice([Input::parser(), Random::parser(), FunctionCallYield::parser(), Index::parser()])
		}))
	}

	pub fn value() -> Box<dyn Parser<TokenData, Atom, Error = Simple<TokenData>>> {
		Box::new(
			yield_instruction_parser()
				.map(|x| Atom::YieldInstruction(x))
				.or(atomic().map(|x| Atom::Expression(x))),
		)
	}

	#[derive(Debug, Clone, PartialEq)]
	pub enum Expression {
		Value(Box<Object>),
		Not(Box<Expression>),
		Add(Box<Expression>, Box<Expression>),
		Sub(Box<Expression>, Box<Expression>),
		Mul(Box<Expression>, Box<Expression>),
		Div(Box<Expression>, Box<Expression>),
		Mod(Box<Expression>, Box<Expression>),
		Pow(Box<Expression>, Box<Expression>),

		LessThan(Box<Expression>, Box<Expression>),
		GreaterThan(Box<Expression>, Box<Expression>),
		LessThanOrEqual(Box<Expression>, Box<Expression>),
		GreaterThanOrEqual(Box<Expression>, Box<Expression>),
		Equal(Box<Expression>, Box<Expression>),
		NotEqual(Box<Expression>, Box<Expression>),
	}

	impl From<f64> for Expression {
		fn from(value: f64) -> Self {
			Expression::Value(Box::new(Object::from(value)))
		}
	}
	impl From<String> for Expression {
		fn from(value: String) -> Self {
			Expression::Value(Box::new(Object::from(value)))
		}
	}
	impl From<&str> for Expression {
		fn from(value: &str) -> Self {
			Expression::Value(Box::new(Object::from(value.to_owned())))
		}
	}
	impl From<bool> for Expression {
		fn from(value: bool) -> Self {
			Expression::Value(Box::new(Object::from(value)))
		}
	}
	impl From<Object> for Expression {
		fn from(value: Object) -> Self {
			Expression::Value(Box::new(value))
		}
	}

	impl Expression {
		pub fn truthy() -> Self {
			Self::Value(Box::new(true.into()))
		}
		pub fn falsy() -> Self {
			Self::Value(Box::new(false.into()))
		}

		pub fn evaluate(&self, currentScope: usize, manager: &mut ScopeManager) -> Object {
			match self {
				Expression::Not(inner) => {
					let inner_value = inner.evaluate(currentScope, manager);
					Object::from(!inner_value.isTruthy())
				}
				Expression::Value(val) => {
					if let Object::Variable(name) = *val.clone() {
						manager
							.get_var(currentScope, name.clone())
							.unwrap_or_else(|| panic!("Variable {} not found in scope {}", name, currentScope))
					} else {
						*val.clone()
					}
				}
				Expression::Add(lhs, rhs) => {
					let left = lhs.evaluate(currentScope, manager);
					let right = rhs.evaluate(currentScope, manager);
					left + right
				}
				Expression::Sub(lhs, rhs) => {
					let left = lhs.evaluate(currentScope, manager);
					let right = rhs.evaluate(currentScope, manager);
					left - right
				}
				Expression::Mul(lhs, rhs) => {
					let left = lhs.evaluate(currentScope, manager);
					let right = rhs.evaluate(currentScope, manager);
					left * right
				}
				Expression::Div(lhs, rhs) => {
					let left = lhs.evaluate(currentScope, manager);
					let right = rhs.evaluate(currentScope, manager);
					left / right
				}
				Expression::Mod(lhs, rhs) => {
					let left = lhs.evaluate(currentScope, manager);
					let right = rhs.evaluate(currentScope, manager);
					left % right
				}
				Expression::Pow(lhs, rhs) => {
					let left = lhs.evaluate(currentScope, manager);
					let right = rhs.evaluate(currentScope, manager);
					left.pow(right)
				}
				Expression::LessThan(lhs, rhs) => Object::from(lhs.evaluate(currentScope, manager) < rhs.evaluate(currentScope, manager)),
				Expression::GreaterThan(lhs, rhs) => Object::from(lhs.evaluate(currentScope, manager) > rhs.evaluate(currentScope, manager)),
				Expression::LessThanOrEqual(lhs, rhs) => Object::from(lhs.evaluate(currentScope, manager) <= rhs.evaluate(currentScope, manager)),
				Expression::GreaterThanOrEqual(lhs, rhs) => Object::from(lhs.evaluate(currentScope, manager) >= rhs.evaluate(currentScope, manager)),
				Expression::Equal(lhs, rhs) => {
					let left = lhs.evaluate(currentScope, manager);
					let right = rhs.evaluate(currentScope, manager);
					(left == right).into()
				}
				Expression::NotEqual(lhs, rhs) => {
					let left = lhs.evaluate(currentScope, manager);
					let right = rhs.evaluate(currentScope, manager);
					(left != right).into()
				}
			}
		}
		pub fn isTruthy(&self, currentScope: usize, manager: &mut ScopeManager) -> bool {
			self.evaluate(currentScope, manager).isTruthy()
		}
	}

	impl Display for Expression {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
				Expression::Value(val) => write!(f, "{}", val),
				Expression::Add(lhs, rhs) => write!(f, "({} + {})", lhs, rhs),
				Expression::Sub(lhs, rhs) => write!(f, "({} - {})", lhs, rhs),
				Expression::Mul(lhs, rhs) => write!(f, "({} * {})", lhs, rhs),
				Expression::Div(lhs, rhs) => write!(f, "({} / {})", lhs, rhs),
				Expression::Mod(lhs, rhs) => write!(f, "({} % {})", lhs, rhs),
				Expression::Pow(lhs, rhs) => write!(f, "({} ^ {})", lhs, rhs),
				Expression::LessThan(lhs, rhs) => write!(f, "({} < {})", lhs, rhs),
				Expression::GreaterThan(lhs, rhs) => write!(f, "({} > {})", lhs, rhs),
				Expression::LessThanOrEqual(lhs, rhs) => write!(f, "({} <= {})", lhs, rhs),
				Expression::GreaterThanOrEqual(lhs, rhs) => write!(f, "({} >= {})", lhs, rhs),
				Expression::Equal(lhs, rhs) => write!(f, "({} == {})", lhs, rhs),
				Expression::NotEqual(lhs, rhs) => write!(f, "({} != {})", lhs, rhs),
				Expression::Not(inner) => write!(f, "(!{})", inner),
			}
		}
	}

	pub fn object() -> Box<dyn Parser<TokenData, Expression, Error = Simple<TokenData>>> {
		Box::new(
			recursive(|parser| {
				filter(|x: &TokenData| {
					x.token == TokenTable::StringLiteral
						|| x.token == TokenTable::NumberLiteral
						|| x.token == TokenTable::BooleanLiteral
						|| x.token == TokenTable::Identifier
				})
				.map(|x| x.asObject())
				.or(just(TokenTable::LSQBRACKET.asTokenData())
					.ignore_then(parser.clone().separated_by(just(TokenTable::Comma.asTokenData())))
					.then_ignore(just(TokenTable::RSQBRACKET.asTokenData()))
					.map(|exprs| Object::from(exprs)))
			})
			.map(|x| x.into()),
		)
	}

	// FIXME: Fix parantheses support.
	pub fn atomic() -> impl Parser<TokenData, Expression, Error = Simple<TokenData>> {
		let (paren_left, paren_right) = parens();

		let expr = recursive(|expr| {
			let not_operator = just(TokenTable::ExclamationMark.asTokenData());

			let atom = Rc::new(
				not_operator
					.then(expr.clone())
					.map(|(_, inner)| Expression::Not(Box::new(inner)))
					.or(just(TokenTable::MathOperatorSubtract.asTokenData())
						.then(object())
						.map(|(_, obj)| Expression::Sub(Box::new(Expression::from(0f64)), Box::new(obj))))
					.or(object())
					.or(expr.delimited_by(paren_left.clone(), paren_right.clone())),
			);

			let mul_operator = just(TokenTable::MathOperatorMultiply.asTokenData())
				.or(just(TokenTable::MathOperatorPower.asTokenData()))
				.or(just(TokenTable::MathOperatorDivide.asTokenData()))
				.or(just(TokenTable::MathOperatorMod.asTokenData()));

			let add_operator = just(TokenTable::MathOperatorAdd.asTokenData()).or(just(TokenTable::MathOperatorSubtract.asTokenData()));

			let comparison_operator = choice([
				just(TokenTable::ComparisonOperatorEqual.asTokenData()),
				just(TokenTable::ComparisonOperatorNotEqual.asTokenData()),
				just(TokenTable::ComparisonOperatorGreaterThan.asTokenData()),
				just(TokenTable::ComparisonOperatorGreaterThanOrEqual.asTokenData()),
				just(TokenTable::ComparisonOperatorLessThan.asTokenData()),
				just(TokenTable::ComparisonOperatorLessThanOrEqual.asTokenData()),
			]);

			let mul = atom
				.clone()
				.then(mul_operator.clone().then(atom.clone()).repeated())
				.foldl(|lhs, (op, rhs)| op.toOp()(Box::new(lhs), Box::new(rhs)));

			let add = mul
				.clone()
				.then(add_operator.clone().then(mul.clone()).repeated())
				.foldl(|lhs, (op, rhs)| op.toOp()(Box::new(lhs), Box::new(rhs)));

			let comparison = add
				.clone()
				.then(comparison_operator.then(add.clone()).repeated())
				.foldl(|lhs, (op, rhs)| op.toOp()(Box::new(lhs), Box::new(rhs)));

			comparison
		});

		expr
	}

	pub fn parens() -> (
		Rc<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>>,
		Rc<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>>,
	) {
		(
			Rc::new(filter(|x: &TokenData| x.token == TokenTable::LPAREN)),
			Rc::new(filter(|x: &TokenData| x.token == TokenTable::RPAREN)),
		)
	}

	pub fn assignment_operator() -> Box<dyn Parser<TokenData, AssignmentMethod, Error = Simple<TokenData>>> {
		Box::new(
			just(TokenTable::AssignmentOperatorSet.asTokenData())
				.to(AssignmentMethod::Set)
				.or(just(TokenTable::AssignmentOperatorAdd.asTokenData()).to(AssignmentMethod::Add))
				.or(just(TokenTable::AssignmentOperatorSubtract.asTokenData()).to(AssignmentMethod::Sub))
				.or(just(TokenTable::AssignmentOperatorMultiply.asTokenData()).to(AssignmentMethod::Mul))
				.or(just(TokenTable::AssignmentOperatorDivide.asTokenData()).to(AssignmentMethod::Div)),
		)
	}

	pub fn number() -> Box<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>> {
		Box::new(filter(|x: &TokenData| x.token == TokenTable::NumberLiteral))
	}

	pub fn string() -> Box<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>> {
		Box::new(filter(|x: &TokenData| x.token == TokenTable::StringLiteral))
	}

	pub fn boolean() -> Box<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>> {
		Box::new(filter(|x: &TokenData| x.token == TokenTable::BooleanLiteral))
	}

	pub fn identifier() -> Box<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>> {
		Box::new(filter(|x: &TokenData| x.token == TokenTable::Identifier))
	}

	pub fn main_types() -> Box<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>> {
		Box::new(
			just(TokenTable::KeywordMetin.asTokenData())
				.or(just(TokenTable::KeywordSayı.asTokenData()))
				.or(just(TokenTable::KeywordMantıksal.asTokenData())),
		)
	}

	pub fn parameter() -> Box<dyn Parser<TokenData, ParameterData, Error = Simple<TokenData>>> {
		Box::new(
			identifier()
				.then_ignore(just(TokenTable::Colon.asTokenData()))
				.then(main_types()) // TODO: Remove this and add support for object and such. For now this only supports the main types.
				.then(just(TokenTable::AssignmentOperatorSet.asTokenData()).ignore_then(atomic()).or_not())
				.map(|((name, type_), default)| ParameterData {
					name: name.asIdentifier(),
					data_type: Some(type_),
					default_value: default,
				}),
		)
	}

	pub fn random_variants() -> Box<dyn Parser<TokenData, RandomizerType, Error = Simple<TokenData>>> {
		Box::new(choice([
			just(TokenTable::KeywordSayı.asTokenData()).to(RandomizerType::Number).boxed(),
			just(TokenTable::KeywordHarf.asTokenData()).to(RandomizerType::Letter).boxed(),
			just(TokenTable::KeywordIhtimal.asTokenData())
				.ignore_then(
					just(TokenTable::LPAREN.asTokenData())
						.ignore_then(just(TokenTable::MathOperatorMod.asTokenData()))
						.ignore_then(atomic())
						.then_ignore(just(TokenTable::RPAREN.asTokenData()))
						.or_not(),
				)
				.map(|chance_opt| {
					let chance = chance_opt.unwrap_or_else(|| Expression::from(50f64));
					RandomizerType::Boolean { chance }
				})
				.boxed(),
		]))
	}
}
