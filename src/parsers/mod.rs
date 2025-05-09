#![allow(dead_code)]

pub mod Print;
pub mod Repeat;
pub mod DefineVariable;

pub mod Parsers {
	use std::fmt::Display;

use crate::features::tokenizer::TokenData;
	use crate::features::tokenizer::InstructionEnum;
    use crate::features::tokenizer::TokenTable;
    use crate::library::Types::Object;
	use chumsky::prelude::*;
use num::pow::Pow;

	use super::Print;
	use super::Repeat;

    type ParserType1 = Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>>;
    type ParserType2 = Box<dyn Parser<TokenData, (ParserOutput, InstructionEnum), Error = Simple<TokenData>>>;
    
    #[derive(Debug, Clone)]
    pub struct ParserOutput {
        pub indent: bool // Specifies if the parser requires the lines after it to be indented.
    }

    pub fn tab() -> Box<dyn Parser<TokenData, TokenData, Error = Simple<TokenData>>> {
        Box::new(just(TokenData::default(TokenTable::Tab)))
    }

    pub fn WithIndentation(inp: ParserType1) -> ParserType2 {
        Box::new(inp.map(|x| (
            ParserOutput { indent: true },
            x
        )))
    }
    
    pub fn WithoutIndentation(inp: ParserType1) -> ParserType2 {
        Box::new(inp.map(|x| (
            ParserOutput { indent: false },
            x
        )))
    }

    pub fn parser() -> Box<dyn Parser<TokenData, (ParserOutput, InstructionEnum), Error = Simple<TokenData>>>{
        Box::new(recursive(|instr_parser|
            choice([
                WithIndentation(Repeat::parser()),
                WithoutIndentation(Print::parser())
            ])
        ))
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
                Expression::LessThan(lhs, rhs) => todo!(),
                Expression::GreaterThan(lhs, rhs) => todo!(),
                Expression::LessThanOrEqual(lhs, rhs) => todo!(),
                Expression::GreaterThanOrEqual(lhs, rhs) => todo!(),
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
        Box::new(filter(|x: &TokenData| {
            x.token == TokenTable::StringLiteral || x.token == TokenTable::NumberLiteral || x.token == TokenTable::BooleanLiteral || x.token == TokenTable::Identifier
        }).map(|x| Expression::Value(Box::new(x.asObject()))))
    }

    pub fn expression() -> impl Parser<TokenData, Expression, Error = Simple<TokenData>> {
        // recursive(|expr| {
        let (lparen, rparen) = parens();
        let obj = filter(|x: &TokenData| {
            x.token == TokenTable::StringLiteral || x.token == TokenTable::NumberLiteral || x.token == TokenTable::BooleanLiteral || x.token == TokenTable::Identifier
        }).map(|x| Expression::Value(Box::new(x.asObject())));
        
        let mul_operator = just(TokenData::default(TokenTable::OperatorMultiply)).or(just(TokenData::default(TokenTable::OperatorDivide))).or(just(TokenData::default(TokenTable::OperatorMod)));
        let add_operator = just::<_, _, Simple<TokenData>>(TokenData::default(TokenTable::OperatorAdd))
            .or(just::<_, _, Simple<TokenData>>(TokenData::default(TokenTable::OperatorSubtract)));
        let mul = obj.clone()
            .then(
                mul_operator
                    .clone()
                    .then(obj.clone())
                    .repeated()
            ).foldl(|lhs, (op, rhs)| {
                op.toOp()(Box::new(lhs), Box::new(rhs))
            });
        let add = mul.clone()
            .then(
                add_operator
                    .clone()
                    .then(mul)
                    .repeated()
            ).foldl(|lhs, (op, rhs)| {
                op.toOp()(Box::new(lhs), Box::new(rhs))
            });

        

        add
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

}
