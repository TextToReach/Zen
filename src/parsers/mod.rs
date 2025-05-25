#![allow(dead_code)]

pub mod Break;
pub mod Continue;
pub mod Define;
pub mod Elif;
pub mod Else;
pub mod Function;
pub mod FunctionCall;
pub mod If;
pub mod Print;
pub mod Repeat;
pub mod WhileTrue;
pub mod For;

pub mod Parsers {
	use super::{Break, Continue, Define, Elif, Else, For, Function, FunctionCall, If, Print, Repeat, WhileTrue};
	use crate::features::tokenizer::{AssignmentMethod, InstructionEnum, TokenData, TokenTable};
	use crate::library::Types::{Object, ParameterData};
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
				WithoutIndentation(FunctionCall::parser()),
				WithoutIndentation(Print::parser()),
				WithoutIndentation(Define::parser()),
				WithoutIndentation(Break::parser()),
				WithoutIndentation(Continue::parser()),
			])
		}))
	}

	#[derive(Debug, Clone, PartialEq)]
	pub enum Expression {
		Value(Box<Object>),
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

	impl Expression {
		pub fn truthy() -> Self {
			Self::Value(Box::new(true.into()))
		}
		pub fn falsy() -> Self {
			Self::Value(Box::new(false.into()))
		}

		pub fn evaluate(&self, currentScope: usize, manager: &mut ScopeManager) -> Object {
			match self {
				Expression::Value(val) => {
					if let Object::Variable(name) = *val.clone() {
						manager.get_var(currentScope, name).unwrap()
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
			}
		}
	}

	pub fn object() -> Box<dyn Parser<TokenData, Expression, Error = Simple<TokenData>>> {
		Box::new(
			filter(|x: &TokenData| {
				x.token == TokenTable::StringLiteral
					|| x.token == TokenTable::NumberLiteral
					|| x.token == TokenTable::BooleanLiteral
					|| x.token == TokenTable::Identifier
			})
			.map(|x| Expression::Value(Box::new(x.asObject()))),
		)
	}

	// FIXME: Fix parantheses support.
	pub fn expression() -> impl Parser<TokenData, Expression, Error = Simple<TokenData>> {
		let (paren_left, paren_right) = parens();

		let expr = recursive(|expr| {
			let atom = Rc::new(
				just(TokenTable::MathOperatorSubtract.asTokenData())
					.then(object())
					.map(|(_, obj)| Expression::Sub(Box::new(Expression::Value(Box::new(0f64.into()))), Box::new(obj)))
					.or(object())
					.or(expr.delimited_by(paren_left, paren_right))
			);

			let mul_operator = just(TokenTable::MathOperatorMultiply.asTokenData())
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
				.then(mul_operator.clone().then(atom).repeated())
				.foldl(|lhs, (op, rhs)| op.toOp()(Box::new(lhs), Box::new(rhs)));

			let add = mul
				.clone()
				.then(add_operator.clone().then(mul).repeated())
				.foldl(|lhs, (op, rhs)| op.toOp()(Box::new(lhs), Box::new(rhs)));

			let comparison = add
				.clone()
				.then(comparison_operator.then(add).repeated())
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
				.then(
					just(TokenTable::AssignmentOperatorSet.asTokenData())
					.ignore_then(expression())
					.or_not()
				)
				.map(|((name, type_), default)| ParameterData {
					name: name.asIdentifier(),
					data_type: Some(type_),
					default_value: default,
				}),
		)
	}
}
