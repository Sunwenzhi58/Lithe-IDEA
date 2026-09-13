//! Editor text-transform commands shared by macOS and Windows.
//!
//! Both products route deterministic line-level editing through this package
//! so selection mapping and comment-token semantics cannot drift between
//! platforms. Platform code only applies the returned replacement through
//! its native text engine.

pub(crate) mod line_edit;

pub(crate) use line_edit::{
    line_comment_token, line_edit, LineCommentToken, LineCommentTokenRequest, LineEditOperation,
    LineEditOutcome, LineEditRequest,
};

#[cfg(test)]
mod tests;
