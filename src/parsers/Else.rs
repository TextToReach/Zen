use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{InstructionEnum, TokenData, TokenTable},
};
use chumsky::prelude::*;

use super::Parsers::{self, Expression};

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = just(TokenTable::KeywordDeğilse.asTokenData()).map(|x| InstructionEnum::ElseBlock { scope_pointer: 0 });

	return Box::new(out);
}
