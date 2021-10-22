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
        OpenParenthesis,
        CloseParenthesis,
        If,
        Semicolon,
        Equals,
        Integer(i64),
        Float(f64),
        Boolean(bool),
        // Number(String),
        OperatorOrSthIdk(String),
    }

    // pub enum Operators {
    //     // +
    //     Addition,
    //     // -
    //     Subtraction,
    //     // *
    //     Multiplication,
    //     // /
    //     Division,
    // }

    #[derive(Debug)]
    pub enum StatementType {
        Declaration(Vec<Tostsken>),
        Conditional((Tostsken, Vec<Tostsken>, Vec<Tostsken>)),
        Other(Vec<Tostsken>),
    }

    pub mod parse_tree {
        #[derive(Debug)]
        pub struct Node {
            pub children: Vec<Node>,
            pub content: Option<String>,
            // pub id: String,
        }
    }
}
