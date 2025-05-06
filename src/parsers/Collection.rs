use crate::features::tokenizer::{TokenData, TokenTable};
use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
	Value(Box<TokenData>),
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

use Expression::*;

pub fn object() -> Box<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>> {
	Box::new(filter(|x: &TokenData| {
		x.token == TokenTable::StringLiteral || x.token == TokenTable::NumberLiteral || x.token == TokenTable::BooleanLiteral || x.token == TokenTable::Identifier
	}))
}

pub fn expression() -> impl Parser<TokenData, Expression, Error = Simple<TokenData>> {
	// recursive(|expr| {
		let (lparen, rparen) = parens();
		let obj = filter(|x: &TokenData| {
			x.token == TokenTable::StringLiteral || x.token == TokenTable::NumberLiteral || x.token == TokenTable::BooleanLiteral || x.token == TokenTable::Identifier
		}).map(|x| Value(Box::new(x)));
		
		let mul_operator = .()
		let mul = obj.clone()
			.then(
				operationPriority1
					.clone()
					.then(obj.clone())
					.repeated()
			)
			.map(|(left, ops)| {
				ops.into_iter().fold(left, |lhs, (op, rhs)| {
					match op.slice.as_str() {
						"*" => Mul(Box::new(lhs), Box::new(rhs)),
						"/" => Div(Box::new(lhs), Box::new(rhs)),
						"%" => Mod(Box::new(lhs), Box::new(rhs)),
						_ => unreachable!()
					}
				})
		});
		
		let add = mul.clone()
			.then(
				operationPriority2
					.then(mul.clone())
					.repeated()
			)
			.map(|(lhs, ops)| {
				ops.into_iter().fold(lhs, |lhs, (op, rhs)| {
					match op.slice.as_str() {
						"+" => Add(Box::new(lhs), Box::new(rhs)),
						"-" => Sub(Box::new(lhs), Box::new(rhs)),
						_ => unreachable!()
					}
				})
		});

		add
	// });

	
	/*
	
	recursive(move |expression| {
		let currentScope = currentScope_clone.clone();
		// Parse a value: number, variable, or parenthesized expression
		let value = choice((
			Object::parser(currentScope.clone()).map(|obj| Expression::Value(Box::new(obj))), // The whole object parser
			Variable::parser(currentScope.clone()).map(|obj| Expression::Value(Box::new(obj))),
			expression.clone().delimited_by(just('('), just(')')), // parser from the previous iteration
		))
		.boxed();

		// Operator precedence: *, /, % > +, - > comparisons
		let op_mul = just('*')
			.to(Expression::Mul as fn(_, _) -> _)
			.or(just('/').to(Expression::Div as fn(_, _) -> _))
			.or(just('%').to(Expression::Mod as fn(_, _) -> _))
			.or(just('^').to(Expression::Pow as fn(_, _) -> _))
			.padded_by(whitespace());
		let op_add = just('+')
			.to(Expression::Add as fn(_, _) -> _)
			.or(just('-').to(Expression::Sub as fn(_, _) -> _))
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
	
	 */
}

pub fn parens() -> (Box<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>>, Box<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>>) {
	(
		Box::new(filter(|x: &TokenData| x.token == TokenTable::LPAREN)),
		Box::new(filter(|x: &TokenData| x.token == TokenTable::RPAREN))
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
