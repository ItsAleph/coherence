use pest::Parser;

use crate::chrpatch::{
    constructor::construct,
    parser::{CHRPatchParser, Rule},
};
use std::fs;

pub fn main(file: &String) -> Result<(), anyhow::Error> {
    let input = fs::read_to_string(file)?;

    let pair = CHRPatchParser::parse(Rule::file, &input)?.next().unwrap();

    println!("{:#?}", pair);

    let chr = construct(pair);

    println!("{:#?}", chr?);

    Ok(())
}
