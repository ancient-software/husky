use crate::*;
use ast::{AstText, AstVariant};
use entity_route::EntityRouteKind;
use file::FilePtr;
use fold::FoldableStorage;
use semantics_eager::parse_func_stmts;
use semantics_entity::EntityDefnQueryGroup;
use semantics_error::*;

#[salsa::query_group(PackageQueryGroupStorage)]
pub trait PackageQueryGroup: EntityDefnQueryGroup {
    fn package(&self, main_file: file::FilePtr) -> SemanticResultArc<Package>;
    fn config(&self, main_file: file::FilePtr) -> SemanticResultArc<Config>;
}

fn package(db: &dyn PackageQueryGroup, main_file: file::FilePtr) -> SemanticResultArc<Package> {
    let module = db.module(main_file).unwrap();
    let ident = match module.kind {
        EntityRouteKind::Package { ident, .. } => ident,
        _ => panic!(),
    };
    Ok(Arc::new(Package {
        ident,
        subentity_defns: db.subentity_defns(module)?,
        main_defn: db.main_defn(main_file)?,
        config: db.config(main_file)?,
    }))
}

fn config(this: &dyn PackageQueryGroup, main_file: file::FilePtr) -> SemanticResultArc<Config> {
    let ast_text = this.ast_text(main_file).unwrap();
    config_from_ast(this, &ast_text, main_file)
}

fn config_from_ast(
    this: &dyn PackageQueryGroup,
    ast_text: &AstText,
    file: FilePtr,
) -> SemanticResultArc<Config> {
    Ok(Arc::new(Config {
        dataset: dataset_config_from_ast_text(this, ast_text, file)?,
    }))
}

fn dataset_config_from_ast_text(
    this: &dyn PackageQueryGroup,
    ast_text: &AstText,
    file: FilePtr,
) -> SemanticResult<DatasetConfig> {
    for item in ast_text.folded_results.iter() {
        match item.value.as_ref().unwrap().variant {
            AstVariant::DatasetConfigDefnHead => {
                return Ok(DatasetConfig::new(parse_func_stmts(
                    this.upcast(),
                    &ast_text.arena,
                    not_none!(item.opt_children),
                    file,
                )?))
            }
            _ => (),
        }
    }
    err!("dataset config not found")
}
