use super::Parsers;
use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{TokenData, InstructionEnum, TokenTable},
};
use chumsky::prelude::*;

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = just(TokenTable::KeywordSürekliTekrarla.asTokenData())
		.map(|a| InstructionEnum::WhileTrue { scope_pointer: 0 });

	return Box::new(out);
}
