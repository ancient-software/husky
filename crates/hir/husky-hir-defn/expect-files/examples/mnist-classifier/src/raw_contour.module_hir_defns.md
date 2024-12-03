```rust
[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    hir_decl: PropsStructHirDecl {
                        path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        fields: [
                            PropsStructFieldHirDecl {
                                ident: `cc`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            HirTemplateArgument::Type(
                                                HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            ),
                                        ],
                                        always_copyable: true,
                                    },
                                ),
                                initialization: None,
                            },
                            PropsStructFieldHirDecl {
                                ident: `points`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                        template_arguments: [
                                            HirTemplateArgument::Type(
                                                HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            ),
                                        ],
                                        always_copyable: false,
                                    },
                                ),
                                initialization: None,
                            },
                        ],
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::raw_contour::RawContour`),
                            ),
                            self_value_ty: None,
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_arena: Arena {
                                data: [],
                            },
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `cc`,
                                            ),
                                            data: HirEagerRuntimeVariableData::FieldVariable,
                                        },
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `points`,
                                            ),
                                            data: HirEagerRuntimeVariableData::FieldVariable,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::Enum(
                EnumHirDefn {
                    path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                    hir_decl: EnumHirDecl {
                        path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::raw_contour::Direction`),
                            ),
                            self_value_ty: None,
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_arena: Arena {
                                data: [],
                            },
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::TypeVariant(
        TypeVariantHirDefn::Unit(
            EnumUnitVariantHirDefn {
                path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Up`),
                hir_decl: EnumUnitTypeVariantHirDecl {
                    path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Up`),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::raw_contour::Direction::Up`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            },
        ),
    ),
    HirDefn::TypeVariant(
        TypeVariantHirDefn::Unit(
            EnumUnitVariantHirDefn {
                path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Left`),
                hir_decl: EnumUnitTypeVariantHirDecl {
                    path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Left`),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::raw_contour::Direction::Left`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            },
        ),
    ),
    HirDefn::TypeVariant(
        TypeVariantHirDefn::Unit(
            EnumUnitVariantHirDefn {
                path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Down`),
                hir_decl: EnumUnitTypeVariantHirDecl {
                    path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Down`),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::raw_contour::Direction::Down`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            },
        ),
    ),
    HirDefn::TypeVariant(
        TypeVariantHirDefn::Unit(
            EnumUnitVariantHirDefn {
                path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Right`),
                hir_decl: EnumUnitTypeVariantHirDecl {
                    path: TypeVariantPath(`mnist_classifier::raw_contour::Direction::Right`),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::raw_contour::Direction::Right`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            },
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Form(
            MajorFormHirDefn::Ritchie(
                MajorFunctionRitchieHirDefn {
                    path: MajorFormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
                        path: MajorFormPath(`mnist_classifier::raw_contour::get_pixel_pair`, `Ritchie(
                            Fn,
                        )`),
                        ritchie_item_kind: RitchieItemKind::Fn,
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirParenateParameters::Eager(
                            HirEagerParenateParameters(
                                [
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 0,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 1,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                ],
                            ),
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 287,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                7,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 288,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Form(
            MajorFormHirDefn::Ritchie(
                MajorFunctionRitchieHirDefn {
                    path: MajorFormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
                        path: MajorFormPath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `Ritchie(
                            Fn,
                        )`),
                        ritchie_item_kind: RitchieItemKind::Fn,
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirParenateParameters::Eager(
                            HirEagerParenateParameters(
                                [
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 0,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 1,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                ],
                            ),
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 289,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                5,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 290,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Form(
            MajorFormHirDefn::Ritchie(
                MajorFunctionRitchieHirDefn {
                    path: MajorFormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
                        path: MajorFormPath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `Ritchie(
                            Fn,
                        )`),
                        ritchie_item_kind: RitchieItemKind::Fn,
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirParenateParameters::Eager(
                            HirEagerParenateParameters(
                                [
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 0,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 1,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                ],
                            ),
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 291,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                7,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 292,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Form(
            MajorFormHirDefn::Ritchie(
                MajorFunctionRitchieHirDefn {
                    path: MajorFormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
                        path: MajorFormPath(`mnist_classifier::raw_contour::get_inward_direction`, `Ritchie(
                            Fn,
                        )`),
                        ritchie_item_kind: RitchieItemKind::Fn,
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirParenateParameters::Eager(
                            HirEagerParenateParameters(
                                [
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 0,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 1,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 2,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                ],
                            ),
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 293,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                22,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 294,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Form(
            MajorFormHirDefn::Ritchie(
                MajorFunctionRitchieHirDefn {
                    path: MajorFormPath(`mnist_classifier::raw_contour::get_angle_change`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
                        path: MajorFormPath(`mnist_classifier::raw_contour::get_angle_change`, `Ritchie(
                            Fn,
                        )`),
                        ritchie_item_kind: RitchieItemKind::Fn,
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirParenateParameters::Eager(
                            HirEagerParenateParameters(
                                [
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 0,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 1,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                ],
                            ),
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 295,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                14,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 296,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Form(
            MajorFormHirDefn::Ritchie(
                MajorFunctionRitchieHirDefn {
                    path: MajorFormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
                        path: MajorFormPath(`mnist_classifier::raw_contour::get_outward_direction`, `Ritchie(
                            Fn,
                        )`),
                        ritchie_item_kind: RitchieItemKind::Fn,
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirParenateParameters::Eager(
                            HirEagerParenateParameters(
                                [
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 0,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 1,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 2,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 3,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                ],
                            ),
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 297,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                32,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 298,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                    hir_decl: PropsStructHirDecl {
                        path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        fields: [
                            PropsStructFieldHirDecl {
                                ident: `prev1`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                        template_arguments: [],
                                        always_copyable: true,
                                    },
                                ),
                                initialization: None,
                            },
                            PropsStructFieldHirDecl {
                                ident: `prev2`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                        template_arguments: [],
                                        always_copyable: true,
                                    },
                                ),
                                initialization: None,
                            },
                        ],
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::raw_contour::StreakCache`),
                            ),
                            self_value_ty: None,
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_arena: Arena {
                                data: [],
                            },
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `prev1`,
                                            ),
                                            data: HirEagerRuntimeVariableData::FieldVariable,
                                        },
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `prev2`,
                                            ),
                                            data: HirEagerRuntimeVariableData::FieldVariable,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Form(
            MajorFormHirDefn::Ritchie(
                MajorFunctionRitchieHirDefn {
                    path: MajorFormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
                        path: MajorFormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Ritchie(
                            Fn,
                        )`),
                        ritchie_item_kind: RitchieItemKind::Fn,
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirParenateParameters::Eager(
                            HirEagerParenateParameters(
                                [
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 0,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                template_arguments: [
                                                    HirTemplateArgument::Type(
                                                        HirType::PathLeading(
                                                            HirTypePathLeading {
                                                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                template_arguments: [],
                                                                always_copyable: false,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                                always_copyable: false,
                                            },
                                        ),
                                    },
                                ],
                            ),
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 300,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                27,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 301,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Form(
            MajorFormHirDefn::Ritchie(
                MajorFunctionRitchieHirDefn {
                    path: MajorFormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
                        path: MajorFormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                            Fn,
                        )`),
                        ritchie_item_kind: RitchieItemKind::Fn,
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirParenateParameters::Eager(
                            HirEagerParenateParameters(
                                [
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 0,
                                        contract: Pure,
                                        ty: HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                template_arguments: [
                                                    HirTemplateArgument::Type(
                                                        HirType::PathLeading(
                                                            HirTypePathLeading {
                                                                ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                template_arguments: [],
                                                                always_copyable: false,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                                always_copyable: true,
                                            },
                                        ),
                                    },
                                ],
                            ),
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                template_arguments: [],
                                                always_copyable: false,
                                            },
                                        ),
                                    ),
                                ],
                                always_copyable: false,
                            },
                        ),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 302,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                297,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 303,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
                    path: TraitForTypeImplBlockPath(`mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    trai: HirTrait {
                        trai_path: TraitPath(`core::visual::Visualize`),
                        template_arguments: [],
                    },
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            },
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::MethodFn(
                TraitForTypeMethodRitchieHirDefn {
                    path: TraitForTypeItemPath(
                        `<mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)>::visualize`,
                        TraitItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TraitForTypeMethodRitchieHirDecl {
                        path: TraitForTypeItemPath(
                            `<mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)>::visualize`,
                            TraitItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter {
                            contract: Pure,
                            self_ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 6,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::visual::Visual`, `Extern`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`<mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)>::visualize`),
                            ),
                            self_value_ty: Some(
                                HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_arena: Arena {
                                data: [],
                            },
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::SelfValue,
                                            data: HirEagerRuntimeVariableData::SelfValue,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    0,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            3,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`<mnist_classifier::raw_contour::RawContour as core::visual::Visualize(0)>::visualize`),
                                ),
                                self_value_ty: Some(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            template_arguments: [],
                                            always_copyable: false,
                                        },
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `points`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::EmptyHtmlTag {
                                                function_ident: `Contour`,
                                                arguments: [
                                                    HirEagerHtmlArgumentExpr {
                                                        property_ident: Ident(
                                                            Coword(
                                                                "points",
                                                            ),
                                                        ),
                                                        expr: 1,
                                                    },
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::visual::Visual`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::visual::Visual`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 2,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_arena: Arena {
                                    data: [],
                                },
                                comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::SelfValue,
                                                data: HirEagerRuntimeVariableData::SelfValue,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        0,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::Type(
            TypeImplBlockHirDefn {
                hir_decl: TypeImplBlockHirDecl {
                    path: TypeImplBlockPath(`mnist_classifier::raw_contour::RawContour(0)`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::raw_contour::RawContour(0)`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            },
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`,
                            TypeItemKind::MemoizedField,
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`),
                            ),
                            self_value_ty: Some(
                                HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            HirTemplateArgument::Type(
                                                HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            ),
                                        ],
                                        always_copyable: true,
                                    },
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_arena: Arena {
                                data: [],
                            },
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::SelfValue,
                                            data: HirEagerRuntimeVariableData::SelfValue,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    0,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            3,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`),
                                ),
                                self_value_ty: Some(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            template_arguments: [],
                                                            always_copyable: false,
                                                        },
                                                    ),
                                                ),
                                            ],
                                            always_copyable: true,
                                        },
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: StackPure {
                                                            place: Idx(
                                                                PlaceIdx(0),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::F32(
                                                    F32Literal {
                                                        value: 1.4,
                                                        text: "1.4f32",
                                                    },
                                                ),
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Compterm,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::AssocFunctionRitchieCall {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`,
                                                        TypeItemKind::AssocRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 7,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        0,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(0),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 10,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        1,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Compterm,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 2,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_arena: Arena {
                                    data: [],
                                },
                                comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::SelfValue,
                                                data: HirEagerRuntimeVariableData::SelfValue,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        0,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::raw_contour::RawContour(0)::bounding_box`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::raw_contour::RawContour(0)::bounding_box`,
                            TypeItemKind::MemoizedField,
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::raw_contour::RawContour(0)::bounding_box`),
                            ),
                            self_value_ty: Some(
                                HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            HirTemplateArgument::Type(
                                                HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            ),
                                        ],
                                        always_copyable: true,
                                    },
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_arena: Arena {
                                data: [],
                            },
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::SelfValue,
                                            data: HirEagerRuntimeVariableData::SelfValue,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    0,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            50,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::raw_contour::RawContour(0)::bounding_box`),
                                ),
                                self_value_ty: Some(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            template_arguments: [],
                                                            always_copyable: false,
                                                        },
                                                    ),
                                                ),
                                            ],
                                            always_copyable: true,
                                        },
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `points`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::USize(
                                                    USizeLiteral {
                                                        value: 0,
                                                    },
                                                ),
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::usize`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                self_argument: 1,
                                                items: [
                                                    2,
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                1,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 4,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                1,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 6,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                1,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 8,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                1,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 10,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 12,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `points`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 13,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_contract: Pure,
                                                ident: `ilen`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::vec::Vec(0)::ilen`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::vec::Vec(0)::ilen`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                            template_arguments: [],
                                                                            always_copyable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 15,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `points`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                6,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(6),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(6),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(6),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                self_argument: 16,
                                                items: [
                                                    17,
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                2,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(2),
                                                        ),
                                                        BorrowMut,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                2,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(2),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                7,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 21,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Leashed {
                                                            place_idx: Some(
                                                                PlaceIdx(7),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 20,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_contract: Pure,
                                                ident: `min`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::num::f32(0)::min`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(2),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(2),
                                                        ),
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::num::f32(0)::min`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 10,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        22,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Leashed {
                                                                    place_idx: Some(
                                                                        PlaceIdx(7),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 19,
                                                opr: Assign,
                                                ropd: 23,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                3,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(3),
                                                        ),
                                                        BorrowMut,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                3,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(3),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                7,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 27,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Leashed {
                                                            place_idx: Some(
                                                                PlaceIdx(7),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 26,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_contract: Pure,
                                                ident: `max`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::num::f32(0)::max`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(3),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(3),
                                                        ),
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::num::f32(0)::max`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 10,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        28,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Leashed {
                                                                    place_idx: Some(
                                                                        PlaceIdx(7),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 25,
                                                opr: Assign,
                                                ropd: 29,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                4,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(4),
                                                        ),
                                                        BorrowMut,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                4,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(4),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                7,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 33,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Leashed {
                                                            place_idx: Some(
                                                                PlaceIdx(7),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 32,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_contract: Pure,
                                                ident: `min`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::num::f32(0)::min`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(4),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(4),
                                                        ),
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::num::f32(0)::min`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 10,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        34,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Leashed {
                                                                    place_idx: Some(
                                                                        PlaceIdx(7),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 31,
                                                opr: Assign,
                                                ropd: 35,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                5,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(5),
                                                        ),
                                                        BorrowMut,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                5,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(5),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                7,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 39,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Leashed {
                                                            place_idx: Some(
                                                                PlaceIdx(7),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 38,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_contract: Pure,
                                                ident: `max`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::num::f32(0)::max`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(5),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(5),
                                                        ),
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::num::f32(0)::max`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 10,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        40,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Leashed {
                                                                    place_idx: Some(
                                                                        PlaceIdx(7),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 37,
                                                opr: Assign,
                                                ropd: 41,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                2,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(2),
                                                        ),
                                                        Move,
                                                    ),
                                                ],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: MutableOnStack {
                                                            place: Idx(
                                                                PlaceIdx(2),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                3,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(3),
                                                        ),
                                                        Move,
                                                    ),
                                                ],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: MutableOnStack {
                                                            place: Idx(
                                                                PlaceIdx(3),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorCall {
                                                path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::geom2d::ClosedRange`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: None,
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 10,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        43,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: MutableOnStack {
                                                                    place: Idx(
                                                                        PlaceIdx(2),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 10,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        44,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: MutableOnStack {
                                                                    place: Idx(
                                                                        PlaceIdx(3),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                4,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(4),
                                                        ),
                                                        Move,
                                                    ),
                                                ],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: MutableOnStack {
                                                            place: Idx(
                                                                PlaceIdx(4),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                5,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(5),
                                                        ),
                                                        Move,
                                                    ),
                                                ],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: MutableOnStack {
                                                            place: Idx(
                                                                PlaceIdx(5),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorCall {
                                                path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::geom2d::ClosedRange`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: None,
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 10,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        46,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: MutableOnStack {
                                                                    place: Idx(
                                                                        PlaceIdx(4),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 10,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        47,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: MutableOnStack {
                                                                    place: Idx(
                                                                        PlaceIdx(5),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorCall {
                                                path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::geom2d::BoundingBox`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: None,
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 54,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        45,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 54,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        48,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    5..12,
                                                ),
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::basic::never`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Never,
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 5,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 18,
                                            coercion: None,
                                        },
                                        Eval {
                                            expr: 24,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr: 30,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr: 36,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr: 42,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 0,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 3,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 1,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 5,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 2,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 7,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 3,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 9,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 4,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 11,
                                            coercion: None,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                for_loop_variable_ident: Ident(
                                                    Coword(
                                                        "i",
                                                    ),
                                                ),
                                                for_loop_variable_ty_path: I32,
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: None,
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            14,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            for_loop_varible_idx: 6,
                                            stmts: ArenaIdxRange(
                                                0..5,
                                            ),
                                        },
                                        Return {
                                            result: 49,
                                            coercion: Trivial(
                                                TrivialHirEagerCoercion {
                                                    expectee_quary: Transient,
                                                },
                                            ),
                                        },
                                    ],
                                },
                                pattern_arena: Arena {
                                    data: [
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `start_point`,
                                                variable_idx: 1,
                                            },
                                            contract: Pure,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `xmin`,
                                                variable_idx: 2,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `xmax`,
                                                variable_idx: 3,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `ymin`,
                                                variable_idx: 4,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `ymax`,
                                                variable_idx: 5,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `point`,
                                                variable_idx: 7,
                                            },
                                            contract: Pure,
                                        },
                                    ],
                                },
                                comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::SelfValue,
                                                data: HirEagerRuntimeVariableData::SelfValue,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `start_point`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `xmin`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `xmax`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `ymin`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `ymax`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LoopVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `point`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        0,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`,
                            TypeItemKind::MemoizedField,
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`),
                            ),
                            self_value_ty: Some(
                                HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            HirTemplateArgument::Type(
                                                HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            ),
                                        ],
                                        always_copyable: true,
                                    },
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_arena: Arena {
                                data: [],
                            },
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::SelfValue,
                                            data: HirEagerRuntimeVariableData::SelfValue,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    0,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            9,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`),
                                ),
                                self_value_ty: Some(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            template_arguments: [],
                                                            always_copyable: false,
                                                        },
                                                    ),
                                                ),
                                            ],
                                            always_copyable: true,
                                        },
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Leash,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Leash,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Leash,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `cc`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MemoizedField {
                                                self_argument: 1,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `raw_contours`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`,
                                                        TypeItemKind::MemoizedField,
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                                    template_arguments: [
                                                                        HirTemplateArgument::Type(
                                                                            HirType::PathLeading(
                                                                                HirTypePathLeading {
                                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                                    template_arguments: [],
                                                                                    always_copyable: false,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::USize(
                                                    USizeLiteral {
                                                        value: 0,
                                                    },
                                                ),
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::usize`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                self_argument: 2,
                                                items: [
                                                    3,
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MemoizedField {
                                                self_argument: 4,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `bounding_box`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::raw_contour::RawContour(0)::bounding_box`,
                                                        TypeItemKind::MemoizedField,
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::raw_contour::RawContour(0)::bounding_box`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Leash,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Leash,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Leash,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MemoizedField {
                                                self_argument: 6,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `bounding_box`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::raw_contour::RawContour(0)::bounding_box`,
                                                        TypeItemKind::MemoizedField,
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::raw_contour::RawContour(0)::bounding_box`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Dedirection(
                                                    Deleash,
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 5,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_contract: Pure,
                                                ident: `relative_bounding_box`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Transient,
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 50,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        7,
                                                        Dedirection(
                                                            Deleash,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 8,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_arena: Arena {
                                    data: [],
                                },
                                comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::SelfValue,
                                                data: HirEagerRuntimeVariableData::SelfValue,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        0,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::raw_contour::RawContour(0)::contour_len`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::raw_contour::RawContour(0)::contour_len`,
                            TypeItemKind::MemoizedField,
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::raw_contour::RawContour(0)::contour_len`),
                            ),
                            self_value_ty: Some(
                                HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            HirTemplateArgument::Type(
                                                HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            ),
                                        ],
                                        always_copyable: true,
                                    },
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_arena: Arena {
                                data: [],
                            },
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::SelfValue,
                                            data: HirEagerRuntimeVariableData::SelfValue,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    0,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            58,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::raw_contour::RawContour(0)::contour_len`),
                                ),
                                self_value_ty: Some(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                            template_arguments: [],
                                                            always_copyable: false,
                                                        },
                                                    ),
                                                ),
                                            ],
                                            always_copyable: true,
                                        },
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::F32(
                                                    F32Literal {
                                                        value: 0.0,
                                                        text: "0.0f32",
                                                    },
                                                ),
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
                                                ),
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 2,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `points`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 3,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_contract: Pure,
                                                ident: `ilen`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::vec::Vec(0)::ilen`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::vec::Vec(0)::ilen`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                            template_arguments: [],
                                                                            always_copyable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 5,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `points`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                2,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(2),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Compterm,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 7,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 8,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                self_argument: 6,
                                                items: [
                                                    9,
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 11,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `points`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                2,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(2),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                self_argument: 12,
                                                items: [
                                                    13,
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                1,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(1),
                                                        ),
                                                        BorrowMut,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                3,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 16,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(3),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(3),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                4,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 18,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(4),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(4),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Leashed {
                                                            place_idx: Some(
                                                                PlaceIdx(4),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 17,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 19,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 20,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_contract: Pure,
                                                ident: `abs`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::num::f32(0)::abs`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Transient,
                                                    indirections: [],
                                                    final_place: Transient,
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::num::f32(0)::abs`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                3,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 22,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(3),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(3),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                4,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 24,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(4),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(4),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Leashed {
                                                            place_idx: Some(
                                                                PlaceIdx(4),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 23,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 25,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 26,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_contract: Pure,
                                                ident: `abs`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::num::f32(0)::abs`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Transient,
                                                    indirections: [],
                                                    final_place: Transient,
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::num::f32(0)::abs`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 21,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 27,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 15,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                ropd: 28,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 30,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `points`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 32,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `points`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 33,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_contract: Pure,
                                                ident: `ilen`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::vec::Vec(0)::ilen`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::vec::Vec(0)::ilen`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                            template_arguments: [],
                                                                            always_copyable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Compterm,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 34,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 35,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                self_argument: 31,
                                                items: [
                                                    36,
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 38,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `points`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::USize(
                                                    USizeLiteral {
                                                        value: 0,
                                                    },
                                                ),
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::usize`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                self_argument: 39,
                                                items: [
                                                    40,
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                1,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(1),
                                                        ),
                                                        BorrowMut,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                5,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 43,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(5),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(5),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                6,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(6),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(6),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 45,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(6),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(6),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(6),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Leashed {
                                                            place_idx: Some(
                                                                PlaceIdx(6),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(6),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 44,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 46,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 47,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_contract: Pure,
                                                ident: `abs`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::num::f32(0)::abs`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Transient,
                                                    indirections: [],
                                                    final_place: Transient,
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::num::f32(0)::abs`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                5,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 49,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(5),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(5),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                6,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(6),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(6),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 51,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(6),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(6),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(6),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Leashed {
                                                            place_idx: Some(
                                                                PlaceIdx(6),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(6),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 50,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 52,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 53,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                                self_contract: Pure,
                                                ident: `abs`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::num::f32(0)::abs`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Transient,
                                                    indirections: [],
                                                    final_place: Transient,
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::num::f32(0)::abs`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 48,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 54,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 42,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                ropd: 55,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                1,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(1),
                                                        ),
                                                        Move,
                                                    ),
                                                ],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: MutableOnStack {
                                                            place: Idx(
                                                                PlaceIdx(1),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    3..9,
                                                ),
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::basic::never`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Never,
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 1,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 10,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 2,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 14,
                                            coercion: None,
                                        },
                                        Eval {
                                            expr: 29,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 0,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 0,
                                            coercion: None,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                for_loop_variable_ident: Ident(
                                                    Coword(
                                                        "i",
                                                    ),
                                                ),
                                                for_loop_variable_ty_path: I32,
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            1,
                                                        ),
                                                        kind: LowerOpen,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            4,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            for_loop_varible_idx: 2,
                                            stmts: ArenaIdxRange(
                                                0..3,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 3,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 37,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 4,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 41,
                                            coercion: None,
                                        },
                                        Eval {
                                            expr: 56,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Return {
                                            result: 57,
                                            coercion: Trivial(
                                                TrivialHirEagerCoercion {
                                                    expectee_quary: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                },
                                            ),
                                        },
                                    ],
                                },
                                pattern_arena: Arena {
                                    data: [
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `contour_len`,
                                                variable_idx: 1,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `a`,
                                                variable_idx: 3,
                                            },
                                            contract: Pure,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `b`,
                                                variable_idx: 4,
                                            },
                                            contract: Pure,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `a`,
                                                variable_idx: 5,
                                            },
                                            contract: Pure,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `b`,
                                                variable_idx: 6,
                                            },
                                            contract: Pure,
                                        },
                                    ],
                                },
                                comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::SelfValue,
                                                data: HirEagerRuntimeVariableData::SelfValue,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `contour_len`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LoopVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `a`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `b`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `a`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `b`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        0,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodRitchieHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::raw_contour::RawContour(0)::displacement`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TypeMethodRitchieHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::raw_contour::RawContour(0)::displacement`,
                            TypeItemKind::MethodRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter {
                            contract: Pure,
                            self_ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 6,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
                                    pattern_idx: 0,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Simple {
                                    pattern_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::raw_contour::RawContour(0)::displacement`),
                            ),
                            self_value_ty: Some(
                                HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_arena: Arena {
                                data: [
                                    HirEagerPatternEntry {
                                        data: HirEagerPatternData::Ident {
                                            symbol_modifier: None,
                                            ident: `start`,
                                            variable_idx: 1,
                                        },
                                        contract: Pure,
                                    },
                                    HirEagerPatternEntry {
                                        data: HirEagerPatternData::Ident {
                                            symbol_modifier: None,
                                            ident: `end`,
                                            variable_idx: 2,
                                        },
                                        contract: Pure,
                                    },
                                ],
                            },
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::SelfValue,
                                            data: HirEagerRuntimeVariableData::SelfValue,
                                        },
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `start`,
                                            ),
                                            data: HirEagerRuntimeVariableData::ParenateParameter,
                                        },
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `end`,
                                            ),
                                            data: HirEagerRuntimeVariableData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    0,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            18,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::raw_contour::RawContour(0)::displacement`),
                                ),
                                self_value_ty: Some(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            template_arguments: [],
                                            always_copyable: false,
                                        },
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `points`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 1,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_contract: Pure,
                                                ident: `ilen`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::vec::Vec(0)::ilen`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::vec::Vec(0)::ilen`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                            template_arguments: [],
                                                                            always_copyable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 3,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `points`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                1,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(1),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                3,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(1),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: ImmutableOnStack {
                                                            place: Idx(
                                                                PlaceIdx(1),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 5,
                                                opr: Closed(
                                                    RemEuclid,
                                                ),
                                                ropd: 6,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                self_argument: 4,
                                                items: [
                                                    7,
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 9,
                                                self_argument_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `points`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                },
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                2,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(2),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                3,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(1),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: ImmutableOnStack {
                                                            place: Idx(
                                                                PlaceIdx(1),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 11,
                                                opr: Closed(
                                                    RemEuclid,
                                                ),
                                                ropd: 12,
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                self_argument: 10,
                                                items: [
                                                    13,
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                4,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: Ref {
                                                    guard: Left(
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: None,
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: Ref {
                                                    guard: Left(
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                5,
                                            ),
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: Ref {
                                                    guard: Left(
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    ),
                                                },
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Ref {
                                                            guard: Left(
                                                                Idx(
                                                                    PlaceIdx(0),
                                                                ),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: Ref {
                                                    guard: Left(
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    ),
                                                },
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 15,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                self_contract: Pure,
                                                ident: `to`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::geom2d::Point2d(0)::to`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Ref {
                                                        guard: Left(
                                                            Idx(
                                                                PlaceIdx(0),
                                                            ),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Ref {
                                                        guard: Left(
                                                            Idx(
                                                                PlaceIdx(0),
                                                            ),
                                                        ),
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::geom2d::Point2d(0)::to`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    variable_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 24,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        16,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Ref {
                                                                    guard: Left(
                                                                        Idx(
                                                                            PlaceIdx(0),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..4,
                                                ),
                                            },
                                            base_ty: HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            contracted_quary_after_coercion: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 0,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 2,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 1,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 8,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 2,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 14,
                                            coercion: None,
                                        },
                                        Eval {
                                            expr: 17,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_arena: Arena {
                                    data: [
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `N`,
                                                variable_idx: 3,
                                            },
                                            contract: Pure,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `ct_start`,
                                                variable_idx: 4,
                                            },
                                            contract: Pure,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `ct_end`,
                                                variable_idx: 5,
                                            },
                                            contract: Pure,
                                        },
                                    ],
                                },
                                comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::SelfValue,
                                                data: HirEagerRuntimeVariableData::SelfValue,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `start`,
                                                ),
                                                data: HirEagerRuntimeVariableData::ParenateParameter,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `end`,
                                                ),
                                                data: HirEagerRuntimeVariableData::ParenateParameter,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `N`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `ct_start`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `ct_end`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        0,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]
```