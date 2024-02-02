use crate::*;

use husky_vfs::ModulePath;

pub trait SynDeclDb {
    fn syn_node_decl_sheet(&self, module_path: ModulePath) -> SynNodeDeclSheet;

    fn syn_decl_sheet(&self, module_path: ModulePath) -> SynDeclSheet;
}

impl SynDeclDb for ::salsa::Db {
    fn syn_node_decl_sheet(&self, module_path: ModulePath) -> SynNodeDeclSheet {
        syn_node_decl_sheet(self, module_path)
    }

    fn syn_decl_sheet(&self, module_path: ModulePath) -> SynDeclSheet {
        syn_decl_sheet(self, module_path)
    }
}

#[salsa::jar]
pub struct SynDeclJar(
    // decl
    // - submodule
    SubmoduleSynNodeDecl,
    submodule_syn_node_decl,
    SubmoduleSynDecl,
    submodule_decl,
    // - type
    ty_node_decl,
    ty_decl,
    EnumTypeSynNodeDecl,
    EnumTypeSynDecl,
    UnitStructTypeSynNodeDecl,
    UnitStructTypeSynDecl,
    TupleStructTypeSynNodeDecl,
    TupleStructTypeSynDecl,
    PropsStructTypeSynNodeDecl,
    PropsStructTypeSynDecl,
    RecordTypeSynNodeDecl,
    RecordTypeSynDecl,
    InductiveTypeSynNodeDecl,
    InductiveTypeSynDecl,
    StructureTypeSynNodeDecl,
    StructureTypeSynDecl,
    ExternTypeSynNodeDecl,
    ExternTypeSynDecl,
    UnionTypeSynNodeDecl,
    UnionTypeSynDecl,
    // - trait
    TraitSynNodeDecl,
    trai_syn_node_decl,
    TraitSynDecl,
    trai_syn_decl,
    // - fugitive
    fugitive_syn_node_decl,
    fugitive_syn_decl,
    MajorValSynNodeDecl,
    MajorValSynDecl,
    MajorFnSynNodeDecl,
    FunctionMajorFnSynDecl,
    MajorGnSynNodeDecl,
    MajorGnSynDecl,
    TypeAliasSynNodeDecl,
    TypeAliasSynDecl,
    // - impl block
    TypeImplBlockSynNodeDecl,
    ty_impl_block_syn_node_decl,
    TypeImplBlockSynDecl,
    ty_impl_block_syn_decl,
    TraitForTypeImplBlockSynNodeDecl,
    trai_for_ty_impl_block_syn_node_decl,
    TraitForTypeImplBlockSynDecl,
    trai_for_ty_impl_block_syn_decl,
    IllFormedImplBlockSynNodeDecl,
    ill_formed_impl_block_syn_node_decl,
    // - variant
    ty_variant_syn_node_decl,
    ty_variant_syn_decl,
    TypeUnitVariantSynNodeDecl,
    TypeUnitVariantSynDecl,
    TypePropsVariantSynNodeDecl,
    TypePropsVariantSynDecl,
    TypeTupleVariantSynNodeDecl,
    TypeTupleVariantSynDecl,
    // - associated items
    // -- type item
    ty_item_syn_node_decl,
    ty_item_syn_decl,
    TypeAssociatedFnSynNodeDecl,
    TypeAssociatedFnSynDecl,
    TypeMethodFnSynNodeDecl,
    TypeMethodFnSynDecl,
    TypeAssociatedTypeSynNodeDecl,
    TypeAssociatedTypeSynDecl,
    TypeAssociatedValSynNodeDecl,
    TypeAssociatedValSynDecl,
    TypeMemoizedFieldSynNodeDecl,
    TypeMemoizedFieldSynDecl,
    // -- trait item
    trai_item_syn_node_decl,
    trai_item_syn_decl,
    TraitAssociatedFnSynNodeDecl,
    TraitAssociatedFnSynDecl,
    TraitMethodFnSynNodeDecl,
    TraitMethodFnSynDecl,
    TraitAssociatedTypeSynNodeDecl,
    TraitAssociatedTypeSynDecl,
    TraitAssociatedValSynNodeDecl,
    TraitAssociatedValSynDecl,
    // -- trait for type item
    trai_for_ty_item_syn_decl,
    trai_for_ty_item_syn_node_decl,
    TraitForTypeAssociatedFnSynNodeDecl,
    TraitForTypeAssociatedFnSynDecl,
    TraitForTypeMethodFnSynNodeDecl,
    TraitForTypeMethodFnSynDecl,
    TraitForTypeAssociatedTypeSynNodeDecl,
    TraitForTypeAssociatedTypeSynDecl,
    TraitForTypeAssociatedValSynNodeDecl,
    TraitForTypeAssociatedValSynDecl,
    // -- ill formed item
    IllFormedItemSynNodeDecl,
    // attr
    DeriveAttrSynDecl,
    DeriveAttrSynNodeDecl,
    attr_syn_node_decl,
    attr_syn_decl,
    // sheet
    SynNodeDeclSheet,
    syn_node_decl_sheet,
    SynDeclSheet,
    syn_decl_sheet,
);
