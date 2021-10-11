pub mod lex {

    use crate::defs::parse::Tostsken;

    trait Add {
        fn add(&mut self, val: String);
    }

    impl Add for Vec<Tostsken> {
        fn add(&mut self, val: String) {
            if val.is_empty() {
                return;
            }
            self.push(match val.as_str() {
                "toaster" => Tostsken::FunctionToaster,
                ":}" | ":{" | "{:" | "}:" => Tostsken::Brace(val),
                // "}" | "{" => Tostsken::Brace(val),
                // " " | "," | ":" | "<" | ">" | "(" | ")" | "." | ";" | "\n" | "\t" => Tostsken::OperatorOrSthIdk(val),
                ";" => Tostsken::Semicolon,
                " " | "\n" | "\t" => Tostsken::WhiteSpace(val),
                x => Tostsken::Word(x.to_string()),
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
                ' ' | ',' /*| ':'*/ | '<' | '>' | '(' | ')' | '.' | ';' | '\n' | '\t' => {
                    tokens.add(word);
                    word = String::from("");
                    // if ch != ' '{ // actually preserve all white space
                    tokens.add(ch.to_string());
                    // }
                    continue;
                },
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
