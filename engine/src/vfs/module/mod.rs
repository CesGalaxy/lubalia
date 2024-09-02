use lubalang::{lexer::lexer, parser::parser, syntax::statement::list::StatementList};

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub items: StatementList,
}

impl Module {
    pub fn new(name: String, items: StatementList) -> Module {
        Module {
            name,
            items,
        }
    }

    pub fn read(source_code: String) -> Option<StatementList> {
        if let Ok(tokens) = lexer(source_code) {
            if let Ok(Some(ast)) = parser(tokens) {
                return Some(ast);
            }
        }

        None
    }
}
