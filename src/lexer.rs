pub mod lex {

    use crate::defs::parse::Tostsken;

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
                    tokens.push(Tostsken::Word(word.clone()));
                    word = String::from("");
                    // if ch != ' '{ // actually preserve all white space
                    tokens.push(Tostsken::OperatorOrSthIdk(String::from(ch).clone()));
                    // }
                    continue;
                }
                _ => {}
            }
            word.push(ch);
        }

        tokens
    }
}
