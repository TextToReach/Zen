use super::super::Parsers::{self, assignment_operator};
use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{AssignmentMethod, Atom, InstructionEnum, TokenData, TokenTable},
};
use crate::{features::tokenizer::YieldInstructionEnum, parsers::FunctionCall};
use chumsky::prelude::*;

pub fn parser() -> Box<dyn Parser<TokenData, YieldInstructionEnum, Error = Simple<TokenData>>> {
	let out = filter(|x: &TokenData| x.token == TokenTable::Identifier)
		.then_ignore(just(TokenTable::LCRBRACKET.asTokenData()))
		.then(Parsers::atomic())
		.then_ignore(just(TokenTable::RCRBRACKET.asTokenData()))
		.map(|(x, y)| YieldInstructionEnum::Index(x.asIdentifier(), Box::new(y.into())));

	return Box::new(out);
}
