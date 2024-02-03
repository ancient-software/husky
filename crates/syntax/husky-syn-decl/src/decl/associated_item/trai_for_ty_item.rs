mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;

pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum TraitForTypeItemSynNodeDecl {
    AssociatedFn(TraitForTypeAssociatedFnSynNodeDecl),
    MethodFn(TraitForTypeMethodFnSynNodeDecl),
    AssociatedType(TraitForTypeAssociatedTypeSynNodeDecl),
    AssociatedVal(TraitForTypeAssociatedValSynNodeDecl),
}

impl From<TraitForTypeItemSynNodeDecl> for ItemSynNodeDecl {
    fn from(decl: TraitForTypeItemSynNodeDecl) -> Self {
        ItemSynNodeDecl::AssociatedItem(decl.into())
    }
}

impl TraitForTypeItemSynNodeDecl {
    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            TraitForTypeItemSynNodeDecl::AssociatedFn(slf) => slf.syn_expr_region(db),
            TraitForTypeItemSynNodeDecl::MethodFn(slf) => slf.syn_expr_region(db),
            TraitForTypeItemSynNodeDecl::AssociatedType(slf) => slf.syn_expr_region(db),
            TraitForTypeItemSynNodeDecl::AssociatedVal(slf) => slf.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            TraitForTypeItemSynNodeDecl::AssociatedFn(slf) => slf.errors(db),
            TraitForTypeItemSynNodeDecl::MethodFn(slf) => slf.errors(db),
            TraitForTypeItemSynNodeDecl::AssociatedType(slf) => slf.errors(db),
            TraitForTypeItemSynNodeDecl::AssociatedVal(slf) => slf.errors(db),
        }
    }
}

impl HasSynNodeDecl for TraitForTypeItemSynNodePath {
    type NodeDecl = TraitForTypeItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        trai_for_ty_item_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_for_ty_item_syn_node_decl(
    db: &::salsa::Db,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> TraitForTypeItemSynNodeDecl {
    let parser = DeclParser::new(db, syn_node_path.into());
    parser.parse_trai_for_ty_item_syn_node_decl()
}

impl<'a> DeclParser<'a> {
    fn parse_trai_for_ty_item_syn_node_decl(&self) -> TraitForTypeItemSynNodeDecl {
        let db = self.db();
        let ItemSynNodePath::AssociatedItem(AssociatedItemSynNodePath::TraitForTypeItem(
            syn_node_path,
        )) = self.syn_node_path()
        else {
            unreachable!()
        };
        match syn_node_path.data(db).item_kind(db) {
            TraitItemKind::MemoizedField => todo!(),
            TraitItemKind::MethodFn => self
                .parse_trai_for_ty_method_fn_node_decl(syn_node_path)
                .into(),
            TraitItemKind::AssociatedType => self
                .parse_trai_for_ty_associated_ty_node_decl(syn_node_path)
                .into(),
            TraitItemKind::AssociatedVal => todo!(),
            TraitItemKind::AssociatedFunctionFn => self
                .parse_trai_for_ty_associated_fn_node_decl(syn_node_path)
                .into(),
            TraitItemKind::AssociatedFunctionGn => todo!(),
            TraitItemKind::AssociatedFormal => todo!(),
            TraitItemKind::AssociatedConst => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum TraitForTypeItemSynDecl {
    AssociatedFn(TraitForTypeAssociatedFnSynDecl),
    MethodFn(TraitForTypeMethodFnSynDecl),
    AssociatedType(TraitForTypeAssociatedTypeSynDecl),
    AssociatedVal(TraitForTypeAssociatedValSynDecl),
}

impl From<TraitForTypeItemSynDecl> for SynDecl {
    fn from(decl: TraitForTypeItemSynDecl) -> Self {
        SynDecl::AssociatedItem(decl.into())
    }
}

/// # constructor
impl TraitForTypeItemSynDecl {
    fn from_node_decl(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        syn_node_decl: TraitForTypeItemSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match syn_node_decl {
            TraitForTypeItemSynNodeDecl::AssociatedFn(syn_node_decl) => {
                TraitForTypeAssociatedFnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TraitForTypeItemSynNodeDecl::MethodFn(syn_node_decl) => {
                TraitForTypeMethodFnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TraitForTypeItemSynNodeDecl::AssociatedType(syn_node_decl) => {
                TraitForTypeAssociatedTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TraitForTypeItemSynNodeDecl::AssociatedVal(_) => todo!(),
        })
    }
}

/// # getters
impl TraitForTypeItemSynDecl {
    pub fn path(self, db: &::salsa::Db) -> TraitForTypeItemPath {
        match self {
            TraitForTypeItemSynDecl::AssociatedFn(decl) => decl.path(db),
            TraitForTypeItemSynDecl::MethodFn(decl) => decl.path(db),
            TraitForTypeItemSynDecl::AssociatedType(decl) => decl.path(db),
            TraitForTypeItemSynDecl::AssociatedVal(decl) => decl.path(db),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            TraitForTypeItemSynDecl::AssociatedFn(decl) => decl.template_parameters(db),
            TraitForTypeItemSynDecl::MethodFn(decl) => decl.template_parameters(db),
            TraitForTypeItemSynDecl::AssociatedType(decl) => decl.template_parameters(db),
            TraitForTypeItemSynDecl::AssociatedVal(_) => &[],
        }
    }

    pub fn parenate_parameters<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> Option<&'a [ParenateSynParameterData]> {
        match self {
            TraitForTypeItemSynDecl::AssociatedFn(syn_decl) => {
                Some(syn_decl.parenate_parameters(db))
            }
            TraitForTypeItemSynDecl::MethodFn(syn_decl) => Some(syn_decl.parenate_parameters(db)),
            TraitForTypeItemSynDecl::AssociatedType(_) => None,
            TraitForTypeItemSynDecl::AssociatedVal(_) => None,
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            TraitForTypeItemSynDecl::AssociatedFn(decl) => decl.syn_expr_region(db),
            TraitForTypeItemSynDecl::MethodFn(decl) => decl.syn_expr_region(db),
            TraitForTypeItemSynDecl::AssociatedType(decl) => decl.syn_expr_region(db),
            TraitForTypeItemSynDecl::AssociatedVal(decl) => decl.syn_expr_region(db),
        }
    }
}

impl HasSynDecl for TraitForTypeItemPath {
    type Decl = TraitForTypeItemSynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl> {
        trai_for_ty_item_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_for_ty_item_syn_decl(
    db: &::salsa::Db,
    path: TraitForTypeItemPath,
) -> DeclResult<TraitForTypeItemSynDecl> {
    let syn_node_decl = path.syn_node_path(db).syn_node_decl(db);
    TraitForTypeItemSynDecl::from_node_decl(db, path, syn_node_decl)
}
