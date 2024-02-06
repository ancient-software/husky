use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TypeItems {
    ast_idx_range: AstIdxRange,
}

impl TypeItems {
    pub fn ast_idx_range(&self) -> AstIdxRange {
        self.ast_idx_range
    }
}
impl IsAstChildren for TypeItems {
    const ALLOW_STMT: AstResult<()> = Err(AstError::Original(
        OriginalAstError::UnexpectedStmtInsideImplBlock,
    ));

    #[inline(always)]
    fn determine_item_kind(item_keyword_group: EntityKindKeywordGroup) -> AstResult<EntityKind> {
        let ty_item_kind = match item_keyword_group {
            EntityKindKeywordGroup::Submodule(_) => {
                Err(OriginalAstError::UnexpectedModUnderFugitive)?
            }
            EntityKindKeywordGroup::FugitiveFn(_) => TypeItemKind::MethodFn,
            EntityKindKeywordGroup::StaticFn(_, _) => TypeItemKind::AssocFunctionFn,
            EntityKindKeywordGroup::Gn(_) => TypeItemKind::AssocFunctionGn,
            EntityKindKeywordGroup::FormalEntity(_) => TypeItemKind::AssocFormal,
            EntityKindKeywordGroup::MajorType(_) => {
                Err(OriginalAstError::UnexpectedMajorTypeInsideImplBlock)?
            }
            EntityKindKeywordGroup::AliasOrAssociateType(_) => TypeItemKind::AssocType,
            EntityKindKeywordGroup::Trait(_) => {
                Err(OriginalAstError::UnexpectedTraitInsideImplBlock)?
            }
            EntityKindKeywordGroup::Val(_) => TypeItemKind::AssocVal,
            EntityKindKeywordGroup::Memo(_) => TypeItemKind::MemoizedField,
            EntityKindKeywordGroup::Const(_) => TypeItemKind::AssocConst,
        };
        Ok(EntityKind::AssocItem {
            associated_item_kind: AssocItemKind::TypeItem(ty_item_kind),
        })
    }
}

impl<'a> TryParseOptionFromStream<AstParser<'a>> for TypeItems {
    type Error = AstError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        parser: &mut AstParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        Ok(parser
            .parse_normal_ast_children_indented::<Self>()
            .map(|children| Self {
                ast_idx_range: children,
            }))
    }
}
