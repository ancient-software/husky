use super::*;
use husky_entity_syn_tree::{helpers::TraitInUseItemsTable, EntityTreeResultRef};
use husky_vfs::VfsPathMenu;

pub trait FluffyTermEngine<'a>: Sized {
    fn db(&self) -> &'a dyn FluffyTermDb;
    fn trai_in_use_items_table(&self) -> EntityTreeResultRef<'a, TraitInUseItemsTable<'a>>;
    fn fluffy_term_region(&self) -> &FluffyTermRegion;
    fn fluffy_term_region_mut(&mut self) -> &mut FluffyTermRegion;
    fn fluffy_terms(&self) -> &FluffyTerms {
        self.fluffy_term_region().terms()
    }
    fn fluffy_terms_mut(&mut self) -> &mut FluffyTerms {
        self.fluffy_term_region_mut().terms_mut()
    }
    fn item_path_menu(&self) -> &'a ItemPathMenu;
    fn term_menu(&self) -> &'a EtherealTermMenu;
    fn expr_region_data(&self) -> &'a SynExprRegionData;

    fn new_ty_ontology_application(
        &mut self,
        src: HoleSource,
        path: TypePath,
        arguments: SmallVec<[FluffyTerm; 2]>,
    ) -> FluffyTerm {
        todo!()
    }

    fn new_hole(&mut self, src: impl Into<HoleSource>, hole_kind: HoleKind) -> FluffyTerm {
        HollowTerm::new_hole(self, src, hole_kind).into()
    }

    fn synthesize_function_application_expr_ty(
        &mut self,
        variance: Variance,
        parameter_symbol: Option<FluffyTerm>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
        argument_ty: FluffyTerm,
        shift: i8,
    ) -> FluffyTermResult<FluffyTerm> {
        debug_assert!(shift >= 0);
        if shift == 0 {
            return Ok(return_ty);
        }
        debug_assert_eq!(parameter_ty.place(), None);
        debug_assert_eq!(return_ty.place(), None);
        match (
            parameter_symbol.map(|symbol| symbol.base_resolved(self)),
            parameter_ty.base_resolved(self),
            return_ty.base_resolved(self),
            argument_ty.base_resolved(self),
        ) {
            (
                None,
                FluffyTermBase::Ethereal(parameter_ty),
                FluffyTermBase::Ethereal(return_ty),
                FluffyTermBase::Ethereal(argument_ty),
            ) => {
                return Ok(EtherealTerm::synthesize_function_application_expr_ty(
                    self.db(),
                    variance,
                    None,
                    parameter_ty,
                    return_ty,
                    argument_ty,
                    shift,
                )?
                .into())
            }
            _ => (),
        }
        if parameter_symbol.is_some() {
            todo!()
        }
        match argument_ty.data(self) {
            FluffyTermData::Curry {
                curry_kind: argument_curry_kind,
                variance: argument_variance,
                parameter_variable: argument_parameter_variable,
                parameter_ty: argument_parameter_ty,
                return_ty: argument_return_ty,
                ..
            } => {
                if argument_parameter_variable.is_some() {
                    todo!()
                }
                let expr_ty = self.synthesize_function_application_expr_ty(
                    variance,
                    parameter_symbol,
                    parameter_ty,
                    return_ty,
                    argument_return_ty,
                    shift - 1,
                );
                todo!()
                // FluffyTerm::new_curry()
            }
            _ => unreachable!(),
        }
    }
}
