[
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::clone::Clone`),
            ),
        ),
        None,
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlock {
                    data: TraitForTypeImplBlockPathData {
                        module_path: `core::clone`,
                        trai_path: TraitPath(`core::clone::Clone`),
                        ty_sketch: TypeSketch::DeriveAny,
                        disambiguator: 0,
                    },
                },
            ),
        ),
        None,
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssocItem(
                            AssocItemPathData::TraitForTypeItem(
                                TraitForTypeItemPathData {
                                    impl_block: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `core::clone`,
                                            trai_path: TraitPath(`core::clone::Clone`),
                                            ty_sketch: TypeSketch::DeriveAny,
                                            disambiguator: 0,
                                        },
                                    },
                                    ident: `clone`,
                                    item_kind: MethodFn,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        None,
    ),
]