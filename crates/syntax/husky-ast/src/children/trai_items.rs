use parsec::TryParseOptionFromStream;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TraitItems {
    ast_idx_range: AstIdxRange,
}

impl TraitItems {
    pub fn ast_idx_range(&self) -> AstIdxRange {
        self.ast_idx_range
    }
}

impl IsAstChildren for TraitItems {
    const ALLOW_STMT: AstResult<()> = Err(AstError::Original(
        OriginalAstError::UnexpectedStmtInsideTrait,
    ));

    #[inline(always)]
    fn determine_item_kind(item_keyword_group: EntityKindKeywordGroup) -> AstResult<EntityKind> {
        let trait_item_kind = match item_keyword_group {
            EntityKindKeywordGroup::Submodule(_) => {
                Err(OriginalAstError::UnexpectedModInsideModuleItem)?
            }
            EntityKindKeywordGroup::FugitiveFn(_) => TraitItemKind::MethodFn,
            EntityKindKeywordGroup::StaticFn(_, _) => TraitItemKind::AssociatedFunctionFn,
            EntityKindKeywordGroup::Gn(_) => todo!(),
            EntityKindKeywordGroup::FormalEntity(_) => todo!(),
            EntityKindKeywordGroup::MajorType(_) => todo!(),
            EntityKindKeywordGroup::AliasOrAssociateType(_) => TraitItemKind::AssociatedType,
            EntityKindKeywordGroup::Trait(_) => Err(OriginalAstError::UnexpectedTraitInsideTrait)?,
            EntityKindKeywordGroup::Val(_) => todo!(),
        };
        let trai_item_kind = trait_item_kind;
        Ok(EntityKind::AssociatedItem {
            associated_item_kind: AssociatedItemKind::TraitItem(trai_item_kind),
        })
    }
}

impl<'a> TryParseOptionFromStream<AstParser<'a>> for TraitItems {
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
