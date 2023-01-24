#![feature(trait_upcasting)]
mod collector;
mod db;
mod defn;
mod error;
mod sheet;
#[cfg(test)]
mod tests;

pub use db::*;
pub use defn::*;
pub use error::*;
pub use sheet::*;

use collector::*;
use husky_decl::*;
use husky_entity_path::*;
use husky_entity_tree::EntityTreeResult;
use husky_expr::ExprIdx;
use husky_expr::*;
use husky_vfs::{ModulePath, VfsResult};
use salsa::DbWithJar;

#[salsa::jar(db = DefnDb)]
pub struct DefnJar(
    defn_sheet,
    // type
    EnumTypeDefn,
    UnitStructTypeDefn,
    TupleStructTypeDefn,
    RegularStructTypeDefn,
    RecordTypeDefn,
    InductiveTypeDefn,
    StructureTypeDefn,
    AlienTypeDefn,
    UnionTypeDefn,
    // form
    ValueDefn,
    FeatureDefn,
    FunctionDefn,
    MorphismDefn,
    TypeAliasDefn,
    // trait
    TraitDefn,
    // enum variant,
    UnitVariantDefn,
    TupleVariantDefn,
    PropsVariantDefn,
    // type item
    TypeAssociatedFunctionDefn,
    TypeMethodDefn,
    TypeAssociatedTypeDefn,
    TypeAssociatedValueDefn,
    TypeMemoDefn,
    // trait item
    TraitAssociatedFunctionDefn,
    TraitMethodDefn,
    TraitAssociatedTypeDefn,
    TraitAssociatedValueDefn,
    // type as trait item
    TypeAsTraitAssociatedFunctionDefn,
    TypeAsTraitMethodDefn,
    TypeAsTraitAssociatedTypeDefn,
    TypeAsTraitAssociatedValueDefn,
);

#[test]
fn defn_sheet_works() {
    use husky_vfs::VfsTestUtils;
    use tests::*;

    DB::default().vfs_expect_test_debug_with_db("defn_sheet", DefnDb::defn_sheet);
}
