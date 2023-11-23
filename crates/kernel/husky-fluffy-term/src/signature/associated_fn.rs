use super::*;
use husky_coword::Ident;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub struct AssociatedFnFluffySignature {
    path: AssociatedItemPath,
    parenate_parameters: SmallVec<[FluffyTermRitchieParameter; 4]>,
    return_ty: FluffyTerm,
    ty: FluffyTerm,
}

impl AssociatedFnFluffySignature {
    pub fn parenate_parameter_contracted_tys(&self) -> &[FluffyTermRitchieParameter] {
        &self.parenate_parameters
    }

    pub fn ty(&self) -> FluffyTerm {
        self.ty
    }

    pub fn path(&self) -> AssociatedItemPath {
        self.path
    }
}

pub(crate) fn ty_associated_fn_fluffy_signature<Term: Copy + Into<FluffyTerm>>(
    engine: &mut impl FluffyTermEngine,
    expr_idx: SynExprIdx,
    template: TypeAssociatedFnEtherealSignatureTemplate,
    ty_template_arguments: &[Term],
    associated_fn_template_arguments: &[FluffyTerm],
) -> FluffyTermMaybeResult<AssociatedFnFluffySignature> {
    let db = engine.db();
    let self_ty_application_expansion = template.self_ty(db).application_expansion(db);
    if self_ty_application_expansion.arguments(db).len() != ty_template_arguments.len() {
        todo!()
    }
    p!(template.path(db).debug(db));
    let mut instantiation_builder = FluffyInstantiationBuilder::new_associated(
        FluffyInstantiationEnvironment::AssociatedFn,
        template
            .path(db)
            .impl_block(db)
            .ethereal_signature_template(db)?
            .template_parameters(db),
        template.template_parameters(db),
        db,
    );
    // FluffyInstantiation::new(FluffyInstantiationEnvironment::AssociatedFn);
    // initialize pattern matcher
    std::iter::zip(
        self_ty_application_expansion.arguments(db).iter().copied(),
        ty_template_arguments.iter().copied(),
    )
    .try_for_each(|(src, dst)| instantiation_builder.try_add_rule(src, dst.into()))?;
    let mut associated_fn_template_argument_iter = associated_fn_template_arguments.iter();
    for _ in template.template_parameters(db).iter() {
        todo!()
    }
    JustOk(AssociatedFnFluffySignature {
        path: template.path(db).into(),
        parenate_parameters: template
            .parenate_parameters(db)
            .iter()
            .map(|param| param.instantiate(engine, expr_idx, &mut instantiation_builder))
            .collect(),
        return_ty: template
            .return_ty(db)
            .instantiate(engine, expr_idx, &mut instantiation_builder),
        ty: template
            .ty(db)
            .instantiate(engine, expr_idx, &mut instantiation_builder),
    })
}
