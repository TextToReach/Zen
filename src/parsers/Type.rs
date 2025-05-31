use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{TokenData, InstructionEnum, TokenTable},
};
use chumsky::prelude::*;

use super::Parsers::{self, Expression};

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = just(TokenTable::KeywordTip.asTokenData())
		.then(
			Parsers::value().separated_by(
				just(TokenTable::Comma.asTokenData())
			)
		).map(|(_, b)| InstructionEnum::Type(b));

	return Box::new(out);
}