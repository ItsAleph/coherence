mod chrpatch;
mod cli;

use pest::Parser;

fn main() {
    for pair in crate::chrpatch::parser::CHRPatchParser::parse(
        crate::chrpatch::parser::Rule::file,
        &std::fs::read_to_string("test.chrpatch").unwrap(),
    )
    .unwrap()
    {
        println!("{:?}", pair);
    }
}
