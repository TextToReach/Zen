use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{InstructionEnum, TokenData, TokenTable},
};
use chumsky::prelude::*;

use super::Parsers::{self, Expression, parens};

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = just(TokenTable::KeywordFonksiyon.asTokenData())
		.ignore_then(Parsers::identifier())
		.then(
			just(TokenTable::LPAREN.asTokenData())
				.ignore_then(Parsers::parameter().separated_by(just(TokenTable::Comma.asTokenData())).allow_trailing())
				.then_ignore(just(TokenTable::RPAREN.asTokenData())),
		)
		.map(|(x, y)| InstructionEnum::Function {
			name: x.asIdentifier(),
			scope_pointer: 0,
			args: y,
		});

	return Box::new(out);
}
