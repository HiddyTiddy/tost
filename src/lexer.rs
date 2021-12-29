pub mod lex {

    use regex::Regex;

    use crate::defs::parse::Tostsken;

    trait Add {
        fn add(&mut self, val: String);
    }

    impl Add for Vec<Tostsken> {
        fn add(&mut self, val: String) {
            let int_regex = Regex::new(r"^(\+|-)?([1-9]([0-9])*|0)$").unwrap();
            let float_regex = Regex::new(r"^(\+|-)?([1-9]([0-9])*|0)\.(\d*)$").unwrap();
            if val.is_empty() {
                return;
            }
            self.push(match val.as_str() {
                "toaster" => Tostsken::FunctionToaster,
                ":}" | ":{" | "{:" | "}:" => Tostsken::Brace(val),
                "if" => Tostsken::If,
                // "}" | "{" => Tostsken::Brace(val),
                // " " | "," | ":" | "<" | ">" | "(" | ")" | "." | ";" | "\n" | "\t" => Tostsken::OperatorOrSthIdk(val),
                ";" => Tostsken::Semicolon,
                "=" => Tostsken::Equals,
                " " | "\n" | "\t" => Tostsken::WhiteSpace(val),
                "(" => Tostsken::OpenParenthesis,
                ")" => Tostsken::CloseParenthesis,
                "true" => Tostsken::Boolean(true),
                "false" => Tostsken::Boolean(false),
                x => {
                    if float_regex.is_match(x) {
                        let val: f64 = x.to_string().parse().unwrap();
                        Tostsken::Float(val)
                    } else if int_regex.is_match(x) {
                        let val: i64 = x.to_string().parse().unwrap();
                        Tostsken::Integer(val)
                    } else {
                        Tostsken::Word(x.to_string())
                    }
                }
            });
        }
    }

    pub fn lexer(code: String) -> Vec<Tostsken> {
        let mut tokens = vec![];
        let mut word = String::new();
        let mut commenting = false;
        // let number_regex = Regex::new(r"^(\+|-)?\d+$");
        let mut in_string: bool = false;
        let mut escaping: bool = false;
        let mut chars = code.chars().peekable();
        while let Some(ch) = chars.next() {
            if word == "!!" {
                commenting = true;
                word = String::new();
            }
            if commenting {
                if ch == '\n' {
                    commenting = false;
                }
                continue;
            }

            if in_string {
                if !escaping {
                    match ch {
                        '\\' => escaping = true,
                        '"' => {
                            in_string = false;
                            tokens.push(Tostsken::Stringy(format!("\"{}\"", word)));
                            word = String::new();
                        }
                        _ => word.push(ch),
                    }
                } else {
                    word.push(match ch {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        _ => ch,
                    });
                    escaping = false
                }
            } else {
                // works but doesnt
                match ch {
                    '"' => {
                        in_string = true;
                    }
                    ' ' | ',' /*| ':'*/ | '<' | '>' | '(' | ')' /*| '.'*/ | ';' | '=' | '\n' | '\t' => {
                        tokens.add(word);
                        word = String::new();
                        tokens.add(ch.to_string());
                    },
                    ':' => {
                        if let Some(after) = chars.peek() {
                            tokens.add(word);
                            word = String::new();
                            if *after == '{'  {
                                tokens.add(":{".to_string());
                                chars.next();
                            } else if *after == '}' {
                                tokens.add(":}".to_string());
                                chars.next();
                            } else {
                                tokens.add(':'.to_string());
                            }
                        } else {
                            tokens.add(word);
                            word = String::new();
                            tokens.add(':'.to_string());
                        }
                    }
                    '{' | '}' => {
                        if let Some(after) = chars.peek() {
                            tokens.add(word);
                            word = String::new();
                            if *after == ':'  {
                                let mut tmp = ch.to_string();
                                tmp.push(chars.next().unwrap());
                                tokens.add(tmp);
                            } else {
                                tokens.add(ch.to_string());
                            }
                        } else {
                            tokens.add(word);
                            word = String::new();
                            tokens.add(ch.to_string());
                        }
                    }
                    // TODO handle + and - (for 1 -1 vs 1 - 1)
                    '+' | '-' => {
                        tokens.add(word);
                        word = String::new();
                        tokens.add(ch.to_string());
                    }
                    _ => {
                        word.push(ch);
                    }
                }
            }
        }
        // still eats newline but eh
        if !word.is_empty() {
            tokens.add(word);
        }

        tokens
    }
}
