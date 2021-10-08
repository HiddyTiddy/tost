pub mod parse_tree {
    use std::vec;

    use crate::defs::parse::parse_tree::*;
    use crate::defs::parse::Tostsken;
    use crate::parse::StatementType;

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
            //println!("\n   parse_funcs {:?}", tokens);
            // this function finds the bounds of functions on the Token ``Tostsken'' vector
            //
            // we expect to have multiple functions / non-function areas
            let mut all = vec![];
            let mut current = vec![];
            let mut depth = -1; // counter to account for nested blocks
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
                if !child.is_empty() {
                    let mut child_node = Node::new();
                    if let Tostsken::FunctionToaster = child[0] {
                        // TODO: only parse_funcs of function body oops
                        // currently only parses functions like `toaster main {:`
                        // instead of `toaster main <args> {:`
                        // actually just parses the function body
                        // but we never actually wanted to call the functions, right?
                        child_node.parse_funcs(find_function_body(child));
                    } else {
                        child_node.parse_statements(child);
                    }
                    self.children.push(child_node);
                }
            }
        }

        // function level
        //   statements (x = 12, if asdas {: :}, function calls)
        //      statement


        fn parse_statements(&mut self, tokens: Vec<Tostsken>) {
            println!("\nparse_statements: {:?}", tokens);

            let mut all: Vec<StatementType> = vec![];
            let mut current: Vec<Tostsken> = vec![];
            let mut depth = -1; // currently not in block

            for i in tokens {
                // add to current until End Of Statement is reached
                match i {
                    Tostsken::FunctionToaster => unreachable!(), // i think this is not reachable
                    Tostsken::Brace(ref brace) => {
                        if depth == -1 {
                            depth = 1;
                        }
                        if brace == "}:" || brace == ":}" {
                            depth-=1;
                        } 
                        current.push(i);
                        if depth == 1 {
                            all.push(StatementType::Other(current));
                            current = vec![];
                        }
                    },// yeah,
                    Tostsken::Semicolon => {
                        all.push(StatementType::Declaration(current));
                        current = vec![];
                    },
                    _ => {
                        current.push(i);
                    }
                };
            }

            for child in all {
                let child_node = Node::new();
                match child {
                    StatementType::Declaration(decl) => {
                        println!("declaration {:?}", decl);
                    },
                    StatementType::Other(_) => todo!(),
                }
                self.children.push(child_node);
            }
            // unimplemented!();
        }
    }



    /*
     * function that carves the function body out of a vector of tokens
     * of form [FunctionToaster, ..., ":{" | "{:", ..., ":}"|"}:"]
     */
    fn find_function_body(tokens: Vec<Tostsken>) -> Vec<Tostsken> {
        let mut in_body = false;
        let mut out = vec![];
        let mut depth = 1; // this is my fav trick

        for token in tokens {
            if in_body {
                if let Tostsken::Brace(brace) = &token {
                    match brace.as_str() {
                        ":}" | "}:" => depth -= 1,
                        "{:" | ":{" => depth += 1,
                        _ => (),
                    };
                }
                if depth == 0 {
                    break;
                }
                out.push(token);
            } else if let Tostsken::Brace(brace) = token {
                if brace == "{:" || brace == ":{" {
                    in_body = true;
                } else {
                    panic!("[ERROR] u fucked up. (wrong brace after function declaration)");
                }
            }
        }

        out
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
        // ^smart idea
        let mut out = Node::new();
        out.parse_funcs(tokens);
        out
    }

    pub fn parse(tokens: Vec<Tostsken>) -> Node {
        actual_parser(tokens)
        // no idea what i thought here
    }
}
