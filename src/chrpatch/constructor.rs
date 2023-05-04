use pest::iterators::{Pair, Pairs};

use crate::ast::*;
use crate::chrpatch::parser::Rule;

pub fn construct(pair: Pair<Rule>) -> anyhow::Result<CHRFile> {
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
    let author_username = author_fields.next().unwrap().as_str().to_owned();
    let author_email = author_fields.next().unwrap().as_str().to_owned();
    let date: u64 = header_fields.next().unwrap().as_str().parse().unwrap();
    let description = header_fields.next().unwrap().as_str().to_owned();

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

    for patch_pair in pairs.by_ref() {
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

        let path = patch_fields.next().unwrap().as_str().to_owned();

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

    let statements = pairs.next().unwrap().into_inner();

    for action_pair in statements {
        let mut action_fields = action_pair.clone().into_inner();

        match action_pair.as_rule() {
            Rule::action_move => {
                let path = action_fields.next().unwrap().as_str().to_owned();

                actions.push(CHRAction::Move { path });
            }
            Rule::action_replace => {
                let line: u64 = action_fields.next().unwrap().as_str().parse().unwrap();
                let content = action_fields.next().unwrap().as_str().to_owned();

                actions.push(CHRAction::Replace { line, content });
            }
            Rule::action_push => {
                let line: u64 = action_fields.next().unwrap().as_str().parse().unwrap();
                let offset: u64 = action_fields.next().unwrap().as_str().parse().unwrap();
                let content = action_fields.next().unwrap().as_str().to_owned();

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
