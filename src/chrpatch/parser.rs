use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./chrpatch/chrpatch.pest"]
pub struct CHRPatchParser;
