use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{InstructionEnum, TokenData, TokenTable},
};
use chumsky::prelude::*;

use super::Parsers::{self, Expression};

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = just(TokenTable::KeywordFonksiyon.asTokenData())
		.ignore_then(Parsers::identifier())
		.then_ignore(just(TokenTable::EmptyParens.asTokenData()))
		.map(|x| InstructionEnum::Function { name: x.asIdentifier(), scope_pointer: 0, args: vec![] });

	return Box::new(out);
}