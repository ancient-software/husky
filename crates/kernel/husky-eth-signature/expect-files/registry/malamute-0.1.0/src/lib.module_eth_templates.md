```rust
[
    (
        ItemPath(`malamute::Class`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Enum(
                        EnumEthTemplate {
                            path: TypePath(`malamute::Class`, `Enum`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        variable: EthSymbolicVariable(`Label`, `mono`),
                                        traits: [],
                                    },
                                ],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`malamute::OneVsAll`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Enum(
                        EnumEthTemplate {
                            path: TypePath(`malamute::OneVsAll`, `Enum`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        variable: EthSymbolicVariable(`Label`, `phan`),
                                        traits: [],
                                    },
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        variable: EthSymbolicVariable(`label`, `phan`),
                                        traits: [],
                                    },
                                ],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`malamute::OneVsAllResult`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Enum(
                        EnumEthTemplate {
                            path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        variable: EthSymbolicVariable(`Label`, `phan`),
                                        traits: [],
                                    },
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        variable: EthSymbolicVariable(`label`, `phan`),
                                        traits: [],
                                    },
                                ],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`malamute::narrow_down`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`malamute::narrow_down`, `Ritchie(
                                Gn,
                            )`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        variable: EthSymbolicVariable(`Label`, `mono`),
                                        traits: [],
                                    },
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        variable: EthSymbolicVariable(`label`, `poly`),
                                        traits: [],
                                    },
                                ],
                            },
                            ritchie_ty: EthRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                parameter_contracted_tys: [
                                    EtherealRitchieParameter::Variadic(
                                        EtherealRitchieVariadicParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Keyed(
                                        EtherealRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    "skip",
                                                ),
                                            ),
                                            contract: Pure,
                                            ty: ItemPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 19,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            has_default: true,
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`OneVsAllResult Label label`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`malamute::OneVsAll as core::default::Default(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::default::Default(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    variable: EthSymbolicVariable(`Label`, `phan`),
                                    traits: [],
                                },
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    variable: EthSymbolicVariable(`label`, `phan`),
                                    traits: [],
                                },
                            ],
                        },
                        trai: EthTerm(`Default`),
                        self_ty_refined: PathLeading(
                            Application(
                                EthApplication(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<malamute::OneVsAll as core::default::Default(0)>::default`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    TraitForTypeItemEthTemplate::AssocRitchie(
                        TraitForTypeAssocRitchieEthTemplate {
                            path: TraitForTypeItemPath(
                                `<malamute::OneVsAll as core::default::Default(0)>::default`,
                                TraitItemKind::AssocRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`OneVsAll Label label`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`malamute::Class as core::ops::Unveil(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`malamute::Class as core::ops::Unveil(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    variable: EthSymbolicVariable(`Label`, `mono`),
                                    traits: [],
                                },
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    variable: EthSymbolicVariable(`label`, `poly`),
                                    traits: [],
                                },
                            ],
                        },
                        trai: EthTerm(`Unveil OneVsAll Label label`),
                        self_ty_refined: PathLeading(
                            Application(
                                EthApplication(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<malamute::Class as core::ops::Unveil(0)>::Output`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    TraitForTypeItemEthTemplate::AssocType(
                        TraitForTypeAssocTypeEthTemplate {
                            path: TraitForTypeItemPath(
                                `<malamute::Class as core::ops::Unveil(0)>::Output`,
                                TraitItemKind::AssocType,
                            ),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            assoc_ty: EthTerm(`unit`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    TraitForTypeItemEthTemplate::AssocRitchie(
                        TraitForTypeAssocRitchieEthTemplate {
                            path: TraitForTypeItemPath(
                                `<malamute::Class as core::ops::Unveil(0)>::unveil`,
                                TraitItemKind::AssocRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`OneVsAll Label label`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: EthTerm(`ControlFlow Class Label unit`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    variable: EthSymbolicVariable(`Label`, `phan`),
                                    traits: [],
                                },
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    variable: EthSymbolicVariable(`LABEL`, `phan`),
                                    traits: [],
                                },
                            ],
                        },
                        trai: EthTerm(`Unveil OneVsAllResult Label LABEL`),
                        self_ty_refined: PathLeading(
                            Application(
                                EthApplication(
                                    Id {
                                        value: 4,
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::Output`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    TraitForTypeItemEthTemplate::AssocType(
                        TraitForTypeAssocTypeEthTemplate {
                            path: TraitForTypeItemPath(
                                `<malamute::OneVsAll as core::ops::Unveil(0)>::Output`,
                                TraitItemKind::AssocType,
                            ),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            assoc_ty: EthTerm(`unit`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    TraitForTypeItemEthTemplate::AssocRitchie(
                        TraitForTypeAssocRitchieEthTemplate {
                            path: TraitForTypeItemPath(
                                `<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`,
                                TraitItemKind::AssocRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`OneVsAllResult Label LABEL`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: EthTerm(`ControlFlow OneVsAll Label LABEL unit`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```