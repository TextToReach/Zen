use super::Parsers;
use crate::{
	Debug, Print, PrintVec,
	features::tokenizer::{InstructionEnum, TokenData, TokenTable},
};
use chumsky::prelude::*;

pub fn parser() -> Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>> {
	let out = Parsers::identifier() // a
		.then_ignore(just(TokenTable::Keywordİçinde.asTokenData())) // içinde
		.then(Parsers::value().then_ignore(just(TokenTable::KeywordArtarak.asTokenData())).or_not()) // 2 artarak
		.then_ignore(just(TokenTable::KeywordDolan.asTokenData())) // dolan
		.then_ignore(just(TokenTable::Colon.asTokenData())) // :
		.then(Parsers::identifier()) // x
		.map(|((name, step), varname)| InstructionEnum::ForIn { name: name.asIdentifier(), step: step, scope_pointer: 0, varname: varname.asIdentifier() });
	return Box::new(out);
}
