use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{InstructionEnum, TokenData, TokenTable},
};
use chumsky::prelude::*;

use super::Parsers::{self, Expression};

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = just(TokenTable::KeywordDöndür.asTokenData())
		.then(Parsers::expression())
		.map(|(_, expr)| InstructionEnum::Return(expr.into()));

	return Box::new(out);
}
