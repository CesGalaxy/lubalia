use lubalang::{lexer::lexer, parser::parser, syntax::node::Node};

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub items: Vec<Node>,
}

impl Module {
    pub fn new(name: String, items: Vec<Node>) -> Module {
        Module {
            name,
            items,
        }
    }

    pub fn read(source_code: String) -> Option<Vec<Node>> {
        if let Ok(tokens) = lexer(source_code) {
            if let Ok(ast) = parser(tokens) {
                return Some(ast);
            }
        }

        None
    }
}
