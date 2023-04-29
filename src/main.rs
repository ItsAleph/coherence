mod chrpatch;
mod cli;

use pest::{Parser, iterators::Pair};

fn main() {
    for pair in crate::chrpatch::parser::CHRPatchParser::parse(
        crate::chrpatch::parser::Rule::file,
        &std::fs::read_to_string("test.chrpatch").unwrap(),
    )
    .unwrap()
    {
        println!("{:?}", pair.as_rule());
        traverse(pair, Some(2));
    }
}

fn traverse(pair: Pair<crate::chrpatch::parser::Rule>, indent_option: Option<i32>) {
    let indent = indent_option.unwrap_or(0);

    for child in pair.into_inner() {
        println!("{}{:?}", " ".repeat(indent as usize), child.as_rule());
        traverse(child, Some(indent + 2));
    }
}
