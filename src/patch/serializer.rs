use crate::ast::*;
use crate::log_err;

pub fn serialize_file(file: &CHRFile) -> anyhow::Result<String> {
    let mut out = String::new();

    out.push_str(&format!("author: {} [{}]", file.username, file.email));
    out.push_str(&format!("date: {}", file.date.to_string()));
    out.push_str(&format!("message: {}", file.message));
    out.push('\n');

    for patch in serialize_patches(file)? {
        out.push_str(&patch);
        out.push('\n');
        out.push('\n');
    }

    Ok(out)
}

pub fn serialize_patches(file: &CHRFile) -> anyhow::Result<Vec<String>> {
    let mut out = vec![];

    for patch in &file.patches {
        out.push(match patch {
            CHRPatch::Create { path: _, additions: _ } => serialize_create_patch(&patch)?,
            CHRPatch::Update { path: _, changes: _ } => serialize_update_patch(&patch)?,
            CHRPatch::Delete { path: _, deletions: _ } => serialize_delete_patch(&patch)?,
        });
    }

    Ok(out)
}

pub fn serialize_create_patch(patch: &CHRPatch) -> anyhow::Result<String> {
    let mut out = String::new();

    match patch {
        CHRPatch::Create { path, additions } => {
            out.push_str(&format!("= create: {} >", path.to_str().unwrap()));
            out.push('\n');

            for addition in additions {
                out.push_str(&format!("+ {}", addition));
                out.push('\n');
            }

            if additions.len() > 0 {
                out.push_str(&format!("= create: {} >", path.to_str().unwrap()));
            };
        }
        _ => {
            return log_err!(
                "error",
                "Expected enum variant CHRPatch::Create, found {:?}",
                patch
            );
        }
    };

    Ok(out)
}

pub fn serialize_delete_patch(patch: &CHRPatch) -> anyhow::Result<String> {
    let mut out = String::new();

    match patch {
        CHRPatch::Delete { path, deletions } => {
            out.push_str(&format!("= delete: {} >", path.to_str().unwrap()));
            out.push('\n');

            for deletion in deletions {
                out.push_str(&format!("- {}", deletion));
                out.push('\n');
            }

            if deletions.len() > 0 {
                out.push_str(&format!("= delete: {} >", path.to_str().unwrap()));
            };
        }
        _ => {
            return log_err!(
                "error",
                "Expected enum variant CHRPatch::Delete, found {:?}",
                patch
            );
        }
    };

    Ok(out)
}

pub fn serialize_update_patch(patch: &CHRPatch) -> anyhow::Result<String> {
    let mut out = String::new();

    match patch {
        CHRPatch::Update { path, changes } => {
            out.push_str(&format!("= update: {} >", path.to_str().unwrap()));
            out.push('\n');

            for change in changes {
                out.push_str(&serialize_change(change)?);
                out.push('\n');
            }

            out.push_str(&format!("= update: {} >", path.to_str().unwrap()));
        }
        _ => {
            return log_err!(
                "error",
                "Expected enum variant CHRPatch::Update, found {:?}",
                patch
            );
        }
    };

    Ok(out)
}

pub fn serialize_change(change: &CHRChange) -> anyhow::Result<String> {
    let mut out = String::new();

    out.push_str(&match change {
        CHRChange::Edit { line: _, new: _, old: _ } => todo!(),
        CHRChange::Push { line: _, offset: _, content: _ } => todo!(),
        CHRChange::Cut { line: _, content: _ } => todo!(),
    });

    Ok(out)
}

pub fn serialize_push_change(change: &CHRChange) -> anyhow::Result<String> {
    
}