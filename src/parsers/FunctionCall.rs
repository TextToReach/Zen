use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{InstructionEnum, TokenData, TokenTable},
};
use chumsky::prelude::*;

use super::Parsers::{self, Expression};

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = Parsers::identifier()
		.then_ignore(just(TokenTable::EmptyParens.asTokenData()))
		.map(|x| InstructionEnum::CallFunction { name: x.asIdentifier(), args: vec![] });

	return Box::new(out);
}