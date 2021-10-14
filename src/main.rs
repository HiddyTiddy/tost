mod defs;
mod lexer;
mod parser;

use defs::parse::parse_tree::Node;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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

fn to_dot(ptree: Node, parent: &String, idx: &mut i32) -> String {
    let mut id: String = format!("Node_{}", idx);
    if let Some(mut name) = ptree.content {
        if name == "=" {
            name = "equals".to_string();
        }
        id = format!("{}_{}", name, idx);
    }

    let mut out = format!("\"{}\" -- \"{}\";\n", parent, id);

    for child in ptree.children {
        *idx += 1;
        let tmp = &to_dot(child, &id, idx);
        out += tmp;
    }

    out
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
    // println!("{:?}\n\n", lex);
    let parsed = parse_tree::parse(lex);
    // println!("{:?}", parsed);
    let mut i = 0;
    println!(
        "graph tost {{ \n{} }}",
        to_dot(parsed, &"root".to_string(), &mut i)
    );
    //println!("[\x1b[0;34mtost\x1b[0m]");
}
