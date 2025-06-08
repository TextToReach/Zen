use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{AssignmentMethod, Atom, InstructionEnum, TokenData, TokenTable, YieldInstructionEnum},
	library::Types::RandomizerType,
};
use chumsky::prelude::*;

use super::super::Parsers::{self, assignment_operator, main_types, random_variants};

pub fn parser() -> Box<dyn Parser<TokenData, YieldInstructionEnum, Error = Simple<TokenData>>> {
	let out = just(TokenTable::KeywordRastgele.asTokenData())
		.ignore_then(random_variants())
		.then(
			Parsers::atomic()
				.then_ignore(just(TokenTable::Comma.asTokenData()))
				.then(Parsers::atomic())
				.or_not(),
		)
		.map(|(_type, span)| YieldInstructionEnum::Random { method: _type, span });

	return Box::new(out);
}
