use crate::{
	features::tokenizer::{AssignmentMethod, ExpressionOrYieldInstruction, InstructionEnum, TokenData, TokenTable, YieldInstructionEnum}, library::Types::RandomizerType, Debug, Print, PrintVec
};
use chumsky::prelude::*;

use super::Parsers::{self, assignment_operator, main_types, random_variants};

pub fn parser() -> Box<dyn Parser<TokenData, YieldInstructionEnum, Error = Simple<TokenData>>> {
	let out = just(TokenTable::KeywordRastgele.asTokenData())
		.ignore_then(random_variants())
		.then(Parsers::expression())
		.then_ignore(just(TokenTable::Comma.asTokenData()))
		.then(Parsers::expression())
		.map(|((_type, from), to)| YieldInstructionEnum::Random{ 
			method: _type,
			from,
			to
		});

	return Box::new(out);
}