use crate::parse::abstract_syntax_tree::{
    ArithmeticNode, AstNode, DeclarationNode, FunctionNode,
};
use crate::parse::parse_tree::NodeType;
use crate::parse::{parse_tree, Types};

pub fn generate_ast(parse_tree: parse_tree::Node) -> Vec<AstNode> {
    // parse all functions high level here
    let mut out = vec![];

    for child in parse_tree.children {
        let function = parse_func(child);
        out.push(AstNode::FunctionNode(function));
    }

    out
}

fn parse_func(function: parse_tree::Node) -> FunctionNode {
    let fname = if let NodeType::Function(fname) = function.content.expect("no function name") {
        fname
    } else {
        unreachable!("function name missing");
    };
    println!("parsing function {} {:?}", &fname, function.children[0]);
    let mut body = vec![];
    for i in &function.children[0].children {
        if let Some(NodeType::If) = &i.content { // what
            unimplemented!("what did i think here");
            // let (rhs, typ) = parse_declaration(fname.clone(), &i.children[1]);
            // // let var_name = &i.children[0].content.as_ref().expect("no variable name");
            // let var_name = if let Some(NodeType::Variable(varname)) = &i.children[0].content {
            //     varname
            // } else {
            //     panic!("no variable name found")
            // };
            // let decl = DeclarationNode {
            //     id: format!("{}var_{}", fname, var_name),
            //     // todo: use illegal tost characters for the ids so that you cant do hacky shit idk
            //     rhs,
            //     typ,
            // };
            // body.push(AstNode::DeclarationNode(decl));
        } else {
            assert_eq!(i.children.len(), 2);
            parse_declaration(fname.clone(), &i.children[0], &i.children[1]);
        }
    }
    FunctionNode { id: fname, body }
}

fn parse_declaration(func_name: String, right: &parse_tree::Node, left: &parse_tree::Node) -> (ArithmeticNode, Types) {
    println!("in func {} {:?} == {:?}", func_name, right, left);
    // let mut op_stack = vec![];
    // let mut val_stack = vec![];

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
    // todo!("[\x1b[0;31mCHANGE Node.content TO BE NOT A STRING\x1b[0m]");
    unimplemented!();
}
