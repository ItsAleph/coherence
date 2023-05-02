use pest::Parser;

use crate::chrpatch::{
    constructor::construct,
    parser::{CHRPatchParser, Rule},
};
use std::fs;

pub fn main(file: &String) -> anyhow::Result<()> {
    let input = fs::read_to_string(file)?;
    let pair = CHRPatchParser::parse(Rule::file, &input)?.next().unwrap();
    let chr = construct(pair)?;

    println!("{:#?}", chr);

    Ok(())
}
