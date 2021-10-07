pub mod parse_tree {
    use crate::defs::parse::Tostsken;
    use crate::defs::parse::parse_tree::*;


    fn actual_parser<'a>(tokens: Vec<Tostsken>) -> (Node<'a>, Vec<Tostsken>) {
        unimplemented!();
    }

    // impl<'a> Default for defs::parse::parse_tree::Node<'a> {
    //     fn default() -> Self {
    //         Self::new()
    //     }
    // }

    impl<'a> Default for Node<'a> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<'a> Node<'a> {
        pub fn new<'b>()->Node<'a> {
            Node{children: vec![],}
        }

        pub fn expand<'b>(){
            
        }
    }

    pub fn parse<'a>(tokens: Vec<Tostsken>) -> Node<'a>{
        let mut root = Node::new();

        // match &tokens[0] {
        //     Tostsken::OperatorOrSthIdk(op) => {
        //         if op.as_str() == "=" {
        //             root.children.push(Typs::Atom(op.to_string()));
        //         };
        //     },
        //     Tostsken::Word(word) => {
        //         root.children.push(
        //             Typs::Atom(word.to_string())
        //         );
        //     },
        // }
        root
    }
}