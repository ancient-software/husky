use crate::*;
use either::*;
use husky_corgi_config::transpilation_setup::TranspilationSetup;
use husky_entity_path::{
    AssociatedItemPath, FugitivePath, MajorItemPath, PatternPath, PreludeIntTypePath,
    PreludeNumTypePath, PreludeTypePath, PrincipalEntityPath, TraitForTypeItemPath, TraitItemPath,
    TraitPath, TypeItemPath, TypePath, TypeSketch, TypeVariantPath,
};
use husky_ethereal_signature::signature::HasEtherealSignatureTemplate;
use husky_hir_ty::HirType;
use husky_javelin::{javelin::JavelinData, path::JavelinPath};
use husky_linkage::{
    instantiation::{LinkageInstantiate, LinkageInstantiation},
    template_argument::{
        ty::{LinkageType, LinkageTypeRitchie},
        LinkageTemplateArgument,
    },
};
use husky_linkage::{
    linkage::{package_linkages, Linkage, LinkageData},
    template_argument::ty::LinkageTypePathLeading,
};
use husky_vfs::{CrateKind, ModulePathData, PackagePathSource};
use salsa::DebugWithDb;

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn package_linkages_transpilation(
    db: &::salsa::Db,
    package_path: PackagePath,
    setup: TranspilationSetup,
) -> String {
    let mut builder_base = RustTranspilationBuilderBase::new(
        db,
        package_path.toolchain(db),
        setup,
        Some(format!(
            r#"use husky_core::*;
use {}::*;
use {}::*;
"#,
            setup.rust_data(db).unwrap().task_dependency_ident.data(db),
            package_path.ident(db).data(db)
        )),
        None,
    );
    let mut builder = RustTranspilationBuilder::new(&mut builder_base);
    builder.on_fresh_semicolon_paragraph(|builder| {
        builder.rustfmt_skip();
        builder.macro_name(RustMacroName::LinkageImpls);
        builder.bracketed_multiline_comma_list(RustBracket::Box, package_linkages(db, package_path))
    });
    builder_base.finish()
}

impl TranspileToRustWith<()> for Linkage {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<()>) {
        let db = builder.db;
        match *self.data(db) {
            LinkageData::FunctionFnItem {
                path,
                ref instantiation,
            } => path.transpile_to_rust(builder),
            LinkageData::ValItem {
                path,
                ref instantiation,
            } => path.transpile_to_rust(builder),
            LinkageData::MethodFn {
                path,
                ref instantiation,
            } => (path, instantiation).transpile_to_rust(builder),
            LinkageData::TypeConstructor {
                path,
                ref instantiation,
            } => builder.ty_constructor(path),
            LinkageData::AssociatedFunctionFn {
                path,
                ref instantiation,
            } => path.transpile_to_rust(builder),
            LinkageData::MemoizedField {
                path,
                ref instantiation,
            } => path.transpile_to_rust(builder),
            LinkageData::PropsStructField { .. } => todo!(),
            LinkageData::Index => todo!(),
            LinkageData::TypeVariantConstructor {
                path,
                ref instantiation,
            } => path.transpile_to_rust(builder),
        }
    }
}

impl TranspileToRustWith<()> for JavelinPath {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<()>) {
        match self {
            JavelinPath::Fugitive(slf) => slf.transpile_to_rust(builder),
            JavelinPath::TypeItem(slf) => slf.transpile_to_rust(builder),
            JavelinPath::TraitItem(slf) => slf.transpile_to_rust(builder),
            JavelinPath::TraitForTypeItem(slf) => slf.transpile_to_rust(builder),
            JavelinPath::TypeConstructor(slf) => slf.transpile_to_rust(builder),
            JavelinPath::TypeVariantConstructor(slf) => slf.transpile_to_rust(builder),
        }
    }
}

impl<E> TranspileToRustWith<E> for (AssociatedItemPath, &LinkageInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, instantiation) = self;
        match path {
            AssociatedItemPath::TypeItem(slf) => (slf, instantiation).transpile_to_rust(builder),
            AssociatedItemPath::TraitItem(slf) => (slf, instantiation).transpile_to_rust(builder),
            AssociatedItemPath::TraitForTypeItem(slf) => {
                (slf, instantiation).transpile_to_rust(builder)
            }
        }
    }
}

impl<E> TranspileToRustWith<E> for (TypeItemPath, &LinkageInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, linkage_instantiation) = self;
        let db = builder.db;
        use husky_print_utils::p;
        use salsa::DebugWithDb;
        p!(self.debug(db));
        let self_ty = HirType::from_ethereal(
            path.impl_block(db)
                .ethereal_signature_template(db)
                .unwrap()
                .self_ty(db),
            db,
        )
        .unwrap()
        .linkage_instantiate(linkage_instantiation, db);
        builder.bracketed(RustBracket::Angle, |builder| {
            self_ty.transpile_to_rust(builder)
        });
        builder.punctuation(RustPunctuation::ColonColon);
        path.ident(db).transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for LinkageType {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        match self {
            LinkageType::PathLeading(slf) => slf.transpile_to_rust(builder),
            LinkageType::Ritchie(slf) => slf.transpile_to_rust(builder),
        }
    }
}

impl<E> TranspileToRustWith<E> for LinkageTypePathLeading {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db;
        self.ty_path(db).transpile_to_rust(builder);
        let template_arguments = self.template_arguments(db);
        if !template_arguments.is_empty() {
            builder.bracketed_comma_list(RustBracket::Angle, template_arguments)
        }
    }
}

impl<E> TranspileToRustWith<E> for LinkageTemplateArgument {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        match self {
            LinkageTemplateArgument::Vacant => todo!(),
            LinkageTemplateArgument::Type(linkage_ty) => linkage_ty.transpile_to_rust(builder),
            LinkageTemplateArgument::Constant(_) => todo!(),
            LinkageTemplateArgument::Lifetime => todo!(),
            LinkageTemplateArgument::Place(_) => todo!(),
        }
    }
}

impl<E> TranspileToRustWith<E> for LinkageTypeRitchie {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        builder.ad_hoc_fn()
    }
}

impl<E> TranspileToRustWith<E> for (TraitItemPath, &LinkageInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, instantiation) = self;
        let db = builder.db;
        path.trai_path(db).transpile_to_rust(builder);
        builder.punctuation(RustPunctuation::ColonColon);
        path.ident(db).transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for (TraitForTypeItemPath, &LinkageInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, instantiation) = self;
        let db = builder.db;
        builder.bracketed(RustBracket::Angle, |builder| {
            match path.impl_block(db).ty_sketch(db) {
                TypeSketch::DeriveAny => builder.todo(),
                TypeSketch::Path(path) => path.transpile_to_rust(builder),
            }
            builder.keyword(RustKeyword::As);
            path.impl_block(db).trai_path(db).transpile_to_rust(builder)
        });
        builder.punctuation(RustPunctuation::ColonColon);
        path.ident(db).transpile_to_rust(builder)
    }
}
