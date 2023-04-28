use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./chrpatch/chrpatch.pest"]
pub struct CHRPatchParser;

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::chrpatch::parser::CHRPatchParser;

    #[test]
    fn t() {}   
}