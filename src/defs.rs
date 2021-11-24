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
        Stringy(String),
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

    // i have no idea what i am doing
    pub mod abstract_syntax_tree {
        #[derive(Debug)]
        pub struct FunctionNode {
            pub id: String,
            pub body: Vec<AstNode>,
            // pub arguments: Vec<?>,
        }

        #[derive(Debug)]
        pub enum Types {
            Integer,
            Float,
            Boolean,
            Stringy,
        }

        #[derive(Debug)]
        pub struct DeclarationNode {
            pub id: String,
            pub rhs: ArithmeticNode,
            pub typ: Types,
        }

        // #[derive(Debug)]
        // pub struct ExpressionNode{
        //     pub child:
        // }
        #[derive(Debug)]
        pub enum ArithmeticStuff {
            ArithmeticNode,
            ConstantNode,
            Variable,
        }

        #[derive(Debug)]
        pub enum ArithmeticOperation {
            Add,
            Sub,
            Mul,
            Div,
        }

        #[derive(Debug)]
        pub enum Literals {
            Integer(i64),
            Float(f64),
            Boolean(bool),
            Stringy(String),
        }

        #[derive(Debug)]
        pub struct ArithmeticNode {
            pub lhs: ArithmeticStuff,
            pub rhs: Option<ArithmeticStuff>,
            pub operation: Option<ArithmeticOperation>,
        }

        #[derive(Debug)]
        pub struct Variable {
            pub id: String,
        }

        #[derive(Debug)]
        pub struct ConstantNode {
            pub value: Literals,
        }

        #[derive(Debug)]
        pub struct IfNode {
            condition: ArithmeticNode,
            body: Vec<AstNode>,
        }

        #[derive(Debug)]
        pub enum AstNode {
            FunctionNode(FunctionNode),
            DeclarationNode(DeclarationNode),
        }
    }
}
