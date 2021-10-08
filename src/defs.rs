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
        // Number(String),
        OperatorOrSthIdk(String),
    }

    pub mod parse_tree {
        #[derive(Debug)]
        pub enum Typs <'a>{
            ChildNode(&'a Node<'a>),
            Atom(String)
            
        }
        
        #[derive(Debug)]
        pub struct Node<'a> {
            pub children: Vec<Typs<'a>>,
        }
    }

}
