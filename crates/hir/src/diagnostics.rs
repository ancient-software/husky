//! Re-export diagnostics such that clients of `hir` don't have to depend on
//! low-level crates.
//!
//! This probably isn't the best way to do this -- ideally, diagnistics should
//! be expressed in terms of hir types themselves.
use either::Either;
use hir_def::{path::ModPath, type_ref::Mutability};
use hir_expand::{name::Name, HirFileID, InFile};
use syntax::{ast, AstPtr, SyntaxNodePtr, TextRange};

use crate::Type;

macro_rules! diagnostics {
    ($($diag:ident,)*) => {
        pub enum AnyDiagnostic {$(
            $diag(Box<$diag>),
        )*}

        $(
            impl From<$diag> for AnyDiagnostic {
                fn from(d: $diag) -> AnyDiagnostic {
                    AnyDiagnostic::$diag(Box::new(d))
                }
            }
        )*
    };
}

diagnostics![
    AddReferenceHere,
    BreakOutsideOfLoop,
    InactiveCode,
    IncorrectCase,
    InvalidDeriveTarget,
    MacroError,
    MalformedDerive,
    MismatchedArgCount,
    MissingFields,
    MissingMatchArms,
    MissingOkOrSomeInTailExpr,
    MissingUnsafe,
    NoSuchField,
    RemoveThisSemicolon,
    ReplaceFilterMapNextWithFindMap,
    UnimplementedBuiltinMacro,
    UnresolvedExternCrate,
    UnresolvedImport,
    UnresolvedMacroCall,
    UnresolvedModule,
    UnresolvedProcMacro,
];

#[derive(Debug)]
pub struct UnresolvedModule {
    pub decl: InFile<AstPtr<ast::Module>>,
    pub candidate: String,
}

#[derive(Debug)]
pub struct UnresolvedExternCrate {
    pub decl: InFile<AstPtr<ast::ExternCrate>>,
}

#[derive(Debug)]
pub struct UnresolvedImport {
    pub decl: InFile<AstPtr<ast::UseTree>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnresolvedMacroCall {
    pub macro_call: InFile<AstPtr<ast::MacroCall>>,
    pub path: ModPath,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InactiveCode {
    pub node: InFile<SyntaxNodePtr>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnresolvedProcMacro {
    pub node: InFile<SyntaxNodePtr>,
    /// If the diagnostic can be pinpointed more accurately than via `node`, this is the `TextRange`
    /// to use instead.
    pub precise_location: Option<TextRange>,
    pub macro_name: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MacroError {
    pub node: InFile<SyntaxNodePtr>,
    pub message: String,
}

#[derive(Debug)]
pub struct UnimplementedBuiltinMacro {
    pub node: InFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub struct InvalidDeriveTarget {
    pub node: InFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub struct MalformedDerive {
    pub node: InFile<SyntaxNodePtr>,
}

#[derive(Debug)]
pub struct NoSuchField {
    pub field: InFile<AstPtr<ast::RecordExprField>>,
}

#[derive(Debug)]
pub struct BreakOutsideOfLoop {
    pub expr: InFile<AstPtr<ast::Expr>>,
}

#[derive(Debug)]
pub struct MissingUnsafe {
    pub expr: InFile<AstPtr<ast::Expr>>,
}

#[derive(Debug)]
pub struct MissingFields {
    pub file: HirFileID,
    pub field_list_parent: Either<AstPtr<ast::RecordExpr>, AstPtr<ast::RecordPat>>,
    pub field_list_parent_path: Option<AstPtr<ast::Path>>,
    pub missed_fields: Vec<Name>,
}

#[derive(Debug)]
pub struct ReplaceFilterMapNextWithFindMap {
    pub file: HirFileID,
    /// This expression is the whole method chain up to and including `.filter_map(..).next()`.
    pub next_expr: AstPtr<ast::Expr>,
}

#[derive(Debug)]
pub struct MismatchedArgCount {
    pub call_expr: InFile<AstPtr<ast::Expr>>,
    pub expected: usize,
    pub found: usize,
}

#[derive(Debug)]
pub struct RemoveThisSemicolon {
    pub expr: InFile<AstPtr<ast::Expr>>,
}

#[derive(Debug)]
pub struct MissingOkOrSomeInTailExpr {
    pub expr: InFile<AstPtr<ast::Expr>>,
    // `Some` or `Ok` depending on whether the return type is Result or Option
    pub required: String,
    pub expected: Type,
}

#[derive(Debug)]
pub struct MissingMatchArms {
    pub file: HirFileID,
    pub match_expr: AstPtr<ast::Expr>,
    pub arms: AstPtr<ast::MatchArmList>,
}

#[derive(Debug)]
pub struct AddReferenceHere {
    pub expr: InFile<AstPtr<ast::Expr>>,
    pub mutability: Mutability,
}

pub use hir_ty::diagnostics::IncorrectCase;
