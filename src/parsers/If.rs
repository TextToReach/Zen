use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{TokenData, InstructionEnum, TokenTable},
};
use chumsky::prelude::*;

use super::Parsers::{self, Expression};

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = just(TokenTable::KeywordEğer.asTokenData())
		.ignore_then(Parsers::value())
		.then_ignore(just(TokenTable::Keywordİse.asTokenData()))
		.map(|x| InstructionEnum::IfBlock { condition: x, scope_pointer: 0 });

	return Box::new(out);
}