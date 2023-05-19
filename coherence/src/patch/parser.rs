use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./patch/chrpatch.pest"]
pub struct CHRPatchParser;
