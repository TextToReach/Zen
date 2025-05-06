pub mod Collection;
pub mod Print;
pub mod Repeat;
pub mod DefineVariable;

pub mod Parsers {
	use crate::features::tokenizer::TokenData;
	use crate::features::tokenizer::InstructionEnum;
	use chumsky::prelude::*;

	use super::Print;
	use super::Repeat;

    type ParserType1 = Box<dyn Parser<TokenData, InstructionEnum, Error = Simple<TokenData>>>;
    type ParserType2 = Box<dyn Parser<TokenData, (ParserOutput, InstructionEnum), Error = Simple<TokenData>>>;
    
    #[derive(Debug, Clone)]
    pub struct ParserOutput {
        pub indent: bool // Specifies if the parser requires the lines after it to be indented.
    }

    pub fn WithIndentation(inp: ParserType1) -> ParserType2 {
        Box::new(inp.map(|x| (
            ParserOutput { indent: true },
            x
        )))
    }
    
    pub fn WithoutIndentation(inp: ParserType1) -> ParserType2 {
        Box::new(inp.map(|x| (
            ParserOutput { indent: false },
            x
        )))
    }

    pub fn parser() -> Box<dyn Parser<TokenData, (ParserOutput, InstructionEnum), Error = Simple<TokenData>>>{
        Box::new(recursive(|instr_parser|
            choice([
                WithIndentation(Repeat::parser()),
                WithoutIndentation(Print::parser()),
            ])
        ))
    }
}
