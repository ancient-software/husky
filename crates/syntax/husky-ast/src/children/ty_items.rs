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
                Err(OriginalAstError::UnexpectedModInsideModuleItem)?
            }
            EntityKindKeywordGroup::FugitiveFn(_) => TypeItemKind::MethodFn,
            EntityKindKeywordGroup::StaticFn(_, _) => TypeItemKind::AssociatedFunctionFn,
            EntityKindKeywordGroup::Gn(_) => TypeItemKind::AssociatedFunctionGn,
            EntityKindKeywordGroup::FormalEntity(_) => TypeItemKind::AssociatedFormal,
            EntityKindKeywordGroup::MajorType(_) => {
                Err(OriginalAstError::UnexpectedMajorTypeInsideImplBlock)?
            }
            EntityKindKeywordGroup::AliasOrAssociateType(_) => TypeItemKind::AssociatedType,
            EntityKindKeywordGroup::Trait(_) => {
                Err(OriginalAstError::UnexpectedTraitInsideImplBlock)?
            }
            EntityKindKeywordGroup::Val(_) => TypeItemKind::MemoizedField,
            EntityKindKeywordGroup::ConstExpr(_) => TypeItemKind::AssociatedConstExpr,
        };
        Ok(EntityKind::AssociatedItem {
            associated_item_kind: AssociatedItemKind::TypeItem(ty_item_kind),
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
