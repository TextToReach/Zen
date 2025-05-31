use super::{
	FunctionCallYield,
	Parsers::{self, assignment_operator},
};
use crate::parsers::FunctionCall;
use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{AssignmentMethod, ExpOrInstr, InstructionEnum, TokenData, TokenTable},
};
use chumsky::prelude::*;

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = filter(|x: &TokenData| x.token == TokenTable::Identifier)
		.then(assignment_operator())
		.then(Parsers::value())
		.map(|((x, op), y)| InstructionEnum::VariableDeclaration(x.slice, y, op));

	return Box::new(out);
}
