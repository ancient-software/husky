pub mod identifiable_entity_path;
pub mod principal_entity_path;

pub use self::identifiable_entity_path::*;
pub use self::principal_entity_path::*;

use crate::*;
use husky_entity_path::path::MajorEntityPath;
use husky_entity_tree::{jar::EntityTreeDb, subitem::SubitemPath};
use parsec::IsStreamParser;

// todo: change this to trait impl
impl<'a, C> SynExprParser<'a, C>
where
    C: IsSynExprContext<'a>,
{
    pub(crate) fn parse_identifiable_item_path_expr(
        &mut self,
        path_name_token: PathNameRegionalToken,
        principal_entity_path: PrincipalEntityPath,
    ) -> ItemPathExpr {
        let parent_expr_idx =
            self.context_mut()
                .alloc_item_path_expr(SynPrincipalEntityPathExpr::Root {
                    path_name_token,
                    principal_entity_path,
                });
        if let Some(major_path) = principal_entity_path.major()
            && let Some(colon_colon_token) = self.try_parse_err_as_none::<ColonColonRegionalToken>()
        {
            self.parse_subitem_identifiable_path_expr(
                parent_expr_idx,
                major_path,
                colon_colon_token,
            )
        } else {
            ItemPathExpr::Principal {
                path_expr_idx: parent_expr_idx,
                opt_path: Some(principal_entity_path),
            }
        }
    }

    fn parse_subitem_identifiable_path_expr(
        &mut self,
        parent_expr_idx: SynPrincipalEntityPathSynExprIdx,
        parent_path: MajorEntityPath,
        colon_colon_regional_token: ColonColonRegionalToken,
    ) -> ItemPathExpr {
        let ident_token: SynExprResult<IdentRegionalToken> =
            self.try_parse_expected(OriginalSynExprError::ExpectIdentAfterScopeResolution);
        let path: SynExprResult<PrincipalEntityPath> = match ident_token {
            Ok(ident_token) => {
                let ident = ident_token.ident();
                match self.db().subitem_path(parent_path, ident) {
                    Ok(subitem_path) => match subitem_path {
                        SubitemPath::Principal(path) => Ok(path),
                        SubitemPath::Assoc => {
                            let MajorEntityPath::MajorItem(parent_path) = parent_path else {
                                unreachable!()
                            };
                            return ItemPathExpr::AssocItem {
                                parent_expr_idx,
                                parent_path,
                                colon_colon_regional_token,
                                ident_token,
                            };
                        }
                    },
                    Err(error) => Err(OriginalSynExprError::EntityTree {
                        regional_token_idx: ident_token.regional_token_idx(),
                        error,
                    }
                    .into()),
                }
            }
            Err(_) => todo!(),
        };
        let opt_path = path.as_ref().ok().copied();
        let expr = SynPrincipalEntityPathExpr::Subitem {
            parent: parent_expr_idx,
            colon_colon_token: colon_colon_regional_token,
            ident_token,
            path,
        };
        let path_expr_idx = self.context_mut().alloc_item_path_expr(expr);
        if let Some(path) = opt_path
            && let Some(major_path) = path.major()
            && let Some(colon_colon_token) = self.try_parse_err_as_none::<ColonColonRegionalToken>()
        {
            self.parse_subitem_identifiable_path_expr(path_expr_idx, major_path, colon_colon_token)
        } else {
            ItemPathExpr::Principal {
                path_expr_idx,
                opt_path,
            }
        }
    }
}
