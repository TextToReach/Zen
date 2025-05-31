use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{InstructionEnum, TokenData, TokenTable},
};
use chumsky::prelude::*;

use super::Parsers::{self, Expression};

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = Parsers::identifier()
		.then_ignore(just(TokenTable::LPAREN.asTokenData()))
		.then(Parsers::value().separated_by(just(TokenTable::Comma.asTokenData())).allow_trailing())
		.then_ignore(just(TokenTable::RPAREN.asTokenData()))
		.map(|(name, args)| InstructionEnum::CallFunction {
			name: name.asIdentifier(),
			args: args,
		});

	return Box::new(out);
}
