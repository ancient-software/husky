use husky_rust_code_repr::registration::NonPrimitiveTypeRegistration;
use husky_write_utils::w;
use std::fmt::Write;

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_registration_content(&mut self) {
        self.write(
            r#"use crate::*;
pub(crate) use __husky::registration::*;

"#,
        );
        let main_module = self.db.module(self.target_entrance).unwrap();
        let entity_link_dependees = self.db.entity_link_dependees(main_module);
        for entity_route in entity_link_dependees.iter() {
            if !entity_route.contains_any() {
                let entity_defn = self.db.entity_defn(*entity_route).unwrap();
                self.might_gen_ty_registration(*entity_route, &entity_defn);
            }
        }
    }

    fn might_gen_ty_registration(&mut self, entity_route: Ty, entity_defn: &EntityDefn) {
        if self.db.is_defn_static(entity_route)
            && !self.db.contains_spatial_parameters(entity_route)
        {
            return;
        }
        match entity_defn.variant {
            EntityDefnVariant::Ty { ty_kind, .. } => {
                match ty_kind {
                    TyKind::Record => return,
                    TyKind::HigherKind => return,
                    _ => (),
                }
                self.gen_ty_registration(entity_route)
            }
            _ => (),
        }
    }

    fn gen_ty_registration(&mut self, entity_route: Ty) {
        let mangled_intrinsic_ty = self.db.mangled_intrinsic_ty(entity_route);
        let needs_eval_ref = self.db.entity_route_contains_eval_ref(entity_route);
        write!(
            self.result,
            "type {}{} = ",
            mangled_intrinsic_ty,
            match needs_eval_ref {
                true => "<'eval>",
                false => "",
            }
        )
        .unwrap();
        self.gen_entity_route(entity_route, EntityRouteRole::Decl);
        self.write(";\n");
        w!(self.result; (NonPrimitiveTypeRegistration { ty: &mangled_intrinsic_ty }))
    }
}
