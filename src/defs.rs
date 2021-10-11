pub mod parse {

    #[derive(Debug, Clone)]
    pub enum Tostsken {
        // TODO: dont use String

        // Statement(String),
        // Expression(String),
        // Function(String),
        Word(String),
        WhiteSpace(String),
        FunctionToaster,
        Brace(String),
        Semicolon,
        Integer(i64),
        Float(f64),
        // Number(String),
        OperatorOrSthIdk(String),
    }

    #[derive(Debug)]
    pub enum StatementType {
        Declaration(Vec<Tostsken>),
        Other(Vec<Tostsken>),
    }

    pub mod parse_tree {
        #[derive(Debug)]
        pub struct Node {
            pub children: Vec<Node>,
            pub content: Option<String>,
        }
    }
}
