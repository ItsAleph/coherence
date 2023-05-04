use pest::Parser;

use crate::{
    chrpatch::{
        constructor::construct,
        parser::{CHRPatchParser, Rule},
    },
    utils::{build_tree, parse_ignore},
};
use std::fs;

pub fn main(file: &String) -> anyhow::Result<()> {
    let input = fs::read_to_string(file)?;
    let pair = CHRPatchParser::parse(Rule::file, &input)?.next().unwrap();
    let _chr = construct(pair)?;

    println!(); // ("{:#?}", chr);

    println!(
        "{:#?}",
        build_tree(".".to_string(), &parse_ignore(".gitignore".to_string())?)?
            .keys()
            .collect::<Vec<&String>>()
    );

    Ok(())
}
