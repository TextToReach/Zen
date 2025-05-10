#![allow(dead_code)]

pub mod DefineVariable;
pub mod Print;
pub mod Repeat;
pub mod If;

pub mod Parsers {
	use std::fmt::Display;
	use std::rc::Rc;

	use crate::features::tokenizer::InstructionEnum;
	use crate::features::tokenizer::TokenData;
	use crate::features::tokenizer::TokenTable;
	use crate::library::Types::Object;
	use chumsky::prelude::*;
	use num::pow::Pow;

	use super::Print;
	use super::Repeat;
	use super::If;

	type ParserType1 = Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>>;
	type ParserType2 = Box<dyn Parser<TokenData, (ParserOutput, InstructionEnum), Error = Simple<TokenData>>>;

	#[derive(Debug, Clone)]
	pub struct ParserOutput {
		pub indent: bool, // Specifies if the parser requires the lines after it to be indented.
	}

	pub fn tab() -> Box<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>> {
		Box::new(just(TokenData::default(TokenTable::Tab)))
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
				WithoutIndentation(Print::parser()),
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
		pub fn evaluate(&self) -> Object {
			match self {
				Expression::Value(val) => *val.clone(),
				Expression::Add(lhs, rhs) => {
					let left = lhs.evaluate();
					let right = rhs.evaluate();
					left + right
				}
				Expression::Sub(lhs, rhs) => {
					let left = lhs.evaluate();
					let right = rhs.evaluate();
					left - right
				}
				Expression::Mul(lhs, rhs) => {
					let left = lhs.evaluate();
					let right = rhs.evaluate();
					left * right
				}
				Expression::Div(lhs, rhs) => {
					let left = lhs.evaluate();
					let right = rhs.evaluate();
					left / right
				}
				Expression::Mod(lhs, rhs) => {
					let left = lhs.evaluate();
					let right = rhs.evaluate();
					left % right
				}
				Expression::Pow(lhs, rhs) => {
					let left = lhs.evaluate();
					let right = rhs.evaluate();
					left.pow(right)
				}
				Expression::LessThan(lhs, rhs) => Object::from(lhs.evaluate() < rhs.evaluate()),
				Expression::GreaterThan(lhs, rhs) => Object::from(lhs.evaluate() > rhs.evaluate()),
				Expression::LessThanOrEqual(lhs, rhs) => Object::from(lhs.evaluate() <= rhs.evaluate()),
				Expression::GreaterThanOrEqual(lhs, rhs) => Object::from(lhs.evaluate() >= rhs.evaluate()),
				Expression::Equal(lhs, rhs) => {
					let left = lhs.evaluate();
					let right = rhs.evaluate();
					(left == right).into()
				}
				Expression::NotEqual(lhs, rhs) => {
					let left = lhs.evaluate();
					let right = rhs.evaluate();
					(left != right).into()
				}
			}
		}
		pub fn isTruthy(&self) -> bool {
			self.evaluate().isTruthy()
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
					|| x.token == TokenTable::NegativeNumberLiteral
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
            let atom = Rc::new(object().or(expr.delimited_by(paren_left, paren_right)));
            
            let mul_operator = just(TokenData::default(TokenTable::MathOperatorMultiply))
                .or(just(TokenData::default(TokenTable::MathOperatorDivide)))
                .or(just(TokenData::default(TokenTable::MathOperatorMod)));

            let add_operator = just(TokenData::default(TokenTable::MathOperatorAdd))
                .or(just(TokenData::default(TokenTable::MathOperatorSubtract)));

			let comparison_operator = choice([
				just::<_, _, Simple<TokenData>>(TokenData::default(TokenTable::ComparisonOperatorEqual)),
				just::<_, _, Simple<TokenData>>(TokenData::default(TokenTable::ComparisonOperatorGreaterThan)),
				just::<_, _, Simple<TokenData>>(TokenData::default(TokenTable::ComparisonOperatorGreaterThanOrEqual)),
				just::<_, _, Simple<TokenData>>(TokenData::default(TokenTable::ComparisonOperatorLessThan)),
				just::<_, _, Simple<TokenData>>(TokenData::default(TokenTable::ComparisonOperatorLessThanOrEqual)),
			]);

            let mul = atom
                .clone()
                .then(mul_operator.clone().then(atom).repeated())
                .foldl(|lhs, (op, rhs)| op.toOp()(Box::new(lhs), Box::new(rhs)));

            let add = mul
                .clone()
                .then(add_operator.clone().then(mul).repeated())
                .foldl(|lhs, (op, rhs)| op.toOp()(Box::new(lhs), Box::new(rhs)));

			let comparison = add.clone()
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

	pub fn number() -> Box<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>> {
		Box::new(filter(|x: &TokenData| x.token == TokenTable::NumberLiteral))
	}

	pub fn string() -> Box<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>> {
		Box::new(filter(|x: &TokenData| x.token == TokenTable::StringLiteral))
	}

	pub fn boolean() -> Box<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>> {
		Box::new(filter(|x: &TokenData| x.token == TokenTable::BooleanLiteral))
	}
}
