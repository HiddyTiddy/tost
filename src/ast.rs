use crate::parse::abstract_syntax_tree::{ArithmeticNode, ArithmeticStuff, AstNode, DeclarationNode, FunctionNode, Types};
use crate::parse::parse_tree;

pub fn generate_ast(parse_tree: parse_tree::Node) -> Vec<AstNode> {
    // parse all functions high level here
    let mut out = vec![];

    for child in parse_tree.children {
        let function = parse_func(child);
        out.push( AstNode::FunctionNode(function));
    }

    out    
}

fn parse_func(function: parse_tree::Node) -> FunctionNode{
    let fname = function.content.expect("no function name");
    println!("parsing function {} {:?}", &fname, function.children[0]);
    let mut body = vec![];
    for i in &function.children[0].children {
        if matches!(&i.content, Some(x) if x == "=") {
            let (rhs, typ) = parse_declaration(fname.clone(), &i.children[1]);
            let var_name = &i.children[0].content.as_ref().expect("no variable name");
            let decl = DeclarationNode {
                id: format!("{}var_{}", fname, var_name), 
                // todo: use illegal tost characters for the ids so that you cant do hacky shit idk
                rhs,
                typ
            };
            body.push(AstNode::DeclarationNode(decl));
        }
    }
    FunctionNode {
        id: fname,
        body
    }
}

fn parse_declaration(func_name: String, right: &parse_tree::Node) -> (ArithmeticNode, Types) {
    println!("in func {} {:?}", func_name, right);
    // let mut lhs;
    // if let Some(var_name) = &right.content {
    //     if !matches!(var_name.as_str(), "+"|"-"|"*"|"/"|">"|"<") {
    //         lhs = ArithmeticStuff:: var_name.clone();
    //     } else {
    //         lhs = right.children[0];  
    //     }
    // }
    // let out = ArithmeticNode {
    //     lhs: 10,
    //     rhs: None,
    //     operation: None,
    // };
    todo!("[\x1b[0;31mCHANGE Node.content TO BE NOT A STRING\x1b[0m]");
    // unimplemented!();
}
