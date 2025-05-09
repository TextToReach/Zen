use super::Parsers;
use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{TokenData, InstructionEnum, TokenTable},
};
use chumsky::prelude::*;

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = Parsers::number()
		.then_ignore(just(TokenData::default(TokenTable::KeywordNDefaTekrarla))) // The header
		.map(|a| InstructionEnum::Repeat(a.asNumberLiteral()) );

	return Box::new(out);
}
