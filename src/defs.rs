pub mod parse {

    #[derive(Debug)]
    pub enum Tostsken {
        // Statement(String),
        // Expression(String),
        // Function(String),
        Word(String),
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
