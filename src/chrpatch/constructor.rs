use pest::iterators::{Pair, Pairs};

use crate::ast::*;
use crate::chrpatch::parser::Rule;

fn construct(pair: Pair<Rule>) -> Result<CHRFile, anyhow::Error> {
    let mut pairs = pair.into_inner();

    let header = construct_header(&mut pairs);

    Err(anyhow::anyhow!("something's fucked up"))
}

fn construct_header(pairs: &mut Pairs<Rule>) -> CHRHeader {
    let header_pair = pairs.next().unwrap();
    let mut header_fields = header_pair.into_inner();
    let author_pair = header_fields.next().unwrap();
    let mut author_fields = author_pair.into_inner();
    let author_username = author_fields.next().unwrap().as_str().clone().to_string();
    let author_email = author_fields.next().unwrap().as_str().clone().to_string();
    let date: u64 = header_fields.next().unwrap().as_str().parse().unwrap();
    let description = header_fields.next().unwrap().as_str().clone().to_string();

    CHRHeader {
        author: CHRAuthor {
            username: author_username,
            email: author_email,
        },
        date,
        description,
    }
}
