//! User (postfix)-snippet definitions.
//!
//! Actual logic is implemented in [`crate::completions::postfix`] and [`crate::completions::snippet`] respectively.

use std::ops::Deref;

// Feature: User Snippet Completions
//
// husky-lang-server allows the user to define custom (postfix)-snippets that may depend on items to be accessible for the current scope to be applicable.
//
// A custom snippet can be defined by adding it to the `husky-lang-server.completion.snippets` object respectively.
//
// [source,json]
// ----
// {
//   "husky-lang-server.completion.snippets": {
//     "thread spawn": {
//       "prefix": ["spawn", "tspawn"],
//       "body": [
//         "thread::spawn(move || {",
//         "\t$0",
//         ")};",
//       ],
//       "description": "Insert a thread::spawn call",
//       "requires": "std::thread",
//       "scope": "expr",
//     }
//   }
// }
// ----
//
// In the example above:
//
// * `"thread spawn"` is the name of the snippet.
//
// * `prefix` defines one or more trigger words that will trigger the snippets completion.
// Using `postfix` will instead create a postfix snippet.
//
// * `body` is one or more lines of content joined via newlines for the final output.
//
// * `description` is an optional description of the snippet, if unset the snippet name will be used.
//
// * `requires` is an optional list of item paths that have to be resolvable in the current crate where the completion is rendered.
// On failure of resolution the snippet won't be applicable, otherwise the snippet will insert an import for the items on insertion if
// the items aren't yet in scope.
//
// * `scope` is an optional filter for when the snippet should be applicable. Possible values are:
// ** for Snippet-Scopes: `expr`, `item` (default: `item`)
// ** for Postfix-Snippet-Scopes: `expr`, `type` (default: `expr`)
//
// The `body` field also has access to placeholders as visible in the example as `$0`.
// These placeholders take the form of `$number` or `${number:placeholder_text}` which can be traversed as tabstop in ascending order starting from 1,
// with `$0` being a special case that always comes last.
//
// There is also a special placeholder, `${receiver}`, which will be replaced by the receiver expression for postfix snippets, or nothing in case of normal snippets.
// It does not act as a tabstop.
use ide_db::helpers::{import_assets::LocatedImport, insert_use::ImportScope};
use itertools::Itertools;
use syntax::{ast, GreenNode, SyntaxNode};

use crate::{context::CompletionContext, ImportEdit};

/// A snippet scope describing where a snippet may apply to.
/// These may differ slightly in meaning depending on the snippet trigger.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnippetScope {
    Item,
    Expr,
    Type,
}

/// A user supplied snippet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snippet {
    pub postfix_triggers: Box<[Box<str>]>,
    pub prefix_triggers: Box<[Box<str>]>,
    pub scope: SnippetScope,
    pub description: Option<Box<str>>,
    snippet: String,
    // These are `ast::Path`'s but due to SyntaxNodes not being Send we store these
    // and reconstruct them on demand instead. This is cheaper than reparsing them
    // from strings
    requires: Box<[GreenNode]>,
}

impl Snippet {
    pub fn new(
        prefix_triggers: &[String],
        postfix_triggers: &[String],
        snippet: &[String],
        description: &str,
        requires: &[String],
        scope: SnippetScope,
    ) -> Option<Self> {
        if prefix_triggers.is_empty() && postfix_triggers.is_empty() {
            return None;
        }
        let (requires, snippet, description) = validate_snippet(snippet, description, requires)?;
        Some(Snippet {
            // Box::into doesn't work as that has a Copy bound 😒
            postfix_triggers: postfix_triggers
                .iter()
                .map(Deref::deref)
                .map(Into::into)
                .collect(),
            prefix_triggers: prefix_triggers
                .iter()
                .map(Deref::deref)
                .map(Into::into)
                .collect(),
            scope,
            snippet,
            description,
            requires,
        })
    }

    /// Returns [`None`] if the required items do not resolve.
    pub(crate) fn imports(
        &self,
        ctx: &CompletionContext,
        import_scope: &ImportScope,
    ) -> Option<Vec<ImportEdit>> {
        import_edits(ctx, import_scope, &self.requires)
    }

    pub fn snippet(&self) -> String {
        self.snippet.replace("${receiver}", "")
    }

    pub fn postfix_snippet(&self, receiver: &str) -> String {
        self.snippet.replace("${receiver}", receiver)
    }
}

fn import_edits(
    ctx: &CompletionContext,
    import_scope: &ImportScope,
    requires: &[GreenNode],
) -> Option<Vec<ImportEdit>> {
    todo!()
}

fn validate_snippet(
    snippet: &[String],
    description: &str,
    requires: &[String],
) -> Option<(Box<[GreenNode]>, String, Option<Box<str>>)> {
    todo!()
}
