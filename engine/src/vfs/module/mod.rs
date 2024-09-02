use lubalang::{lexer::lexer, parser::parser, syntax::node::Node};

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

    pub fn parse(source_code: String) -> Vec<Node> {
        let tokens = lexer(source_code).unwrap();
        let ast = parser(tokens).unwrap();

        ast
    }
}
