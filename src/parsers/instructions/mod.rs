pub mod yazdir;
pub mod Kit {
    use chumsky::prelude::*;
    use crate::library::Types::Object;

    use super::yazdir;

    pub fn parser<'a>() -> Box<dyn Parser<char, Object, Error = Simple<char>> + 'a> {
        Box::new(choice([
            yazdir::parser()
        ]))
    }
}