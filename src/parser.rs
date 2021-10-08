pub mod parse_tree {
    use crate::defs::parse::parse_tree::*;
    use crate::defs::parse::Tostsken;

    fn slice(tokens: Vec<Tostsken>) -> Vec<Vec<Tostsken>> {
        // TODO: find start & end of statement

        let mut out = vec![];

        let mut current = vec![];
        // step 1 just worry abt functions
        let mut depth = -1;
        for i in tokens {
            // if true /* TODO this condition kinda is the most important part */ {

            //     out.push(current);
            //     current = vec![];
            // }

            match i {
                Tostsken::FunctionToaster => {
                    if depth == -1
                    // completely outside of functions
                    {
                        depth = 1; // we are now in the function
                        out.push(current);
                        current = vec![];
                    } /*else {
                          current.push(i);
                      }*/
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
                out.push(current);
                depth = -1;
                current = vec![];
            }
        }
        if !current.is_empty() {
            out.push(current);
        }

        out
    }

    fn actual_parser<'a>(tokens: Vec<Tostsken>) -> Node<'a> {
        // TODO: -> pass tokens[start..end] to actual_parser and tokens[end..] to actual_parser
        // TODO: find start & end of tokens
        // TODO: fix everythinf

        let sliced = slice(tokens);

        for i in sliced.iter() {
            println!("function: {:?}\n", i);
        }

        unimplemented!();
    }

    impl<'a> Default for Node<'a> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<'a> Node<'a> {
        pub fn new() -> Node<'a> {
            Node { children: vec![] }
        }

        pub fn expand(&mut self) {}
    }

    pub fn parse<'a>(tokens: Vec<Tostsken>) -> Node<'a> {
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
