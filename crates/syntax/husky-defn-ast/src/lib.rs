// base 1

use husky_regional_token::RegionalTokenVerseIdx;

use idx_arena::*;

/// asts that forms the body of a definition
///
/// everything is regional, in the sense that the token indices are counted relative to the starting token
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum DefnAst {
    Err,
    /// let or return or require or a single `if` or `elif` or `else` or case branch
    BasicStmtOrBranch {
        /// the token group for the head
        regional_token_verse_idx: RegionalTokenVerseIdx,
        /// maybe have body or not
        body: Option<DefnAstIdxRange>,
    },
    /// it's guaranteed that branches fall into `DefnAst::BasicStmtOrBranch`
    IfElseStmts {
        /// must have at least one `if` branch
        if_branch: DefnAstIdx,
        /// may zero or multiple `elif` branches
        elif_branches: DefnAstIdxRange,
        /// may have `else` branch or not
        else_branch: Option<DefnAstIdx>,
    },
    /// it's guaranteed that branches fall into `DefnAst::BasicStmtOrBranch`
    MatchStmt {
        /// the token group for the head
        regional_token_verse_idx: RegionalTokenVerseIdx,
        /// ast idx for the head
        pattern_stmt: DefnAstIdx,
        case_branches: DefnAstIdxRange,
    },
}

pub type DefnAstArena = Arena<DefnAst>;
pub type DefnAstArenaRef<'a> = ArenaRef<'a, DefnAst>;
pub type DefnAstIdx = ArenaIdx<DefnAst>;
pub type DefnAstIdxRange = ArenaIdxRange<DefnAst>;
