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
            let mut all: Vec<Vec<Tostsken>> = vec![];
            let mut current_block: Vec<Tostsken> = vec![]; // temporary variable to store the current block in
            let mut depth: i32 = -1; // counter to account for nested blocks
            for i in tokens {
                match i {
                    // keyword 'toast'
                    Tostsken::FunctionToaster => {
                        if depth == -1
                        // completely outside of functions
                        {
                            depth = 1; // we are now in the function
                            all.push(current_block);
                            current_block = vec![];
                        } else {
                            // we hit another function while being inside
                            // increase depth
                            depth += 1;
                        }
                    }
                    // symbol :{, }:, {:, or :}
                    Tostsken::Brace(ref op) => {
                        if depth > 0 && (op == ":}" || op == "}:") {
                            depth -= 1;
                        }
                        // else if op == "{:" || op == ":{" {
                        //     depth += 1;
                        // }
                    }
                    _ => (),
                };
                // add the token to the current block
                current_block.push(i);
                if depth == 0 {
                    all.push(current_block);
                    depth = -1;
                    current_block = vec![];
                }
            }

            // after the loop, there might still be values in the block such as in
            /*tost
                toaster main {
                    x = 10;
                }

                y = 10;
            */
            // therefore, push it to all blocks to not discard it
            if !current_block.is_empty() {
                all.push(current_block);
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
                        let function = find_function_body(child);
                        child_node.parse_funcs(function.1); // function body
                        child_node.content = Some(function.0); // name of function
                    } else {
                        child_node.parse_statements(child);
                        child_node.content = Some("statement-list".to_string());
                    }
                    self.children.push(child_node);
                }
            }
        }

        // function level
        //   statements (x = 12, if asdas {: :}, function calls)
        //      statement
        fn parse_statements(&mut self, tokens: Vec<Tostsken>) {
            //println!("\nparse_statements: {:?}", tokens);

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
                            depth -= 1;
                        }
                        current.push(i);
                        if depth == 1 {
                            all.push(StatementType::Other(current));
                            current = vec![];
                        }
                    } // yeah,
                    Tostsken::Semicolon => {
                        all.push(StatementType::Declaration(current));
                        current = vec![];
                    }
                    _ => {
                        current.push(i);
                    }
                };
            }

            for child in all {
                let mut child_node = Node::new();
                match child {
                    StatementType::Declaration(decl) => {
                        //println!("declaration {:?}", decl);
                        child_node.parse_declaration(decl);
                    }
                    _ => todo!(),
                }
                self.children.push(child_node);
            }
            // unimplemented!();
        }

        fn parse_declaration(&mut self, tokens: Vec<Tostsken>) {
            let mut lhs = Node::new();
            let mut rhs: Vec<Tostsken> = vec![];
            let mut rhs_time: bool = false;
            for tok in tokens {
                // wtf
                if rhs_time {
                    match tok {
                        Tostsken::Word(_) | Tostsken::Integer(_) | Tostsken::Float(_) => {
                            rhs.push(tok);
                        }
                        _ => {}
                    }
                } else {
                    match tok {
                        Tostsken::Word(x) => {
                            lhs.content = Some(x);
                        }
                        Tostsken::Equals => {
                            rhs_time = true;
                        }
                        _ => {}
                    }
                }
            }

            self.children.push(lhs);
            self.content = Some("=".to_string());
            let mut right_child = Node::new();

            right_child.parse_arithmetic(rhs);

            self.children.push(right_child);
        }

        fn parse_arithmetic(&mut self, tokens: Vec<Tostsken>) {
            if tokens.len() == 1{ // a bit neater graph
                match &tokens[0] {
                    Tostsken::Integer(integer) => self.content = Some(format!("{}",integer)),
                    Tostsken::Float(floateger) => self.content = Some(format!("{}",floateger)),
                    token => {
                        unimplemented!("parse_arithmetic type {:?} not yet implemented", token);
                    },
                }
                return;
            }
            // TODO CHANGE THIS
            // kinda hacky and shitty but i wanna get results
            let mut left: Vec<Tostsken> = vec![];
            // this is horrible
            let mut right: Vec<Tostsken> = vec![];
            let mut rhs = false;
            let mut operation: String = "unreachable".to_string();
            let mut depth = 0;
            for tok in &tokens {
                // if let Tostsken::WhiteSpace(_) = tok {
                //     continue;
                // }

                if rhs {
                    right.push(tok.to_owned());
                } else {
                    if let Tostsken::OpenParenthesis = tok {
                        depth += 1;
                    }
                    else if let Tostsken::CloseParenthesis = tok {
                        depth -= 1;
                    }

                    // TODO: add tokens for plus minus etc
                    if depth == 0 {
                        if let Tostsken::Word(ref op) = tok {
                            match op.as_str() {
                                "+" | "-" | "*" | "/" => {
                                    operation = op.to_string();
                                    rhs = true;
                                    continue;
                                }
                                _ => (),
                            }
                        }
                    }
                    left.push(tok.to_owned());
                }
            }
            // if let Some(Tostsken::Word(word)) = tokens
            //     .iter()
            //     .find(|elem| -> bool { !matches!(elem, Tostsken::WhiteSpace(_)) })
            // {
            //     self.content = Some(word.to_string());
            // }

            println!("{:?} -> {:?} {:?} {:?} ", tokens, left, operation, right);
            if left.is_empty() {
                unreachable!();
            }

            
            let mut left_child = Node::new();
            if left.len() == 1 {
                if let Tostsken::Float(f) = left[0] {
                    left_child.content = Some(format!("{}",f));
                }
                else if let Tostsken::Integer(i) = left[0] {
                    left_child.content = Some(format!("{}",i));
                }
                else if let Tostsken::Word(w) = &left[0] {
                    left_child.content = Some(w.to_string());
                }
            }

            // println!("{:?}",left_child);
            
            // if left.len() != 1 {
            //     left_child.parse_arithmetic(tokens[1..(tokens.len() - 1)].to_vec());
            // }
            // left_child.parse_arithmetic(left);
            if !right.is_empty() {
                self.content = Some(operation);
    
                let mut right_child = Node::new();
                    right_child.parse_arithmetic(right);
                self.children = vec![left_child, right_child];
            } else {
                self.children = vec![left_child];
            }


        }
    }

    /*
     * function that carves the function body out of a vector of tokens
     * of form [FunctionToaster, ..., ":{" | "{:", ..., ":}"|"}:"]
     */
    fn find_function_body(tokens: Vec<Tostsken>) -> (String, Vec<Tostsken>) {
        let mut in_body = false;
        let mut out = vec![];
        let mut depth = 1; // this is my fav trick
        let mut funcname: String = "".to_string();

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
            } else {
                match token {
                    Tostsken::Brace(brace) => {
                        if brace == "{:" || brace == ":{" {
                            in_body = true;
                        } else {
                            panic!("[ERROR] u fucked up. (wrong brace after function declaration)");
                        }
                    }
                    Tostsken::Word(word) => {
                        funcname = word;
                    }
                    _ => (),
                }
            }
        }

        (funcname, out)
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
        let mut root = Node::new(); // this is the root of the tree
        root.parse_funcs(tokens); // parse the tokens on function level
                                  // and have them be children of root
        root.content = Some("root".to_string());
        root
    }

    pub fn parse(tokens: Vec<Tostsken>) -> Node {
        actual_parser(tokens)
        // no idea what i thought here
    }
}
