use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{TokenData, InstructionEnum, TokenTable},
};
use chumsky::prelude::*;

use super::Parsers::{self, Expression};

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = just(TokenTable::KeywordYazdır.asTokenData())
		.then(
			Parsers::expression().separated_by(
				just(TokenTable::Comma.asTokenData())
			)
		).map(|(_, b)| InstructionEnum::Print(b));

	return Box::new(out);
}
