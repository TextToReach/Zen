use super::Parsers;
use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{TokenData, InstructionEnum, TokenTable},
};
use chumsky::prelude::*;

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = Parsers::value() // 0
		.then_ignore(just(TokenTable::Keywordİle.asTokenData())) // ile
		.then(Parsers::value()) // 10
		.then_ignore(just(TokenTable::KeywordAralığında.asTokenData()).or(just(TokenTable::KeywordArasında.asTokenData()))) // aralığında / arasında
		.then(
			Parsers::value().then_ignore(
				just(TokenTable::KeywordArtarak.asTokenData())
			).or_not()
		) // 2 artarak
		.then_ignore(just(TokenTable::Colon.asTokenData())) // :
		.then(Parsers::identifier())
		.map(|(((from, to), step), name)| InstructionEnum::For{ from, to, step, name: name.asIdentifier(), scope_pointer: 0 } );

	
	return Box::new(out);
}