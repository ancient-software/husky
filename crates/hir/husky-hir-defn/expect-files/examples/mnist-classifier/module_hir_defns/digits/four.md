[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::four::left_components`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::four::left_components`, `Val`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::four::left_components`, `Val`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                6,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 45,
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
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::four::left_coordinate_max`, `FunctionFn`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::four::left_coordinate_max`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    ),
                                ],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::four::left_coordinate_max`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `cc`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRvarEntry {
                                            name: HirEagerRvarName::Ident(
                                                `cc`,
                                            ),
                                            data: HirEagerRvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            4,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::four::left_coordinate_max`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MemoizedField {
                                                owner_hir_expr_idx: 1,
                                                ident: `relative_bounding_box`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`, `MemoizedField`),
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 2,
                                                self_contract: Pure,
                                                ident: `xmax`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                            coersion: Some(
                                                WrapInSome,
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRvarEntry {
                                                name: HirEagerRvarName::Ident(
                                                    `cc`,
                                                ),
                                                data: HirEagerRvarData::ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                5,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 49,
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
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                5,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 51,
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
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::four::is_four`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::four::is_four`, `Val`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::four::is_four`, `Val`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                86,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 53,
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
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::four::displacement_downwards`, `FunctionFn`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::four::displacement_downwards`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    ),
                                ],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::four::displacement_downwards`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `cc`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRvarEntry {
                                            name: HirEagerRvarName::Ident(
                                                `cc`,
                                            ),
                                            data: HirEagerRvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            9,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::four::displacement_downwards`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            0.0,
                                                        ),
                                                        text: "0.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 4,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                ropd: 5,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 7,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..4,
                                                ),
                                            },
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 2,
                                            coersion: None,
                                        },
                                        Require {
                                            condition: Other {
                                                hir_eager_expr_idx: 6,
                                                conversion: None,
                                            },
                                        },
                                        Eval {
                                            expr_idx: 8,
                                            coersion: Some(
                                                WrapInSome,
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `dp`,
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRvarEntry {
                                                name: HirEagerRvarName::Ident(
                                                    `cc`,
                                                ),
                                                data: HirEagerRvarData::ParenateParameter,
                                            },
                                            HirEagerRvarEntry {
                                                name: HirEagerRvarName::Ident(
                                                    `dp`,
                                                ),
                                                data: HirEagerRvarData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::four::cc_box_heights`, `FunctionFn`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::four::cc_box_heights`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::option::Option`, `Enum`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                                template_arguments: [],
                                                always_copyable: true,
                                            },
                                        ),
                                    ),
                                ],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::four::cc_box_heights`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_arena: Arena {
                                data: [
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `cc`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRvarEntry {
                                            name: HirEagerRvarName::Ident(
                                                `cc`,
                                            ),
                                            data: HirEagerRvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            15,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::four::cc_box_heights`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            0.0,
                                                        ),
                                                        text: "0.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 4,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 5,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MemoizedField {
                                                owner_hir_expr_idx: 7,
                                                ident: `relative_bounding_box`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`, `MemoizedField`),
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 8,
                                                self_contract: Pure,
                                                ident: `ymin`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            0.4,
                                                        ),
                                                        text: "0.4f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 9,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 10,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MemoizedField {
                                                owner_hir_expr_idx: 12,
                                                ident: `relative_bounding_box`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`, `MemoizedField`),
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 13,
                                                self_contract: Pure,
                                                ident: `ymin`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..5,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 2,
                                            coersion: None,
                                        },
                                        Require {
                                            condition: Other {
                                                hir_eager_expr_idx: 6,
                                                conversion: None,
                                            },
                                        },
                                        Require {
                                            condition: Other {
                                                hir_eager_expr_idx: 11,
                                                conversion: None,
                                            },
                                        },
                                        Eval {
                                            expr_idx: 14,
                                            coersion: Some(
                                                WrapInSome,
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `dp`,
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRvarEntry {
                                                name: HirEagerRvarName::Ident(
                                                    `cc`,
                                                ),
                                                data: HirEagerRvarData::ParenateParameter,
                                            },
                                            HirEagerRvarEntry {
                                                name: HirEagerRvarName::Ident(
                                                    `dp`,
                                                ),
                                                data: HirEagerRvarData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]