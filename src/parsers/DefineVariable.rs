use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{TokenData, InstructionEnum, TokenTable},
};
use chumsky::prelude::*;

use super::Parsers;

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = just(TokenData::default(TokenTable::KeywordYazdır))
		.then(Parsers::object().separated_by(just(TokenData::default(TokenTable::Comma))))
		.map(|(_, b)| InstructionEnum::Print(b));

	return Box::new(out);
}