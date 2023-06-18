use super::*;
use husky_defn::*;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct DefnDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn defn_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> DefnDiagnosticSheet {
    let mut sheet_collector = SheetDiagnosticsCollector::new(db, module_path);
    if let (Ok(ranged_token_sheet), Ok(defn_sheet)) = (
        db.ranged_token_sheet(module_path),
        db.collect_defns(module_path),
    ) {
        let _token_sheet_data = ranged_token_sheet.token_sheet_data(db);
        for (_path, defn) in defn_sheet.defns() {
            if let Ok(defn) = defn {
                if let Some(expr_region) = defn.expr_region(db) {
                    let mut region_collector =
                        RegionDiagnosticsCollector::new(db, expr_region, &mut sheet_collector);
                    region_collector.visit_defn(defn)
                }
            }
        }
    }
    // todo
    DefnDiagnosticSheet::new(db, sheet_collector.finish())
}

impl<'a, 'b> RegionDiagnosticsCollector<'a, 'b> {
    fn visit_defn(&mut self, defn: Defn) {
        // todo
        match defn {
            Defn::ModuleItem(_) => (),
            // Defn::Type(Defn) => match Defn {
            //     TypeDefn::Enum(_Defn) => (),
            //     TypeDefn::RegularStruct(_Defn) => (),
            //     TypeDefn::UnitStruct(_Defn) => (),
            //     TypeDefn::TupleStruct(_Defn) => (),
            //     TypeDefn::Record(_Defn) => (),
            //     TypeDefn::Inductive(_Defn) => (),
            //     TypeDefn::Structure(_Defn) => (),
            //     TypeDefn::Extern(_Defn) => (),
            //     TypeDefn::Union(_Defn) => (),
            // },
            // Defn::Fugitive(_Defn) => (),
            // Defn::Trait(_Defn) => (),
            Defn::ImplBlock(_Defn) => (),
            Defn::AssociatedItem(_Defn) => (),
            Defn::TypeVariant(_Defn) => (),
        }
    }
}
