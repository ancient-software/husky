use super::*;
use husky_decl::*;
use husky_expr::ExprError;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct DeclDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn decl_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> DeclDiagnosticSheet {
    let mut sheet_collector = SheetDiagnosticsCollector::new(db, module_path);
    if let (Ok(ranged_token_sheet), Ok(decl_sheet)) = (
        db.ranged_token_sheet(module_path),
        db.decl_sheet(module_path),
    ) {
        let _token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        for (_path, decl) in decl_sheet.decls().iter().copied() {
            match decl {
                Err(DeclError::Original(error)) => sheet_collector.visit_atom(error),
                _ => (),
            }
        }
    }
    // todo
    DeclDiagnosticSheet::new(db, sheet_collector.finish())
}

impl Diagnose for OriginalDeclError {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, ctx: &Self::Context<'_>) -> String {
        match self {
            OriginalDeclError::ExpectLCurlOrLParOrSemicolon(_) => {
                format!("ExpectLCurlOrLParOrSemicolon")
            }
            OriginalDeclError::NoSuchItem => format!("NoSuchItem"),
            OriginalDeclError::Expr(e) => e.message(ctx),
            OriginalDeclError::Deprecated => "deprecated".to_string(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        match self {
            OriginalDeclError::ExpectLCurlOrLParOrSemicolon(token_idx) => todo!(),
            OriginalDeclError::NoSuchItem => todo!(),
            OriginalDeclError::Expr(e) => e.range(ctx),
            OriginalDeclError::Deprecated => todo!(),
        }
    }
}

impl Diagnose for OriginalDeclExprError {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, ctx: &Self::Context<'_>) -> String {
        match self {
            OriginalDeclExprError::Expr(e) => e.message(ctx),
            OriginalDeclExprError::ExpectOutputType(_) => {
                format!("Syntax Error: expect output type")
            }
            OriginalDeclExprError::ExpectCurry(_) => {
                format!("Syntax Error: expect `->`",)
            }
            OriginalDeclExprError::ExpectedEolColon(_e) => {
                format!("Syntax Error: expect end-of-line colon",)
            }
            OriginalDeclExprError::ExpectRightCurlyBrace(_) => {
                format!("Syntax Error: expect `}}`",)
            }
            OriginalDeclExprError::ExpectRightAngleBracketForImplicitParameterDeclList {
                ..
            } => {
                format!("Syntax Error: expect `>` for implicit parameter declaration list",)
            }
            OriginalDeclExprError::ExpectParameterDeclList(_) => {
                format!("Syntax Error: ExpectParameterDeclList",)
            }
            OriginalDeclExprError::ExpectImplicitParameterDecl(_) => {
                format!("Syntax Error: expect implicit parameter declaration",)
            }
            OriginalDeclExprError::ExpectRightParenthesisInParameterList(_) => {
                format!("Syntax Error: ExpectRightParenthesisInParameterList",)
            }
            OriginalDeclExprError::ExpectVariableType(_) => {
                format!("Syntax Error: ExpectVariableType",)
            }
            OriginalDeclExprError::ExpectEqTokenForVariable(_) => {
                format!("Syntax Error: ExpectEqTokenForVariable",)
            }
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        match self {
            OriginalDeclExprError::Expr(error) => error.range(ctx),
            OriginalDeclExprError::ExpectOutputType(token_stream_state)
            | OriginalDeclExprError::ExpectCurry(token_stream_state)
            | OriginalDeclExprError::ExpectedEolColon(token_stream_state)
            | OriginalDeclExprError::ExpectRightCurlyBrace(token_stream_state)
            | OriginalDeclExprError::ExpectRightAngleBracketForImplicitParameterDeclList {
                token_stream_state,
                ..
            }
            | OriginalDeclExprError::ExpectParameterDeclList(token_stream_state)
            | OriginalDeclExprError::ExpectImplicitParameterDecl(token_stream_state)
            | OriginalDeclExprError::ExpectRightParenthesisInParameterList(token_stream_state)
            | OriginalDeclExprError::ExpectVariableType(token_stream_state)
            | OriginalDeclExprError::ExpectEqTokenForVariable(token_stream_state) => {
                todo!()
                // ctx.token_text_range(*token_idx)
            }
        }
    }
}
