pub mod parse_tree {
    use crate::defs::parse::parse_tree::*;
    use crate::defs::parse::Tostsken;

    impl Default for Node {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Node {
        pub fn new() -> Node {
            Node {
                children: vec![],
                content: None,
            }
        }

        fn parse_funcs(&mut self, tokens: Vec<Tostsken>) {
            let mut all = vec![];
            let mut current = vec![];
            let mut depth = -1;
            for i in tokens {
                match i {
                    Tostsken::FunctionToaster => {
                        if depth == -1
                        // completely outside of functions
                        {
                            depth = 1; // we are now in the function
                            all.push(current);
                            current = vec![];
                        } else {
                            // we hit another function while being inside 
                            // increase depth
                            depth += 1;
                        }
                    }
                    Tostsken::Brace(ref op) => {
                        if depth > 0 && (op == ":}" || op == "}:") {
                            depth -= 1;
                        }
                    }
                    _ => (),
                };
                current.push(i);
                if depth == 0 {
                    all.push(current);
                    depth = -1;
                    current = vec![];
                }
            }
            if !current.is_empty() {
                all.push(current);
            }

            for child in all {
                let mut child_node = Node::new();
                if let Tostsken::FunctionToaster = child[0] {
                    child_node.parse_funcs(child);
                } else {
                    child_node.parse_statements(child);
                }
                self.children.push(child_node);
            }
        }

        // function level
        //   statements (x = 12, if asdas {: :}, function calls)

        fn parse_statements(&mut self, tokens: Vec<Tostsken>) {
            unimplemented!();
        }
    }

    fn actual_parser(tokens: Vec<Tostsken>) -> Node {
        // TODO: -> pass tokens[start..end] to actual_parser and tokens[end..] to actual_parser
        // TODO: find start & end of tokens
        // TODO: fix everythinf

        ///////////////////////////////////////////////////////
        //  IDEA: function for parsing specific features     //
        //   seems smarter                                   //
        //             a bit                                 //
        //                                                   //
        //                                                   //
        ///////////////////////////////////////////////////////
        let mut out = Node::new();
        out.parse_funcs(tokens);
        out
    }

    pub fn parse(tokens: Vec<Tostsken>) -> Node {
        actual_parser(tokens)
    }
}
