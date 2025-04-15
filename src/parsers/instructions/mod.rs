pub mod yazdir;
pub mod forloop1;

/// Instructions with a different name
pub mod Kit {
    use chumsky::prelude::*;
    use crate::library::Types::{Instruction, Object};

    use super::{forloop1, yazdir};

    pub fn parser<'a>() -> Box<dyn Parser<char, Instruction, Error = Simple<char>> + 'a> {
        Box::new(choice([
            yazdir::parser(),
            forloop1::parser(),
        ]))
    }
}
