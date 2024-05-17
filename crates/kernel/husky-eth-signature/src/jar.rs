#[salsa::jar]
pub struct EthSignatureJar(
    // assoc_items
    // - type items
    crate::signature::assoc_item::ty_item::ty_item_eth_templates_map,
    crate::signature::assoc_item::ty_item::method_ritchie::TypeMethodRitchieEthTemplate,
    crate::signature::assoc_item::ty_item::method_curry::TypeMethodCurryEthTemplate,
    crate::signature::assoc_item::ty_item::memo_field::TypeMemoizedFieldEthTemplate,
    crate::signature::assoc_item::ty_item::assoc_ritchie::TypeAssocRitchieEthTemplate,
    crate::signature::assoc_item::ty_item::assoc_ty::TypeAssocTypeEthTemplate,
    // ty_item_eth_template,
    // - trait items
    crate::signature::assoc_item::trai_item::assoc_ritchie::TraitAssocRitchieEthTemplate,
    crate::signature::assoc_item::trai_item::method_ritchie::TraitMethodFnEthTemplate,
    // - trait for type items
    crate::signature::assoc_item::trai_for_ty_item::assoc_ritchie::TraitForTypeAssocRitchieEthTemplate,
    crate::signature::assoc_item::trai_for_ty_item::assoc_val::TraitForTypeAssocValEthTemplate,
    crate::signature::assoc_item::trai_for_ty_item::assoc_ty::TraitForTypeAssocTypeEthTemplate,
    crate::signature::assoc_item::trai_for_ty_item::assoc_ty::TraitForTypeAssocTypeEtherealSignatureBuilder,
    // trai_for_ty_item_eth_template,
    crate::signature::assoc_item::trai_for_ty_item::assoc_ty::trai_for_ty_assoc_ty_ethereal_signature_signature_builder_try_into_signature,
    crate::signature::assoc_item::trai_for_ty_item::method_ritchie::TraitForTypeMethodRitchieEthTemplate,
    crate::signature::assoc_item::trai_for_ty_item::method_ritchie::TraitForTypeMethodRitchieEtherealSignatureBuilder,
    crate::signature::assoc_item::trai_for_ty_item::method_ritchie::trai_for_ty_method_ritchie_ethereal_signature_signature_builder_try_into_signature,
    // trai
    crate::signature::major_item::trai::TraitEthTemplate,
    crate::signature::major_item::trai::trai_eth_template,
    // Form
    crate::signature::major_item::form::form_eth_template,
    crate::signature::major_item::form::function_ritchie::MajorFunctionRitchieEthTemplate,
    crate::signature::major_item::form::ty_alias::MajorTypeAliasEthTemplate,
    crate:: signature::major_item::form::val::MajorValEthTemplate,
    // ty
    crate::signature::major_item::ty::r#enum::EnumEthTemplate,
    crate::signature::major_item::ty::r#extern::ExternTypeEthTemplate,
    crate::signature::major_item::ty::inductive::InductiveTypeEthTemplate,
    crate::signature::major_item::ty::props_struct::PropsStructEthTemplate,
    crate::signature::major_item::ty::structure::StructureTypeEthTemplate,
    crate::signature::major_item::ty::tuple_struct::TupleStructEthTemplate,
    crate::signature::major_item::ty::union::UnionTypeEthTemplate,
    crate::signature::major_item::ty::unit_struct::UnitStructEthTemplate,
    crate::signature::major_item::ty::ty_eth_template,
    // ty variant
    crate::signature::ty_variant::enum_tuple_ty_variant::EnumTupleVariantEthTemplate,
    crate::signature::ty_variant::enum_props_ty_variant::EnumPropsVariantEthTemplate,
    crate::signature::ty_variant::enum_unit_ty_variant::EnumUnitTypeVariantEthTemplate,
    crate::signature::ty_variant::ty_variant_eth_template,
    // impl block
    // - type
    crate::signature::impl_block::ty_impl_block::TypeImplBlockEthTemplate,
    // - trait for type
    crate::signature::impl_block::trai_for_ty_impl_block::TraitForTypeImplBlockEthTemplate,
    crate::signature::impl_block::trai_for_ty_impl_block::EthTraitForTypeImplBlockSignatureBuilder,
    crate::signature::impl_block::trai_for_ty_impl_block::trai_for_ty_impl_block_eth_template,
    crate::signature::impl_block::ty_impl_block::ty_impl_block_eth_template,
    crate::signature::impl_block::trai_for_ty_impl_block::trai_for_ty_impl_block_with_ty_instantiated_assoc_output_ethereal_signature_builder,
    crate::signature::impl_block::trai_for_ty_impl_block::trai_for_ty_impl_block_with_ty_instantiated_item_eth_template,
    // attr
    crate::signature::attr::derive::ty_path_derive_attr_eth_templates_map,
    // attr_eth_template,
    crate::signature::attr::derive::DeriveAttrEthTemplate,
    crate::signature::attr::derive::DeriveAttrShardEthTemplate,
    // helpers
    crate::helpers::trai_for_ty::ty_side_impl_block_signature_templates_map,
    crate::helpers::trai_for_ty::trai_side_path_leading_trai_for_ty_impl_block_eth_templates_map,
    crate::helpers::trai_for_ty::trai_side_derive_any_eth_templates,
);
