use crate::{
	features::tokenizer::{AssignmentMethod, ExpressionOrYieldInstruction, InstructionEnum, TokenData, TokenTable}, Debug, Print, PrintVec
};
use chumsky::prelude::*;

use super::Parsers::{self, assignment_operator};

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = filter(|x: &TokenData| x.token == TokenTable::Identifier)
		.then(assignment_operator())
		.then(
			Parsers::expression().map(|x| ExpressionOrYieldInstruction::Expression(x))
			.or(Parsers::yield_instruction_parser().map(|x| ExpressionOrYieldInstruction::Instruction(x)))
		)
		.map(|((x, op), y)| InstructionEnum::VariableDeclaration(x.slice, y, op));

	return Box::new(out);
}