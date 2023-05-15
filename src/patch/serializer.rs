use crate::ast::*;

pub fn serialize_file(file: &CHRFile) -> String {
    let mut serialized = "".to_string();

    serialized.push_str(&serialize_header(&file.header));
    serialized.push('\n');
    serialized.push('\n');

    for patch in &file.patches {
        serialized.push_str(&serialize_patch(patch));
        serialized.push('\n');
        serialized.push('\n');
    };

    serialized
}

pub fn serialize_header(header: &CHRHeader) -> String {
    let mut serialized = "".to_string();

    serialized.push_str(&serialize_author(&header.author));
    serialized.push('\n');
    serialized.push_str(&format!("date: {}", header.date));
    serialized.push('\n');
    serialized.push_str(&format!("description: {}", header.description));

    serialized
}

pub fn serialize_author(author: &CHRAuthor) -> String {
    let mut serialized = "".to_string();

    serialized.push_str(&format!("author: {} [{}]", author.username, author.email));

    serialized
}

pub fn serialize_patch(patch: &CHRPatch) -> String {
    let mut serialized = "".to_string();

    let patch_header = format!("= {}: {} >", serialize_patch_type(&patch.type_), patch.path);

    serialized.push_str(&patch_header);
    serialized.push('\n');

    for action in &patch.actions {
        serialized.push_str(&serialize_action(action));
        serialized.push('\n');
    };

    serialized.push_str(&patch_header);

    serialized
}

pub fn serialize_patch_type(type_: &CHRPatchType) -> String {
    match type_ {
        CHRPatchType::Create => "create".to_string(),
        CHRPatchType::Update => "update".to_string(),
        CHRPatchType::Delete => "delete".to_string(),
    }
}

pub fn serialize_action(action: &CHRAction) -> String {
    match action {
        CHRAction::Move { path: _ } => serialize_move_action(action).unwrap(),
        CHRAction::Cut { number: _ } => serialize_cut_action(action).unwrap(),
        CHRAction::Replace { line: _, content: _ } => serialize_replace_action(action).unwrap(),
        CHRAction::Push { line: _, offset: _, content: _ } => serialize_push_action(action).unwrap(),
    }
}

pub fn serialize_move_action(action: &CHRAction) -> anyhow::Result<String> {
    if let CHRAction::Move { path } = action {
        return Ok(format!("move {}", path));
    };

    anyhow::bail!("Expected enum variant CHRAction::Move, found enum variant {:?}", action);
}

pub fn serialize_cut_action(action: &CHRAction) -> anyhow::Result<String> {
    if let CHRAction::Cut { number } = action {
        return Ok(format!("cut {}", number));
    };

    anyhow::bail!("Expected enum variant CHRAction::Cut, found enum variant {:?}", action);
}

pub fn serialize_replace_action(action: &CHRAction) -> anyhow::Result<String> {
    if let CHRAction::Replace { line, content } = action {
        return Ok(format!("replace {} {}", line, content));
    };

    anyhow::bail!("Expected enum variant CHRAction::Replace, found enum variant {:?}", action);
}

pub fn serialize_push_action(action: &CHRAction) -> anyhow::Result<String> {
    if let CHRAction::Push { line, offset, content } = action {
        return Ok(format!("push {}+{} {}", line, offset, content));
    };

    anyhow::bail!("Expected enum variant CHRAction::Push, found enum variant {:?}", action);
}
