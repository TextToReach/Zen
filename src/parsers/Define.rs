use crate::{
	features::tokenizer::{AssignmentMethod, ExpOrInstr, InstructionEnum, TokenData, TokenTable}, Debug, Print, PrintVec
};
use chumsky::prelude::*;
use crate::parsers::FunctionCall;
use super::{FunctionCallYield, Parsers::{self, assignment_operator}};

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = filter(|x: &TokenData| x.token == TokenTable::Identifier)
		.then(assignment_operator())
		.then(Parsers::value())
		.map(|((x, op), y)| InstructionEnum::VariableDeclaration(x.slice, y, op));

	return Box::new(out);
}