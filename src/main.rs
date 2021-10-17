mod defs;
mod lexer;
mod parser;

use defs::parse::parse_tree::Node;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

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

fn _to_dot(ptree: Node, parent: &str, idx: &mut i32) -> String {
    let mut id: String = format!("Node_{}", idx);
    let mut disp_name = "Node".to_string();
    if let Some(name) = ptree.content {
        id = format!("{}_{}", name, idx);
        disp_name = name;
    }

    let mut out = format!("\"{}\" [label=\"{}\"]\n\"{}\" -- \"{}\";\n", id, disp_name, parent, id);

    for child in ptree.children {
        *idx += 1;
        let tmp = &_to_dot(child, &id, idx);
        out += tmp;
    }

    out
}

fn to_dot(ptree: Node) -> String {
    let mut idx = 0;
    let mut out = "\"root\" [label=\"root\"]\n\n".to_string();
    for ch in ptree.children {
        out += &_to_dot(ch, "root", &mut idx);
    }
    out
}

fn save_dot(filename :&str, dot_code: &str) {
    let mut file= File::create(filename).unwrap();

    write!(&mut file, "graph tost {{ \n{} }}", dot_code).unwrap();
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
    // parse_root node on top bc im too lazy to have a wrapper recursion function lol
    save_dot("graph.dot", &to_dot(parsed))
    //println!("[\x1b[0;34mtost\x1b[0m]");
}
