use std::path::PathBuf;

use time::OffsetDateTime;

/// Represents a .chr commit file
#[derive(Debug)]
pub struct CHRFile {
    /// Commit's author's username
    pub username: String,
    /// Commit's author's email
    pub email: String,
    /// Commit's creation date
    pub date: OffsetDateTime,
    /// Commit message
    pub message: String,
    /// Vec of patches commit contains
    pub patches: Vec<CHRPatch>,
}

/// Represents a patch
#[derive(Debug)]
pub enum CHRPatch {
    /// "create" patch
    Create {
        /// Path to the file this patch affects
        path: PathBuf,
        /// Vec of additions made in this patch
        additions: Vec<String>,
    },
    /// "update" patch
    Update {
        /// Path to the file this patch affects
        path: PathBuf,
        /// Vec of changes made in this patch
        changes: Vec<CHRChange>,
    },
    /// "delete" patch
    Delete {
        /// Path to the file this patch affects
        path: PathBuf,
        /// Vec of deletions in this patch
        deletions: Vec<String>,
    },
}

/// Represents a change made in an "update" patch
#[derive(Debug)]
pub enum CHRChange {
    /// An "edit" change
    Edit {
        /// The line this change applies to
        line: u64,
        /// New line content
        new: String,
        /// Old line content
        old: String,
    },
    /// A "push" change
    Push {
        /// The line this change applies to
        line: u64,
        /// Offset of this change relative to the line
        offset: u64,
        /// The content of the line
        content: String,
    },
    /// A "cut" change
    Cut {
        /// The line this change applies to
        line: u64,
        /// The content of the line
        content: String,
    },
}
