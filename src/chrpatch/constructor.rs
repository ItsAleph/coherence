use pest::iterators::{Pair, Pairs};

use crate::ast::*;
use crate::chrpatch::parser::Rule;

pub fn construct(pair: Pair<Rule>) -> Result<CHRFile, anyhow::Error> {
    let mut pairs = pair.into_inner();

    let header = construct_header(&mut pairs);
    let patches = construct_patches(&mut pairs);

    Ok(CHRFile { header, patches })
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

fn construct_patches(pairs: &mut Pairs<Rule>) -> Vec<CHRPatch> {
    let mut patches: Vec<CHRPatch> = vec![];

    while let Some(patch_pair) = pairs.next() {
        match patch_pair.as_rule() {
            Rule::patch => (),
            _ => continue,
        };

        let mut patch_fields = patch_pair.into_inner();
        let type_pair = patch_fields.next().unwrap();

        let type_ = match type_pair.as_str() {
            "create" => CHRPatchType::Create,
            "update" => CHRPatchType::Update,
            "delete" => CHRPatchType::Delete,
            _ => unreachable!(),
        };

        let path = patch_fields.next().unwrap().as_str().clone().to_string();

        let actions: Vec<CHRAction> = construct_actions(&mut patch_fields);

        patches.push(CHRPatch {
            type_,
            path,
            actions,
        });
    }

    patches
}

fn construct_actions(pairs: &mut Pairs<Rule>) -> Vec<CHRAction> {
    let mut actions: Vec<CHRAction> = vec![];

    let mut statements = pairs.next().unwrap().into_inner();

    while let Some(action_pair) = statements.next() {
        let mut action_fields = action_pair.clone().into_inner();

        match action_pair.as_rule() {
            Rule::action_move => {
                let path = action_fields.next().unwrap().as_str().clone().to_string();

                actions.push(CHRAction::Move { path });
            }
            Rule::action_replace => {
                let line: u64 = action_fields.next().unwrap().as_str().parse().unwrap();
                let content = action_fields.next().unwrap().as_str().clone().to_string();

                actions.push(CHRAction::Replace { line, content });
            }
            Rule::action_push => {
                let line: u64 = action_fields.next().unwrap().as_str().parse().unwrap();
                let offset: u64 = action_fields.next().unwrap().as_str().parse().unwrap();
                let content = action_fields.next().unwrap().as_str().clone().to_string();

                actions.push(CHRAction::Push {
                    line,
                    offset,
                    content,
                });
            }
            Rule::action_cut => {
                let number: u64 = action_fields.next().unwrap().as_str().parse().unwrap();

                actions.push(CHRAction::Cut { number });
            }
            _ => unreachable!(),
        }
    }

    actions
}
