use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{InstructionEnum, TokenData, TokenTable},
};
use chumsky::prelude::*;

use super::Parsers::{self, Expression};

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = just(TokenTable::KeywordDeğilseVe.asTokenData())
		.ignore_then(Parsers::value())
		.then_ignore(just(TokenTable::Keywordİse.asTokenData()))
		.map(|x| InstructionEnum::ElifBlock {
			condition: x,
			scope_pointer: 0,
		});

	return Box::new(out);
}
