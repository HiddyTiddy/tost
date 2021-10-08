pub mod lex {

    use crate::defs::parse::Tostsken;
    
    trait Add {
        fn add(&mut self, val: String);
    }

    impl Add for Vec<Tostsken> {
        fn add(&mut self, val:String) {
            self.push
            (match val.as_str() {
                "toaster" => Tostsken::FunctionToaster,
                x => Tostsken::Word(x.to_string()) 
            });
        }
    }

    pub fn lexer(code: String) -> Vec<Tostsken> {
        let mut tokens = vec![];
        let mut word = String::from("");
        let mut commenting = false;
        // let number_regex = Regex::new(r"^(\+|-)?\d+$");
        for ch in code.chars() {
            if word == "!!" {
                commenting = true;
                word = String::from("");
            }
            if commenting {
                if ch == '\n' {
                    commenting = false;
                }
                continue;
            }
            // works but doesnt
            match ch {
                ' ' | ',' | ':' | '<' | '>' | '(' | ')' | '.' | ';' | '\n' | '\t' => {
                    tokens.add(word);
                    word = String::from("");
                    // if ch != ' '{ // actually preserve all white space
                    tokens.push(match ch {
                        ' ' | '\n' | '\t' => Tostsken::WhiteSpace(String::from(ch).clone()),
                        _ => Tostsken::OperatorOrSthIdk(String::from(ch).clone()),
                    });
                    // }
                    continue;
                }
                _ => {}
            }
            word.push(ch);
        }
        // still eats newline but eh
        if !word.is_empty() {
            tokens.add(word);
        }

        tokens
    }
}
