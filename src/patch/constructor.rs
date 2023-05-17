use pest::iterators::{Pair, Pairs};
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;

use crate::ast::*;
use crate::patch::parser::Rule;

pub fn construct(pair: &Pair<Rule>) -> anyhow::Result<CHRFile> {
    let mut rules = pair.clone().into_inner();
    let username = rules.next().unwrap().as_str().to_owned();
    let email = rules.next().unwrap().as_str().to_owned();
    let date = OffsetDateTime::parse(rules.next().unwrap().as_str(), &Iso8601::DEFAULT)?;
    let message = rules.next().unwrap().as_str().to_owned();
    let patches = construct_patches(&mut rules)?;

    Ok(CHRFile {
        username,
        email,
        date,
        message,
        patches,
    })
}

pub fn construct_patches(rules: &mut Pairs<Rule>) -> anyhow::Result<Vec<CHRPatch>> {
    let mut patches = vec![];

    while let Some(pair) = rules.next() {
        patches.push(match pair.as_rule() {
            Rule::create_patch => construct_create_patch(&pair)?,
            Rule::update_patch => construct_update_patch(&pair)?,
            Rule::delete_patch => construct_delete_patch(&pair)?,
            _ => unreachable!(
                "Encountered unknown rule when matching next patch rule: {:?}",
                &pair
            ),
        });
    }

    Ok(patches)
}

pub fn construct_create_patch(pair: &Pair<Rule>) -> anyhow::Result<CHRPatch> {
    let mut rules = pair.clone().into_inner();
    let mut additions = vec![];

    let path = rules.next().unwrap().as_str().into();

    while let Some(rule) = rules.next() {
        match rule.as_rule() {
            Rule::rest => additions.push(rule.as_str().to_owned()),
            Rule::path => break,
            _ => unreachable!(
                "Encountered unknown rule when matching next patch line rule: {:?}",
                &rule
            ),
        };
    }

    Ok(CHRPatch::Create { path, additions })
}

pub fn construct_update_patch(pair: &Pair<Rule>) -> anyhow::Result<CHRPatch> {
    let mut rules = pair.clone().into_inner();
    let mut changes = vec![];

    let path = rules.next().unwrap().as_str().into();

    while let Some(rule) = rules.next() {
        changes.push(match rule.as_rule() {
            Rule::update_patch_push => construct_push_change(&rule)?,
            Rule::update_patch_edit => construct_edit_change(&rule)?,
            Rule::update_patch_cut => construct_cut_change(&rule)?,
            _ => unreachable!(
                "Encountered unknown rule when matching next patch line rule: {:?}",
                &rule
            ),
        });
    }

    Ok(CHRPatch::Update { path, changes })
}

pub fn construct_push_change(pair: &Pair<Rule>) -> anyhow::Result<CHRPatchChange> {
    let mut rules = pair.clone().into_inner();

    let line = rules.next().unwrap().as_str().parse::<u64>()?;
    let offset = rules.next().unwrap().as_str().parse::<u64>()?;
    let content = rules.next().unwrap().as_str().to_owned();

    Ok(CHRPatchChange::Push {
        line,
        offset,
        content,
    })
}

pub fn construct_edit_change(pair: &Pair<Rule>) -> anyhow::Result<CHRPatchChange> {
    let mut rules = pair.clone().into_inner();

    let line = rules.next().unwrap().as_str().parse::<u64>()?;
    let new = rules.next().unwrap().as_str().to_owned();
    let _ = rules.next().unwrap();
    let old = rules.next().unwrap().as_str().to_owned();

    Ok(CHRPatchChange::Edit { line, new, old })
}

pub fn construct_cut_change(pair: &Pair<Rule>) -> anyhow::Result<CHRPatchChange> {
    let mut rules = pair.clone().into_inner();

    let line = rules.next().unwrap().as_str().parse::<u64>()?;
    let content = rules.next().unwrap().as_str().to_owned();

    Ok(CHRPatchChange::Cut { line, content })
}

pub fn construct_delete_patch(pair: &Pair<Rule>) -> anyhow::Result<CHRPatch> {
    let mut rules = pair.clone().into_inner();
    let mut deletions = vec![];

    let path = rules.next().unwrap().as_str().into();

    while let Some(rule) = rules.next() {
        match rule.as_rule() {
            Rule::rest => deletions.push(rule.as_str().to_owned()),
            Rule::path => break,
            _ => unreachable!(
                "Encountered unknown rule when matching next patch line rule: {:?}",
                &rule
            ),
        };
    }

    Ok(CHRPatch::Delete { path, deletions })
}
