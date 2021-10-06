use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
enum Tostsken {
    // Statement(String),
    // Expression(String),
    // Function(String),
    Word(String),
    OperatorOrSthIdk(String),
}

// no idea
fn read_file(fname: &str) -> Result<String, io::Error> {
    let fhandle = File::open(fname)?;
    let reader = BufReader::new(fhandle);
    let mut out = String::new();

    for (i, line) in reader.lines().enumerate() {
        if i != 0 {
            out.push('\n'); // no idea
        }
        out += &line.unwrap();
    }

    Ok(out)
}

fn lexer(code: String) -> Vec<Tostsken> {
    let mut tokens = vec![];
    let mut word = String::from("");
    let mut commenting = false;
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
        match ch {
            ' ' => {
                tokens.push(Tostsken::Word(word.clone()));
                word = String::from("");
                continue;
            }
            ',' | ':' | '<' | '>' | '(' | ')' | '.' | ';' | '\n' | '\t' => {
                tokens.push(Tostsken::Word(word.clone()));
                word = String::from("");
                tokens.push(Tostsken::OperatorOrSthIdk(String::from(ch).clone()))
            }
            _ => {}
        }
        word.push(ch);
    }

    tokens
}

fn main() {
    let source = match read_file("./tost.tst") {
        Ok(data) => data,
        Err(err) => {
            eprintln!("couldn't read file {}", err);
            return;
        }
    };
    println!("{}", source);
    let lex = lexer(source);
    println!("{:?}", lex);
    println!("[\x1b[0;34mtost\x1b[0m]");
}
