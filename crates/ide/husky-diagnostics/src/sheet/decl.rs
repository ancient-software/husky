use super::*;
use husky_syn_decl::*;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct DeclDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn decl_diagnostic_sheet(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> DeclDiagnosticSheet {
    let mut collector = ModuleDiagnosticsCollector::new(db, module_path);
    for (_, syn_node_decl) in db
        .syn_node_decl_sheet(module_path)
        .decls(db)
        .iter()
        .copied()
    {
        if let Some(syn_expr_region) = syn_node_decl.syn_expr_region(db) {
            for error in syn_node_decl.node_decl_errors(db) {
                if let SynNodeDeclError::Original(error) = error {
                    collector
                        .region_collector(syn_expr_region)
                        .visit_atom(error)
                }
            }
        }
    }
    DeclDiagnosticSheet::new(db, collector.finish())
}

impl Diagnose for OriginalSynNodeDeclError {
    type Context<'a> = RegionDiagnosticsContext<'a>;

    fn message(&self, ctx: &Self::Context<'_>) -> String {
        match self {
            OriginalSynNodeDeclError::Expr(e) => e.message(ctx),
            OriginalSynNodeDeclError::ExpectedOutputType(_) => {
                format!("Syntax Error: expect output type")
            }
            OriginalSynNodeDeclError::ExpectedCurry(_) => {
                format!("Syntax Error: expect `->`",)
            }
            OriginalSynNodeDeclError::ExpectedEolColon(_e) => {
                format!("Syntax Error: expect end-of-line colon",)
            }
            OriginalSynNodeDeclError::ExpectedRcurl(_) => {
                format!("Syntax Error: expect `}}`",)
            }
            OriginalSynNodeDeclError::ExpectedRightAngleBracketForImplicitParameterDeclList {
                ..
            } => {
                format!("Syntax Error: expect `>` for implicit parameter declaration list",)
            }
            OriginalSynNodeDeclError::ExpectedParameterDeclList(_) => {
                format!("Syntax Error: ExpectParameterDeclList",)
            }
            OriginalSynNodeDeclError::ExpectedImplicitParameterDecl(_) => {
                format!("Syntax Error: expect implicit parameter declaration",)
            }
            OriginalSynNodeDeclError::ExpectedRightParenthesisInParameterList(_) => {
                format!("Syntax Error: expected `)` in parameter list",)
            }
            OriginalSynNodeDeclError::ExpectedRightParenthesisInTupleStructFieldTypeList(_) => {
                format!("Syntax Error: expected `)` in tuple struct field type list",)
            }
            OriginalSynNodeDeclError::ExpectedValReturnType(_) => {
                format!("Syntax Error: ExpectVariableType",)
            }
            OriginalSynNodeDeclError::ExpectEqTokenForVariable(_) => {
                format!("Syntax Error: ExpectEqTokenForVariable",)
            }
            OriginalSynNodeDeclError::ExpectedLcurlOrLparOrSemicolonForStruct(_) => {
                format!("Syntax Error: expected `{{` `(` or `;` for struct",)
            }
            OriginalSynNodeDeclError::ExpectedEqForAssociatedType(_) => todo!(),
            OriginalSynNodeDeclError::ExpectLeftBracketInDerive(_) => todo!(),
            OriginalSynNodeDeclError::ExpectRightBracketInDerive(_) => todo!(),
            OriginalSynNodeDeclError::ExpectedColonBeforeValReturnType(_) => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        match self {
            OriginalSynNodeDeclError::Expr(error) => error.range(ctx),
            OriginalSynNodeDeclError::ExpectedOutputType(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedCurry(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedEolColon(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedRcurl(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedRightAngleBracketForImplicitParameterDeclList {
                regional_token_stream_state,
                ..
            }
            | OriginalSynNodeDeclError::ExpectedParameterDeclList(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedImplicitParameterDecl(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedRightParenthesisInParameterList(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedRightParenthesisInTupleStructFieldTypeList(
                regional_token_stream_state,
            )
            | OriginalSynNodeDeclError::ExpectedValReturnType(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectEqTokenForVariable(regional_token_stream_state)
            | OriginalSynNodeDeclError::ExpectedLcurlOrLparOrSemicolonForStruct(
                regional_token_stream_state,
            ) => ctx.token_stream_state_text_range(*regional_token_stream_state),
            OriginalSynNodeDeclError::ExpectedEqForAssociatedType(_) => todo!(),
            OriginalSynNodeDeclError::ExpectLeftBracketInDerive(_) => todo!(),
            OriginalSynNodeDeclError::ExpectRightBracketInDerive(_) => todo!(),
            OriginalSynNodeDeclError::ExpectedColonBeforeValReturnType(_) => todo!(),
        }
    }
}
