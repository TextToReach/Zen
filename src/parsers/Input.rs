use crate::{
	features::tokenizer::{AssignmentMethod, ExpressionOrYieldInstruction, InstructionEnum, TokenData, TokenTable, YieldInstructionEnum}, Debug, Print, PrintVec
};
use chumsky::prelude::*;

use super::Parsers::{self, assignment_operator, main_types};

pub fn parser() -> Box<dyn Parser<TokenData, YieldInstructionEnum, Error = Simple<TokenData>>> {
	let out = just(TokenTable::KeywordGirdi.asTokenData())
		.ignore_then(
			just(TokenTable::LPAREN.asTokenData())
			.ignore_then(main_types())
			.then_ignore(just(TokenTable::RPAREN.asTokenData()))
			.or_not()
		).then(Parsers::expression())
		.map(|(_type, quote)| YieldInstructionEnum::Input{ _type, quote });

	return Box::new(out);
}