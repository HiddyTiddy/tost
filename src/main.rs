mod defs;
mod lexer;
mod parser;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub use defs::parse;
pub use lexer::lex;
pub use parser::parse_tree;


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

fn main() {
    let source = match read_file("./foo.tst") {
        Ok(data) => data,
        Err(err) => {
            eprintln!("couldn't read file {}", err);
            return;
        }
    };
    // println!("{}", source);
    let lex = lex::lexer(source);
    println!("{:?}\n\n", lex);
    let parsed = parse_tree::parse(lex);
    println!("{:?}", parsed);
    //println!("[\x1b[0;34mtost\x1b[0m]");
}
