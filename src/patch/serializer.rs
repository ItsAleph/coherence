use crate::ast::*;

pub fn serialize_file(file: &CHRFile) -> anyhow::Result<String> {
    let mut out = String::new();

    out.push_str(&format!("author: {} [{}]", file.username, file.email));
    out.push_str(&format!("date: {}", file.date.to_string()));
    out.push_str(&format!("message: {}", file.message));
    out.push('\n');

    for patch in serialize_patches(file)? {
        out.push('\n');
        out.push_str(&patch);
    }

    Ok(out)
}

pub fn serialize_patches(file: &CHRFile) -> anyhow::Result<Vec<String>> {
    let mut out = vec![];

    for patch in file.patches {
        out.push(match patch {
            CHRPatch::Create { path, additions } => serialize_create_patch(&patch)?,
            CHRPatch::Update { path, changes } => serialize_update_patch(&patch)?,
            CHRPatch::Delete { path, deletions } => serialize_delete_patch(&patch)?,
        });
    }

    out
}

pub fn serialize_create_patch(patch: &CHRPatch) -> anyhow::Result<String> {
    let mut out = String::new();

    match patch {
        CHRPatch::Create { path, additions } => {
            out.push_str(&format!("= create: {} >", path.to_str()));

            for addition in additions {
                out.push('\n');
                out.push_str(&format!("+ {}", addition));
            }

            if additions.len() > 0 {
                out.push_str(&format!("= create: {} >", path.to_str()));
            };
        }
        _ => return log_err!("Expected enum variant CHRPatch::Create, found {:?}", patch),
    };

    Ok(out)
}
