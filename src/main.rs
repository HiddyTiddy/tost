mod defs;
mod lexer;
mod parser;
//mod ast;

use defs::parse::parse_tree::Node;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub use defs::parse;
pub use lexer::lex;
use parse::parse_tree::NodeType;

use crate::parse::Tostsken;
use crate::parser::parse_tree;

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

fn _to_dot(ptree: &Node, parent: &str, idx: &mut i32) -> String {
    let mut id: String = format!("Node_{}", idx);
    let mut disp_name = "Node".to_string();
    if let Some(name) = ptree.content.to_owned() {

        let mut name:String = match name {
            NodeType::Function(fname) => fname,
            NodeType::Assignment => "=".to_string(),
            NodeType::Variable(varname) => varname,
            NodeType::Operation(op) => op,
            NodeType::Literal(literal) => match literal {
                parse::Literals::Integer(int) => format!("{}", int),
                parse::Literals::Float(float) => format!("{}", float),
                parse::Literals::Boolean(boolean) => if boolean { "true" } else { "false" }.to_string(),
                parse::Literals::Stringy(stringy) => stringy.to_string(),
            },
            NodeType::If => "if".to_string(),
            NodeType::While => todo!(),
            NodeType::Block => "block".to_string(),
            NodeType::StatementList => "<statement list>".to_string(),
        };
        name = str::replace(&name, "\"", "\\\"");
        id = format!("{}_{}", name, idx);
        disp_name = name;
    }

    let mut out = format!(
        "\"{}\" [label=\"{}\"]\n\"{}\" -- \"{}\";\n",
        id, disp_name, parent, id
    );

    for child in &ptree.children {
        *idx += 1;
        let tmp = &_to_dot(child, &id, idx);
        out += tmp;
    }

    out
}

fn to_dot(ptree: &Node) -> String {
    let mut idx = 0;
    let mut out = "\"root\" [label=\"root\"]\n\n".to_string();
    for ch in &ptree.to_owned().children {
        out += &_to_dot(ch, "root", &mut idx);
    }
    out
}

fn save_dot(filename: &str, dot_code: &str) {
    let mut file = File::create(filename).unwrap();

    write!(&mut file, "graph tost {{ \n{} }}", dot_code).unwrap();
}

fn main() {
    let source = match read_file("./testing/strings.tst") {
        Ok(data) => data,
        Err(err) => {
            eprintln!("couldn't read file {}", err);
            return;
        }
    };
    // println!("{}", source);
    let lex: Vec<Tostsken> = lex::lexer(source);
    // println!("{:?}\n\n", lex);
    let parsed:Node = parse_tree::parse(lex);
    // println!("{:?}", parsed);
    // parse_root node on top bc im too lazy to have a wrapper recursion function lol
    // dbg!(&parsed);
    save_dot("graph.dot", &to_dot(&parsed));
    // let ast = ast::generate_ast(parsed);
    // println!("{:?}", ast);
    println!("[\x1b[0;34mtost\x1b[0m]");
}
