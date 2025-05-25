use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{InstructionEnum, TokenData, TokenTable},
};
use chumsky::prelude::*;

use super::Parsers::{self, Expression};

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = just(TokenTable::KeywordFonksiyon.asTokenData())
		.ignore_then(Parsers::identifier())
		.then(
			// just(TokenTable::EmptyParens.asTokenData()).to(vec![])
			// .or(
				Parsers::parameter().separated_by(just(TokenTable::Comma.asTokenData()))
			// )
		)
		.map(|(x, y)| {
			println!("Function: {}({:?})", x.asIdentifier(), y);
			InstructionEnum::Function { name: x.asIdentifier(), scope_pointer: 0, args: y }
		});

	return Box::new(out);
}