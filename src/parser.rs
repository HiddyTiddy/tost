pub mod parse_tree {
    use crate::defs::parse::Tostsken;
    use crate::defs::parse::parse_tree::*;

    fn slice(tokens: Vec<Tostsken>) -> Vec<Vec<Tostsken>> {
        // TODO: find start & end of statement
    
        let mut out = vec![];

        let mut current = vec![];
        for i in tokens {
            current.push(i);
            if true /* TODO this condition kinda is the most important part */ {
                out.push(current);
                current = vec![];
            }
        }

        out

    }

    fn actual_parser<'a>(tokens: Vec<Tostsken>) -> Node<'a> {
        // TODO: -> pass tokens[start..end] to actual_parser and tokens[end..] to actual_parser
        // TODO: find start & end of tokens 
        // TODO: fix everythinf

        slice(tokens);

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
        pub fn new/*<'b>*/()->Node<'a> {
            Node{children: vec![],}
        }

        pub fn expand/*<'b>*/(&mut self){
            
        }
    }

    pub fn parse<'a>(tokens: Vec<Tostsken>) -> Node<'a>{
        // let root = Node::new();


        let root = actual_parser(tokens);
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