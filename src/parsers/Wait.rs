use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{AssignmentMethod, ExpOrInstr, InstructionEnum, TokenData, TokenTable, YieldInstructionEnum},
	library::Types::{RandomizerType, TimeUnit},
};
use chumsky::prelude::*;

use super::Parsers::{self, assignment_operator, main_types, random_variants};

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = Parsers::value()
		.then(
			just(TokenTable::KeywordSalise.asTokenData()).to(TimeUnit::Millisecond)
			.or(just(TokenTable::KeywordSaniye.asTokenData()).to(TimeUnit::Second))
			.or(just(TokenTable::KeywordDakika.asTokenData()).to(TimeUnit::Minute))
			.or(just(TokenTable::KeywordSaat.asTokenData()).to(TimeUnit::Hour))
			.or(just(TokenTable::KeywordGün.asTokenData()).to(TimeUnit::Day))
			.or(just(TokenTable::KeywordHafta.asTokenData()).to(TimeUnit::Week))
			.or(just(TokenTable::KeywordAy.asTokenData()).to(TimeUnit::Month))
			.or(just(TokenTable::KeywordYıl.asTokenData()).to(TimeUnit::Year)),
		)
		.then_ignore(just(TokenTable::KeywordBekle.asTokenData()))
		.map(|(amount, unit)| InstructionEnum::Wait { amount, unit });

	return Box::new(out);
}
