use super::*;
use husky_entity_path::{SubmoduleItemPath, TraitPath, TypeVariantPath};
use husky_hir_defn::{HasHirDefn, TypeHirDefn};
use husky_linkage::template_argument::{qual::LinQual, ty::LinType};
use husky_manifest::PackageDependency;

impl<'a, 'b> RustTranspilationBuilder<'a, 'b> {
    pub(crate) fn pub_use_all_in_submodule(&mut self, submodule_path: SubmoduleItemPath) {
        let db = self.db;
        self.on_fresh_semicolon_line(|builder| {
            builder.write_str("pub use self::");
            builder.write_str(submodule_path.ident(db).data(db));
            builder.write_str("::*")
        })
    }

    pub(crate) fn use_all_in_dep(&mut self, dep: &PackageDependency) {
        let db = self.db;
        self.on_fresh_semicolon_line(|builder| {
            builder.write_str("use ");
            builder.write_str(dep.package_path().ident(db).data(db));
            builder.write_str("::*")
        })
    }
}

impl<'a, 'b, HirEagerExprRegion> RustTranspilationBuilder<'a, 'b, HirEagerExprRegion> {
    pub(crate) fn one(&mut self) {
        self.write_str("1")
    }

    pub(crate) fn call_rev(&mut self) {
        self.write_str(".rev()")
    }

    pub(crate) fn struct_ty_constructor_path(&mut self, ty_path: TypePath) {
        ty_path.transpile_to_rust(self);
        match ty_path.hir_defn(self.db).unwrap() {
            TypeHirDefn::PropsStruct(_) => self.write_str("::__constructor"),
            TypeHirDefn::TupleStruct(_) => (),
            _ => unreachable!(),
        }
    }

    pub(crate) fn struct_ty_constructor_ident(&mut self) {
        self.write_str("__constructor")
    }

    pub(crate) fn enum_ty_variant_constructor_path(&mut self, path: TypeVariantPath) {
        path.parent_ty_path(self.db).transpile_to_rust(self);
        self.punctuation(RustPunctuation::ColonColon);
        self.enum_ty_variant_constructor_ident(path)
    }

    pub(crate) fn enum_ty_variant_constructor_ident(&mut self, ty_variant_path: TypeVariantPath) {
        let db = self.db;
        self.write_str("__");
        self.write_str(ty_variant_path.ident(db).data(db));
        self.write_str("_constructor");
    }

    fn qual_suffix(&mut self, qual: LinQual) {
        self.write_str(match qual {
            LinQual::Ref => "_ref",
            LinQual::RefMut => "_mut",
            LinQual::Transient => "",
        });
    }

    pub(crate) fn use_all_in_crate(&mut self) {
        self.on_fresh_semicolon_line(|builder| builder.write_str("use crate::*"))
    }

    pub(crate) fn use_all_in_super(&mut self) {
        self.on_fresh_semicolon_line(|builder| builder.write_str("use super::*"))
    }

    pub(crate) fn r8(&mut self) {
        self.write_str("r8")
    }

    pub(crate) fn r16(&mut self) {
        self.write_str("u16")
    }

    pub(crate) fn r32(&mut self) {
        self.write_str("u32")
    }

    pub(crate) fn r64(&mut self) {
        self.write_str("u64")
    }

    pub(crate) fn r128(&mut self) {
        self.write_str("u128")
    }

    pub(crate) fn rsize(&mut self) {
        self.write_str("usize")
    }

    pub(crate) fn usize(&mut self) {
        self.write_str("usize")
    }

    pub(crate) fn unit(&mut self) {
        self.write_str("()")
    }

    pub(crate) fn wrap_in_some_left(&mut self, wrap_in_some_flag: &mut bool) {
        debug_assert!(!*wrap_in_some_flag);
        *wrap_in_some_flag = true;
        self.write_str("Some(")
    }

    pub(crate) fn wrap_in_some_right(&mut self) {
        self.write_str(")")
    }

    pub(crate) fn eq_zero(&mut self) {
        self.write_str(" == 0")
    }

    pub(crate) fn ne_zero(&mut self) {
        self.write_str(" != 0")
    }

    pub(crate) fn method_fn_ident_mut(&mut self, ident: Ident) {
        let db = self.db;
        self.write_str(ident.data(db));
        self.write_str("_mut")
    }
}

impl<'a, 'b, E> RustTranspilationBuilder<'a, 'b, E> {
    pub(crate) fn todo(&mut self) {
        self.write_str("todo!()")
    }

    pub(crate) fn cyclic_slice_leashed_ty(&mut self) {
        self.write_str("CyclicSliceLeashed")
    }

    pub(crate) fn lpar(&mut self) {
        self.write_str("(")
    }

    pub(crate) fn rpar(&mut self) {
        self.write_str(")")
    }

    pub(crate) fn derive(&mut self, trais: &[TraitPath]) {
        self.on_fresh_line(|builder| {
            builder.write_str("#[derive");
            builder.delimited_comma_list(RustDelimiter::Par, trais);
            builder.write_str("]\n")
        })
    }

    pub(crate) fn omitted_curly_block(&mut self) {
        self.write_str(" {}")
    }

    pub(crate) fn rustfmt_skip(&mut self) {
        self.result += "#[rustfmt::skip]\n"
    }

    pub(crate) fn crate_(&mut self) {
        self.result += "crate"
    }

    pub(crate) fn husky_core(&mut self) {
        self.result += "husky_core"
    }

    pub(crate) fn ad_hoc_fn(&mut self) {
        self.result += "fn(/* ad hoc*/)"
    }

    pub(crate) fn value_conversion(&mut self) {
        use std::fmt::Write;

        let db = self.db;
        let task_dep = self
            .rust_transpilation_setup_data
            .task_dependency_ident
            .data(db);
        write!(self.result, "#[{}::value_conversion]\n", task_dep).unwrap()
    }

    pub(crate) fn call_unwrap(&mut self) {
        self.result += ".unwrap()"
    }

    pub(crate) fn type_runtime_const_symbols_is(&mut self) {
        self.result += "type RuntimeConstSymbols = "
    }

    pub(crate) fn copy_trait(&mut self) {
        self.result += "Copy"
    }

    pub(crate) fn v(&mut self) {
        self.result += "v"
    }

    pub(crate) fn vec_ty(&mut self, element_ty: LinType) {
        self.result += "Vec";
        self.delimited(RustDelimiter::Angle, |builder| {
            element_ty.transpile_to_rust(builder)
        })
    }

    pub(crate) fn static_lifetime(&mut self) {
        self.result += "'static"
    }

    pub(crate) fn visual_synchrotron_parameter_decl(&mut self) {
        self.result += ", __visual_synchrotron: &mut __VisualSynchrotron"
    }

    pub(crate) fn visual_synchrotron_argument(&mut self) {
        self.result += "__visual_synchrotron"
    }

    pub(crate) fn tuple_field(&mut self, index: u8) {
        use std::fmt::Write;

        write!(self.result, ".{index}").unwrap();
    }

    pub(crate) fn enum_tuple_variant_field(&mut self, index: usize) {
        use std::fmt::Write;

        write!(self.result, "v{index}").unwrap();
    }
}
