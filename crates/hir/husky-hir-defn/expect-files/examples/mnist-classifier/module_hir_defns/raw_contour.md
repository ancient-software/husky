[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
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
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `cc`,
                                            ),
                                            data: HirEagerRuntimeSvarData::FieldVariable,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `points`,
                                            ),
                                            data: HirEagerRuntimeSvarData::FieldVariable,
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
                    hir_decl: EnumTypeHirDecl {
                        path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
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
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
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
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
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
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
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
                                        ident: `row`,
                                    },
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `j`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `row`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `j`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            8,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
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
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 2,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 3,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 1,
                                                opr: Shift(
                                                    Shr,
                                                ),
                                                ropd: 4,
                                            },
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
                                            data: HirEagerExprData::Literal(
                                                Literal::R32(
                                                    3,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 5,
                                                opr: Closed(
                                                    BitAnd,
                                                ),
                                                ropd: 6,
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
                                            expr_idx: 7,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
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
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `row`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `j`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
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
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
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
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
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
                                        ident: `row`,
                                    },
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `j`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `row`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `j`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            6,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_left`, `FunctionFn`),
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
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 1,
                                                opr: Shift(
                                                    Shr,
                                                ),
                                                ropd: 2,
                                            },
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
                                            data: HirEagerExprData::Literal(
                                                Literal::R32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 3,
                                                opr: Closed(
                                                    BitAnd,
                                                ),
                                                ropd: 4,
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
                                            expr_idx: 5,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
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
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `row`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `j`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
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
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
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
                                ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
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
                                        ident: `row`,
                                    },
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `j`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `row`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `j`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            8,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::get_pixel_to_the_right`, `FunctionFn`),
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
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 2,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 3,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 1,
                                                opr: Shift(
                                                    Shr,
                                                ),
                                                ropd: 4,
                                            },
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
                                            data: HirEagerExprData::Literal(
                                                Literal::R32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 5,
                                                opr: Closed(
                                                    BitAnd,
                                                ),
                                                ropd: 6,
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
                                            expr_idx: 7,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
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
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `row`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `j`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
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
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 3,
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
                                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
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
                                        ident: `row_above`,
                                    },
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `row_below`,
                                    },
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `j`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `row_above`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `row_below`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `j`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            23,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
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
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::FunctionFnCall {
                                                path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 12,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        1,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        2,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::FunctionFnCall {
                                                path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 12,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        4,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            2,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        5,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
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
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Left`,
                                                                    index: U8(
                                                                        1,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Up`,
                                                                    index: U8(
                                                                        0,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unreachable,
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Down`,
                                                                    index: U8(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Right`,
                                                                    index: U8(
                                                                        3,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Left`,
                                                                    index: U8(
                                                                        1,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Up`,
                                                                    index: U8(
                                                                        0,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unreachable,
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Right`,
                                                                    index: U8(
                                                                        3,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Up`,
                                                                    index: U8(
                                                                        0,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unreachable,
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unreachable,
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    16..19,
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
                                            expr_idx: 9,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 10,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 11,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 6,
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 7,
                                                    stmts: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 8,
                                                    stmts: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                },
                                            ],
                                            match_target: 8,
                                        },
                                        Eval {
                                            expr_idx: 12,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 14,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 15,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 16,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 17,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 11,
                                                    stmts: ArenaIdxRange(
                                                        6..7,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 14,
                                                    stmts: ArenaIdxRange(
                                                        7..8,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 15,
                                                    stmts: ArenaIdxRange(
                                                        8..9,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 16,
                                                    stmts: ArenaIdxRange(
                                                        9..10,
                                                    ),
                                                },
                                            ],
                                            match_target: 13,
                                        },
                                        Eval {
                                            expr_idx: 19,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 20,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 21,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 20,
                                                    stmts: ArenaIdxRange(
                                                        11..12,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 21,
                                                    stmts: ArenaIdxRange(
                                                        12..13,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 22,
                                                    stmts: ArenaIdxRange(
                                                        13..14,
                                                    ),
                                                },
                                            ],
                                            match_target: 18,
                                        },
                                        Eval {
                                            expr_idx: 22,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 3,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 6,
                                            coersion: None,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 3,
                                                    stmts: ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 9,
                                                    stmts: ArenaIdxRange(
                                                        5..6,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 10,
                                                    stmts: ArenaIdxRange(
                                                        10..11,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 17,
                                                    stmts: ArenaIdxRange(
                                                        14..15,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 23,
                                                    stmts: ArenaIdxRange(
                                                        15..16,
                                                    ),
                                                },
                                            ],
                                            match_target: 7,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `pixel_pair_above`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `pixel_pair_below`,
                                        },
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                0,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                1,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                3,
                                            ),
                                        ),
                                        HirEagerPatternExpr::OneOf {
                                            options: ArenaIdxRange(
                                                4..6,
                                            ),
                                        },
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                2,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `_`,
                                        },
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                1,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                2,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                0,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                1,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                3,
                                            ),
                                        ),
                                        HirEagerPatternExpr::OneOf {
                                            options: ArenaIdxRange(
                                                12..14,
                                            ),
                                        },
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                2,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `_`,
                                        },
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                3,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                0,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                1,
                                            ),
                                        ),
                                        HirEagerPatternExpr::OneOf {
                                            options: ArenaIdxRange(
                                                18..20,
                                            ),
                                        },
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                2,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `_`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `_`,
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `row_above`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `row_below`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `j`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `pixel_pair_above`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `pixel_pair_below`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSvarData::CaseVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSvarData::CaseVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSvarData::CaseVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSvarData::CaseVariable,
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
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
                                            ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
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
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
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
                                        ident: `inward`,
                                    },
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `outward`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `inward`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `outward`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
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
                                            FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::As {
                                                opd: 1,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
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
                                            data: HirEagerExprData::As {
                                                opd: 3,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 2,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 4,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::As {
                                                opd: 5,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    2,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 6,
                                                self_contract: Pure,
                                                ident: `last_bits`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::raw_bits::r32(0)::last_bits`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        7,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Const,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
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
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
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
                                            data: HirEagerExprData::As {
                                                opd: 10,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 12,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unreachable,
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    4..6,
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
                                            expr_idx: 11,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 13,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Const,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 14,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 8,
                                            coersion: None,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 5,
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 6,
                                                    stmts: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 7,
                                                    stmts: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                },
                                            ],
                                            match_target: 9,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `raw_angle_change`,
                                        },
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                0,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                1,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                2,
                                            ),
                                        ),
                                        HirEagerPatternExpr::OneOf {
                                            options: ArenaIdxRange(
                                                2..5,
                                            ),
                                        },
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                3,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `_`,
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `inward`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `outward`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `raw_angle_change`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSvarData::CaseVariable,
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
                    path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
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
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 3,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 4,
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
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
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
                                        ident: `row_above`,
                                    },
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `row_below`,
                                    },
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `j`,
                                    },
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `inward_direction`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `row_above`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `row_below`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `j`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `inward_direction`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            33,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
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
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::FunctionFnCall {
                                                path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 12,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        1,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        2,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::FunctionFnCall {
                                                path: FugitivePath(`mnist_classifier::raw_contour::get_pixel_pair`, `FunctionFn`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 12,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        4,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            2,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        5,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
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
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Down`,
                                                                    index: U8(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Left`,
                                                                    index: U8(
                                                                        1,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unreachable,
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Right`,
                                                                    index: U8(
                                                                        3,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Down`,
                                                                    index: U8(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Left`,
                                                                    index: U8(
                                                                        1,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Right`,
                                                                    index: U8(
                                                                        3,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unreachable,
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Left`,
                                                                    index: U8(
                                                                        1,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unreachable,
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Up`,
                                                                    index: U8(
                                                                        0,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Up`,
                                                                    index: U8(
                                                                        0,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Down`,
                                                                    index: U8(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unreachable,
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unreachable,
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Right`,
                                                                    index: U8(
                                                                        3,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath(
                                                        ItemPathId {
                                                            data: ItemPathData::TypeVariant(
                                                                TypeVariantPathData {
                                                                    parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                    ident: `Down`,
                                                                    index: U8(
                                                                        2,
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unreachable,
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unreachable,
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    26..29,
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
                                            expr_idx: 9,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 10,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 11,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 4,
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 7,
                                                    stmts: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 8,
                                                    stmts: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                },
                                            ],
                                            match_target: 8,
                                        },
                                        Eval {
                                            expr_idx: 13,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 14,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 16,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 17,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 18,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 13,
                                                    stmts: ArenaIdxRange(
                                                        7..8,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 14,
                                                    stmts: ArenaIdxRange(
                                                        8..9,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 15,
                                                    stmts: ArenaIdxRange(
                                                        9..10,
                                                    ),
                                                },
                                            ],
                                            match_target: 15,
                                        },
                                        Eval {
                                            expr_idx: 19,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 20,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 10,
                                                    stmts: ArenaIdxRange(
                                                        5..6,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 11,
                                                    stmts: ArenaIdxRange(
                                                        6..7,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 12,
                                                    stmts: ArenaIdxRange(
                                                        10..11,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 16,
                                                    stmts: ArenaIdxRange(
                                                        11..12,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 17,
                                                    stmts: ArenaIdxRange(
                                                        12..13,
                                                    ),
                                                },
                                            ],
                                            match_target: 12,
                                        },
                                        Eval {
                                            expr_idx: 22,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 24,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 25,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 26,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 24,
                                                    stmts: ArenaIdxRange(
                                                        15..16,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 25,
                                                    stmts: ArenaIdxRange(
                                                        16..17,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 26,
                                                    stmts: ArenaIdxRange(
                                                        17..18,
                                                    ),
                                                },
                                            ],
                                            match_target: 23,
                                        },
                                        Eval {
                                            expr_idx: 27,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 22,
                                                    stmts: ArenaIdxRange(
                                                        14..15,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 23,
                                                    stmts: ArenaIdxRange(
                                                        18..19,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 27,
                                                    stmts: ArenaIdxRange(
                                                        19..20,
                                                    ),
                                                },
                                            ],
                                            match_target: 21,
                                        },
                                        Eval {
                                            expr_idx: 29,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 30,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 31,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 31,
                                                    stmts: ArenaIdxRange(
                                                        21..22,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 32,
                                                    stmts: ArenaIdxRange(
                                                        22..23,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 33,
                                                    stmts: ArenaIdxRange(
                                                        23..24,
                                                    ),
                                                },
                                            ],
                                            match_target: 28,
                                        },
                                        Eval {
                                            expr_idx: 32,
                                            coersion: Some(
                                                Never,
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 3,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 6,
                                            coersion: None,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 3,
                                                    stmts: ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 9,
                                                    stmts: ArenaIdxRange(
                                                        13..14,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 18,
                                                    stmts: ArenaIdxRange(
                                                        20..21,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 28,
                                                    stmts: ArenaIdxRange(
                                                        24..25,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 34,
                                                    stmts: ArenaIdxRange(
                                                        25..26,
                                                    ),
                                                },
                                            ],
                                            match_target: 7,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `pixel_pair_above`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `pixel_pair_below`,
                                        },
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                0,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                1,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                2,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                3,
                                            ),
                                        ),
                                        HirEagerPatternExpr::OneOf {
                                            options: ArenaIdxRange(
                                                5..7,
                                            ),
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `_`,
                                        },
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                1,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                0,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                1,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                2,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Unit(
                                            PatternPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                ident: `Down`,
                                                                index: U8(
                                                                    2,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerPatternExpr::Unit(
                                            PatternPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                ident: `Up`,
                                                                index: U8(
                                                                    0,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `_`,
                                        },
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                3,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `_`,
                                        },
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                2,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                0,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                2,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                3,
                                            ),
                                        ),
                                        HirEagerPatternExpr::OneOf {
                                            options: ArenaIdxRange(
                                                19..22,
                                            ),
                                        },
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                1,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Unit(
                                            PatternPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                ident: `Left`,
                                                                index: U8(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerPatternExpr::Unit(
                                            PatternPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                ident: `Right`,
                                                                index: U8(
                                                                    3,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `_`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `_`,
                                        },
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                3,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                0,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                2,
                                            ),
                                        ),
                                        HirEagerPatternExpr::OneOf {
                                            options: ArenaIdxRange(
                                                29..31,
                                            ),
                                        },
                                        HirEagerPatternExpr::Literal(
                                            Literal::R32(
                                                1,
                                            ),
                                        ),
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `_`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `_`,
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `row_above`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `row_below`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `j`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `inward_direction`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `pixel_pair_above`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `pixel_pair_below`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSvarData::CaseVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSvarData::CaseVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSvarData::CaseVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSvarData::CaseVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSvarData::CaseVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSvarData::CaseVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `_`,
                                                ),
                                                data: HirEagerRuntimeSvarData::CaseVariable,
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
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
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
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),
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
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `prev1`,
                                            ),
                                            data: HirEagerRuntimeSvarData::FieldVariable,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `prev2`,
                                            ),
                                            data: HirEagerRuntimeSvarData::FieldVariable,
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
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
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
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
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
                                        ident: `points`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `points`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            28,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
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
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `ilen`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                item_groups: [],
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
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    2,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 4,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 5,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 3,
                                                items: [
                                                    6,
                                                ],
                                            },
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
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
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 9,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 10,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 8,
                                                items: [
                                                    11,
                                                ],
                                            },
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: Ref {
                                                guard: Left(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 13,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Ref {
                                                guard: Left(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: Ref {
                                                guard: Left(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 15,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Ref {
                                                guard: Left(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 14,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 16,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            2.0,
                                                        ),
                                                        text: "2.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 17,
                                                opr: Closed(
                                                    Div,
                                                ),
                                                ropd: 18,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: Ref {
                                                guard: Left(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 20,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Ref {
                                                guard: Left(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: Ref {
                                                guard: Left(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 22,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Ref {
                                                guard: Left(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 21,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 23,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            2.0,
                                                        ),
                                                        text: "2.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 24,
                                                opr: Closed(
                                                    Div,
                                                ),
                                                ropd: 25,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorFnCall {
                                                path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        19,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        26,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..5,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
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
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 7,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 12,
                                            coersion: None,
                                        },
                                        Eval {
                                            expr_idx: 27,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `N`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `p0`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `p2`,
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `points`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `N`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `p0`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `p2`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
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
                    path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
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
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
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
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `cc`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            298,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::raw_contour::find_raw_contours`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::NewList {
                                                items: [],
                                                element_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::AssocFunctionFnCall {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist::BinaryGrid28(0)::new_zeros`, `AssocFunctionFn`),
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    29,
                                                ),
                                            ),
                                            ty_place: Const,
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
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 5,
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 7,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 8,
                                            },
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 6,
                                                items: [
                                                    9,
                                                ],
                                            },
                                            ty_place: Leashed,
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
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 11,
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 12,
                                                items: [
                                                    13,
                                                ],
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 15,
                                                opr: Shift(
                                                    Shl,
                                                ),
                                                ropd: 16,
                                            },
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 18,
                                                opr: Shift(
                                                    Shl,
                                                ),
                                                ropd: 19,
                                            },
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 21,
                                                items: [
                                                    22,
                                                ],
                                            },
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 24,
                                                opr: Closed(
                                                    BitOr,
                                                ),
                                                ropd: 25,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 26,
                                                opr: Closed(
                                                    BitOr,
                                                ),
                                                ropd: 27,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                8,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        7,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 28,
                                                opr: Closed(
                                                    BitOr,
                                                ),
                                                ropd: 29,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 31,
                                                opr: Closed(
                                                    BitAnd,
                                                ),
                                                ropd: 32,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 33,
                                                opr: Closed(
                                                    BitAnd,
                                                ),
                                                ropd: 34,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                8,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        7,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 35,
                                                opr: Closed(
                                                    BitAnd,
                                                ),
                                                ropd: 36,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: BitNot,
                                                opd_hir_expr_idx: 37,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 30,
                                                opr: Closed(
                                                    BitAnd,
                                                ),
                                                ropd: 38,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 23,
                                                opr: Assign,
                                                ropd: 39,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    29,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                9,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        8,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 43,
                                                items: [
                                                    44,
                                                ],
                                            },
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::NewList {
                                                items: [],
                                                element_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                9,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        8,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                9,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        8,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 48,
                                                items: [
                                                    49,
                                                ],
                                            },
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 50,
                                                self_contract: Pure,
                                                ident: `ctz`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::raw_bits::r32(0)::ctz`, `MethodFn`),
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
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 52,
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 54,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 55,
                                            },
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 53,
                                                items: [
                                                    56,
                                                ],
                                            },
                                            ty_place: Leashed,
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
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 58,
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 59,
                                                items: [
                                                    60,
                                                ],
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                13,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        12,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                14,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        13,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                12,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::FunctionFnCall {
                                                path: FugitivePath(`mnist_classifier::raw_contour::get_inward_direction`, `FunctionFn`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 12,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        62,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            12,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 12,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        63,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            13,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        64,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            11,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                12,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                15,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        14,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 72,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 74,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 76,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                16,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        15,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 78,
                                                opr: Comparison(
                                                    Eq,
                                                ),
                                                ropd: 79,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                12,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                17,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        16,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 81,
                                                opr: Comparison(
                                                    Eq,
                                                ),
                                                ropd: 82,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 80,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 83,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                15,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        14,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                18,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        17,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 85,
                                                opr: Comparison(
                                                    Eq,
                                                ),
                                                ropd: 86,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 84,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 87,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: NotBool,
                                                opd_hir_expr_idx: 88,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                13,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        12,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                14,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        13,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                12,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                15,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        14,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::FunctionFnCall {
                                                path: FugitivePath(`mnist_classifier::raw_contour::get_outward_direction`, `FunctionFn`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 12,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        90,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            12,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 12,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        91,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            13,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        92,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            11,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 49,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        93,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            14,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                15,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        14,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                25,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        24,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::FunctionFnCall {
                                                path: FugitivePath(`mnist_classifier::raw_contour::get_angle_change`, `FunctionFn`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 49,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        95,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            14,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 49,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        96,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            24,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 98,
                                                items: [
                                                    99,
                                                ],
                                            },
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 101,
                                                items: [
                                                    102,
                                                ],
                                            },
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::R32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                12,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 104,
                                                opr: Shift(
                                                    Shl,
                                                ),
                                                ropd: 105,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: BitNot,
                                                opd_hir_expr_idx: 106,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 103,
                                                opr: Closed(
                                                    BitAnd,
                                                ),
                                                ropd: 107,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 100,
                                                opr: Assign,
                                                ropd: 108,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                26,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        25,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                19,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        18,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 112,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 111,
                                                opr: Comparison(
                                                    Eq,
                                                ),
                                                ropd: 113,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                20,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        19,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 116,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 115,
                                                opr: Comparison(
                                                    Eq,
                                                ),
                                                ropd: 117,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 114,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 118,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                24,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        23,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 120,
                                                opr: Comparison(
                                                    Eq,
                                                ),
                                                ropd: 121,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 119,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 122,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                22,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        21,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 125,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 124,
                                                opr: Comparison(
                                                    Neq,
                                                ),
                                                ropd: 126,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 123,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 127,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                23,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        22,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 129,
                                                opr: Comparison(
                                                    Eq,
                                                ),
                                                ropd: 130,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 128,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 131,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                10,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 133,
                                                self_contract: At,
                                                ident: `last`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::last`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::SelfPlace,
                                                            ),
                                                            HirTermSvarResolution::SelfPlace(
                                                                MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            9,
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unwrap {
                                                opd_hir_expr_idx: 134,
                                            },
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                10,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::FunctionFnCall {
                                                path: FugitivePath(`mnist_classifier::raw_contour::get_concave_middle_point`, `FunctionFn`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 20,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        136,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            9,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 135,
                                                opr: Assign,
                                                ropd: 137,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                10,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                12,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::AssocFunctionFnCall {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssocFunctionFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        140,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            10,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        141,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            11,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 139,
                                                self_contract: BorrowMut,
                                                ident: `push`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::SelfLifetime,
                                                            ),
                                                            HirTermSvarResolution::SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 19,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        142,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                23,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        22,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 145,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 144,
                                                opr: Assign,
                                                ropd: 146,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                22,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        21,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 149,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 148,
                                                opr: Assign,
                                                ropd: 150,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                19,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        18,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 153,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 152,
                                                opr: Comparison(
                                                    Eq,
                                                ),
                                                ropd: 154,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                22,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        21,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 156,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 157,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 155,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 158,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                22,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        21,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 160,
                                                opr: Comparison(
                                                    Eq,
                                                ),
                                                ropd: 161,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 159,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 162,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                10,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 164,
                                                self_contract: At,
                                                ident: `last`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::last`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::SelfPlace,
                                                            ),
                                                            HirTermSvarResolution::SelfPlace(
                                                                MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            9,
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unwrap {
                                                opd_hir_expr_idx: 165,
                                            },
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                12,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::AssocFunctionFnCall {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssocFunctionFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        167,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            10,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        168,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            11,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 166,
                                                opr: Assign,
                                                ropd: 169,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                23,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        22,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                22,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        21,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 171,
                                                opr: Assign,
                                                ropd: 172,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                22,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        21,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                24,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        23,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 174,
                                                opr: Assign,
                                                ropd: 175,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                19,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        18,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 178,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 177,
                                                opr: Comparison(
                                                    Eq,
                                                ),
                                                ropd: 179,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                22,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        21,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 181,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 182,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 180,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 183,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                24,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        23,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 185,
                                                opr: Comparison(
                                                    Eq,
                                                ),
                                                ropd: 186,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 184,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 187,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                22,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        21,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 189,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 190,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 188,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 191,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                10,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 193,
                                                self_contract: At,
                                                ident: `last`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::last`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::SelfPlace,
                                                            ),
                                                            HirTermSvarResolution::SelfPlace(
                                                                MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            9,
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unwrap {
                                                opd_hir_expr_idx: 194,
                                            },
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                12,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::AssocFunctionFnCall {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssocFunctionFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        196,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            10,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        197,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            11,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 195,
                                                opr: Assign,
                                                ropd: 198,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                23,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        22,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 201,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 200,
                                                opr: Assign,
                                                ropd: 202,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                22,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        21,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 205,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 204,
                                                opr: Assign,
                                                ropd: 206,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                10,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                12,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::AssocFunctionFnCall {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssocFunctionFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        209,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            10,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        210,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            11,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 208,
                                                self_contract: BorrowMut,
                                                ident: `push`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::SelfLifetime,
                                                            ),
                                                            HirTermSvarResolution::SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 19,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        211,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                23,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        22,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                22,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        21,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 213,
                                                opr: Assign,
                                                ropd: 214,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                22,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        21,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                24,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        23,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 216,
                                                opr: Assign,
                                                ropd: 217,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                24,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        23,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 219,
                                                opr: Assign,
                                                ropd: 220,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                20,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        19,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                19,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        18,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 222,
                                                opr: Assign,
                                                ropd: 223,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                19,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        18,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                26,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        25,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 225,
                                                opr: Assign,
                                                ropd: 226,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                25,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        24,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 230,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 231,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 229,
                                                opr: Assign,
                                                ropd: 232,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                14,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        13,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                13,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        12,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 234,
                                                opr: Assign,
                                                ropd: 235,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                13,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        12,
                                                    ),
                                                ),
                                            },
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
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 238,
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 240,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 241,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 239,
                                                items: [
                                                    242,
                                                ],
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 237,
                                                opr: Assign,
                                                ropd: 243,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 246,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 247,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 245,
                                                opr: Assign,
                                                ropd: 248,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                13,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        12,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                14,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        13,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 250,
                                                opr: Assign,
                                                ropd: 251,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                14,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        13,
                                                    ),
                                                ),
                                            },
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
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 254,
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        10,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 255,
                                                items: [
                                                    256,
                                                ],
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 253,
                                                opr: Assign,
                                                ropd: 257,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                12,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                12,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 260,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 261,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 259,
                                                opr: Assign,
                                                ropd: 262,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                12,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                12,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        11,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 265,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 266,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 264,
                                                opr: Assign,
                                                ropd: 267,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                15,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        14,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                25,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        24,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 269,
                                                opr: Assign,
                                                ropd: 270,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                24,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        23,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 273,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 272,
                                                opr: Comparison(
                                                    Neq,
                                                ),
                                                ropd: 274,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                24,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        23,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Suffix {
                                                opd_hir_expr_idx: 276,
                                                opr: Incr,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                19,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        18,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 279,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 278,
                                                opr: Comparison(
                                                    Eq,
                                                ),
                                                ropd: 280,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                24,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        23,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 282,
                                                opr: Comparison(
                                                    Eq,
                                                ),
                                                ropd: 283,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 281,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 284,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                22,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        21,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 286,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 287,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 285,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 288,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                10,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 290,
                                                self_contract: BorrowMut,
                                                ident: `pop`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::SelfLifetime,
                                                            ),
                                                            HirTermSvarResolution::SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
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
                                            data: HirEagerExprData::Variable(
                                                10,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        9,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorFnCall {
                                                path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 17,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        293,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 20,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        294,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            9,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 292,
                                                self_contract: BorrowMut,
                                                ident: `push`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                            template_arguments: [],
                                                                            always_copyable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::SelfLifetime,
                                                            ),
                                                            HirTermSvarResolution::SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 5,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        295,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    59..64,
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
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 10,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 14,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 17,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 6,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 20,
                                            coersion: None,
                                        },
                                        Eval {
                                            expr_idx: 40,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 138,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 143,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 147,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 151,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 170,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 173,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 176,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 199,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 203,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 207,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 212,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 215,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 218,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 132,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    6..10,
                                                ),
                                            },
                                            elif_branches: [
                                                HirEagerElifBranch {
                                                    condition: Other {
                                                        hir_eager_expr_idx: 163,
                                                        conversion: None,
                                                    },
                                                    stmts: ArenaIdxRange(
                                                        10..13,
                                                    ),
                                                },
                                                HirEagerElifBranch {
                                                    condition: Other {
                                                        hir_eager_expr_idx: 192,
                                                        conversion: None,
                                                    },
                                                    stmts: ArenaIdxRange(
                                                        13..16,
                                                    ),
                                                },
                                            ],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        16..19,
                                                    ),
                                                },
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 221,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 224,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 227,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 233,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 236,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 244,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 249,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 252,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 258,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 263,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 268,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 277,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 22,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 94,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 23,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 97,
                                            coersion: None,
                                        },
                                        Eval {
                                            expr_idx: 109,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 110,
                                                    conversion: IntToBool(
                                                        I32,
                                                    ),
                                                },
                                                stmts: ArenaIdxRange(
                                                    19..23,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Match {
                                            case_branches: [
                                                HirEagerCaseBranch {
                                                    pattern: 24,
                                                    stmts: ArenaIdxRange(
                                                        23..26,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 25,
                                                    stmts: ArenaIdxRange(
                                                        26..29,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 26,
                                                    stmts: ArenaIdxRange(
                                                        29..30,
                                                    ),
                                                },
                                                HirEagerCaseBranch {
                                                    pattern: 27,
                                                    stmts: ArenaIdxRange(
                                                        30..31,
                                                    ),
                                                },
                                            ],
                                            match_target: 228,
                                        },
                                        Eval {
                                            expr_idx: 271,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 275,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    31..32,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 291,
                                            coersion: None,
                                            discarded: true,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 7,
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 20,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            contract: Move,
                                            initial_value: 46,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 8,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 47,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 9,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 51,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 10,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 57,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 11,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 61,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 12,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 65,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 13,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 66,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 14,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 67,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 15,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 68,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 16,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 69,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 17,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 70,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 18,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 71,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 19,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 73,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 20,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 75,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 21,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 77,
                                            coersion: None,
                                        },
                                        DoWhile {
                                            condition: Other {
                                                hir_eager_expr_idx: 89,
                                                conversion: None,
                                            },
                                            block: ArenaIdxRange(
                                                32..39,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 289,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    39..40,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 296,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        While {
                                            condition: Other {
                                                hir_eager_expr_idx: 45,
                                                conversion: IntToBool(
                                                    R32,
                                                ),
                                            },
                                            stmts: ArenaIdxRange(
                                                40..58,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 16,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            contract: Move,
                                            initial_value: 1,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 2,
                                            coersion: None,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            3,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            4,
                                                        ),
                                                        kind: UpperClosed,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                1..6,
                                            ),
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 81,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            41,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            42,
                                                        ),
                                                        kind: UpperClosed,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                58..59,
                                            ),
                                        },
                                        Return {
                                            result: 297,
                                            coersion: Trivial(
                                                TrivialHirEagerCoersion {
                                                    expectee_place: MutableStackOwned {
                                                        location: PlaceIdx(
                                                            ShiftedU32(
                                                                1,
                                                            ),
                                                        ),
                                                    },
                                                },
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `result`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `boundary_unsearched`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `r_ur`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `r_dr`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `r_ul`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `r_dl`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `contour`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `i`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `j`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `row_above`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `row_below`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `inward_direction`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `i0`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `j0`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `dir0`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `prev_angle_change1`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `prev_angle_change2`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `total_angle_change`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `prev_streak1`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `prev_streak2`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `current_streak`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `outward_direction`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `angle_change`,
                                        },
                                        HirEagerPatternExpr::Unit(
                                            PatternPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                ident: `Up`,
                                                                index: U8(
                                                                    0,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerPatternExpr::Unit(
                                            PatternPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                ident: `Down`,
                                                                index: U8(
                                                                    2,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerPatternExpr::Unit(
                                            PatternPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                ident: `Left`,
                                                                index: U8(
                                                                    1,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                        HirEagerPatternExpr::Unit(
                                            PatternPath::TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId {
                                                        data: ItemPathData::TypeVariant(
                                                            TypeVariantPathData {
                                                                parent_ty_path: TypePath(`mnist_classifier::raw_contour::Direction`, `Enum`),
                                                                ident: `Right`,
                                                                index: U8(
                                                                    3,
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `cc`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `result`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `boundary_unsearched`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LoopVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `r_ur`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `r_dr`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `r_ul`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `r_dl`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `k`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LoopVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `contour`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `j`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `row_above`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `row_below`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `inward_direction`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `i0`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `j0`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `dir0`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `prev_angle_change1`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `prev_angle_change2`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `total_angle_change`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `prev_streak1`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `prev_streak2`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `current_streak`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `outward_direction`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `angle_change`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
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
    HirDefn::ImplBlock(
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
                    path: TraitForTypeImplBlock {
                        data: TraitForTypeImplBlockPathData {
                            module_path: `mnist_classifier::raw_contour`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
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
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist_classifier::raw_contour`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
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
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
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
                TraitForTypeMethodFnHirDefn {
                    path: TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssocItem(
                                AssocItemPathData::TraitForTypeItem(
                                    TraitForTypeItemPathData {
                                        impl_block: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `mnist_classifier::raw_contour`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                        ident: `visualize`,
                                        item_kind: MethodFn,
                                    },
                                ),
                            ),
                        },
                    ),
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssocItem(
                                    AssocItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `mnist_classifier::raw_contour`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `visualize`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter {
                            contract: Pure,
                            self_ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 5,
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
                            path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssocItem(
                                                    AssocItemPathData::TraitForTypeItem(
                                                        TraitForTypeItemPathData {
                                                            impl_block: TraitForTypeImplBlock {
                                                                data: TraitForTypeImplBlockPathData {
                                                                    module_path: `mnist_classifier::raw_contour`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                            ident: `visualize`,
                                                            item_kind: MethodFn,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
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
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::SelfValue,
                                            data: HirEagerRuntimeSvarData::SelfValue,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            4,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssocItem(
                                        AssocItemPath::TraitForTypeItem(
                                            TraitForTypeItemPath(
                                                ItemPathId {
                                                    data: ItemPathData::AssocItem(
                                                        AssocItemPathData::TraitForTypeItem(
                                                            TraitForTypeItemPathData {
                                                                impl_block: TraitForTypeImplBlock {
                                                                    data: TraitForTypeImplBlockPathData {
                                                                        module_path: `mnist_classifier::raw_contour`,
                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                                ident: `visualize`,
                                                                item_kind: MethodFn,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
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
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
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
                                            },
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::EmptyHtmlTag {
                                                function_ident: `Contour`,
                                                arguments: [
                                                    HirEagerHtmlArgumentExpr {
                                                        property_ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 68,
                                                                },
                                                            ),
                                                        ),
                                                        expr: 2,
                                                    },
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
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
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::SelfValue,
                                                data: HirEagerRuntimeSvarData::SelfValue,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
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
                    path: TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::raw_contour`,
                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        },
                    ),
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
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TypeImplBlock(
                                    TypeImplBlockPath(
                                        ItemPathId {
                                            data: ItemPathData::ImplBlock(
                                                ImplBlockPathData::TypeImplBlock(
                                                    TypeImplBlockPathData {
                                                        module_path: `mnist_classifier::raw_contour`,
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
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
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
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
                    path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`, `MemoizedField`),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`, `MemoizedField`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`, `MemoizedField`),
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
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::SelfValue,
                                            data: HirEagerRuntimeSvarData::SelfValue,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            4,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssocItem(
                                        AssocItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::line_segment_sketch`, `MemoizedField`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            1.4,
                                                        ),
                                                        text: "1.4f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::AssocFunctionFnCall {
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`, `AssocFunctionFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 6,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        1,
                                                        PlaceToLeash,
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        2,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Const,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
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
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::SelfValue,
                                                data: HirEagerRuntimeSvarData::SelfValue,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
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
                    path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
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
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::SelfValue,
                                            data: HirEagerRuntimeSvarData::SelfValue,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            51,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssocItem(
                                        AssocItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
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
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::USize(
                                                    TermUSizeLiteral {
                                                        value: 0,
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 2,
                                                items: [
                                                    3,
                                                ],
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 5,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 7,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 9,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 11,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 13,
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
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 14,
                                                self_contract: Pure,
                                                ident: `ilen`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 16,
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
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 17,
                                                items: [
                                                    18,
                                                ],
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                8,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 22,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 21,
                                                self_contract: Pure,
                                                ident: `min`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::min`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        23,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Leashed,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 20,
                                                opr: Assign,
                                                ropd: 24,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                8,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 28,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 27,
                                                self_contract: Pure,
                                                ident: `max`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::max`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        29,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Leashed,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 26,
                                                opr: Assign,
                                                ropd: 30,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                8,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 34,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 33,
                                                self_contract: Pure,
                                                ident: `min`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::min`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        35,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Leashed,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 32,
                                                opr: Assign,
                                                ropd: 36,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                8,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 40,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 39,
                                                self_contract: Pure,
                                                ident: `max`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::max`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        41,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Leashed,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 38,
                                                opr: Assign,
                                                ropd: 42,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorFnCall {
                                                path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        44,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        45,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            2,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorFnCall {
                                                path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        47,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        48,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: PlaceIdx(
                                                                        ShiftedU32(
                                                                            4,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorFnCall {
                                                path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 40,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        46,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 40,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        49,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    6..13,
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
                                                pattern_expr_idx: 6,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 19,
                                            coersion: None,
                                        },
                                        Eval {
                                            expr_idx: 25,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 31,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 37,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 43,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 4,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 6,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 8,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 10,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 12,
                                            coersion: None,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: None,
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            15,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                1..6,
                                            ),
                                        },
                                        Return {
                                            result: 50,
                                            coersion: Trivial(
                                                TrivialHirEagerCoersion {
                                                    expectee_place: Transient,
                                                },
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `start_point`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `xmin`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `xmax`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `ymin`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `ymax`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `point`,
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::SelfValue,
                                                data: HirEagerRuntimeSvarData::SelfValue,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `start_point`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `xmin`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `xmax`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `ymin`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `ymax`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LoopVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `point`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
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
                    path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`, `MemoizedField`),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`, `MemoizedField`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`, `MemoizedField`),
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
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::SelfValue,
                                            data: HirEagerRuntimeSvarData::SelfValue,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            10,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssocItem(
                                        AssocItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::relative_bounding_box`, `MemoizedField`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
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
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MemoizedField {
                                                owner_hir_expr_idx: 2,
                                                ident: `raw_contours`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`, `MemoizedField`),
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::USize(
                                                    TermUSizeLiteral {
                                                        value: 0,
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 3,
                                                items: [
                                                    4,
                                                ],
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MemoizedField {
                                                owner_hir_expr_idx: 5,
                                                ident: `bounding_box`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MemoizedField {
                                                owner_hir_expr_idx: 7,
                                                ident: `bounding_box`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 6,
                                                self_contract: Pure,
                                                ident: `relative_bounding_box`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 41,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        8,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Leashed,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 9,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
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
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::SelfValue,
                                                data: HirEagerRuntimeSvarData::SelfValue,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
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
                    path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::contour_len`, `MemoizedField`),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::contour_len`, `MemoizedField`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::contour_len`, `MemoizedField`),
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
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::SelfValue,
                                            data: HirEagerRuntimeSvarData::SelfValue,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            59,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssocItem(
                                        AssocItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::contour_len`, `MemoizedField`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 3,
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
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 4,
                                                self_contract: Pure,
                                                ident: `ilen`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 6,
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
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 8,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 9,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 7,
                                                items: [
                                                    10,
                                                ],
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 12,
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
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 13,
                                                items: [
                                                    14,
                                                ],
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 17,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 19,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 18,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 20,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 21,
                                                self_contract: Pure,
                                                ident: `abs`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::abs`, `MethodFn`),
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
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 23,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 25,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 24,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 26,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 27,
                                                self_contract: Pure,
                                                ident: `abs`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::abs`, `MethodFn`),
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
                                            data: HirEagerExprData::Binary {
                                                lopd: 22,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 28,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 16,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                ropd: 29,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 31,
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
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 33,
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
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 34,
                                                self_contract: Pure,
                                                ident: `ilen`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 35,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 36,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 32,
                                                items: [
                                                    37,
                                                ],
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 39,
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
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::USize(
                                                    TermUSizeLiteral {
                                                        value: 0,
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 40,
                                                items: [
                                                    41,
                                                ],
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 44,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 46,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 45,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 47,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 48,
                                                self_contract: Pure,
                                                ident: `abs`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::abs`, `MethodFn`),
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
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 50,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            ty_place: Leashed,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 52,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Leashed,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 51,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 53,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 54,
                                                self_contract: Pure,
                                                ident: `abs`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::abs`, `MethodFn`),
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
                                            data: HirEagerExprData::Binary {
                                                lopd: 49,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 55,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 43,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                ropd: 56,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: MutableStackOwned {
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
                                                    4..10,
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
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 11,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 15,
                                            coersion: None,
                                        },
                                        Eval {
                                            expr_idx: 30,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 1,
                                            coersion: None,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            2,
                                                        ),
                                                        kind: LowerOpen,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            5,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                1..4,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 38,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 42,
                                            coersion: None,
                                        },
                                        Eval {
                                            expr_idx: 57,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Return {
                                            result: 58,
                                            coersion: Trivial(
                                                TrivialHirEagerCoersion {
                                                    expectee_place: MutableStackOwned {
                                                        location: PlaceIdx(
                                                            ShiftedU32(
                                                                1,
                                                            ),
                                                        ),
                                                    },
                                                },
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `contour_len`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `a`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `b`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `a`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `b`,
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::SelfValue,
                                                data: HirEagerRuntimeSvarData::SelfValue,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `contour_len`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LoopVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `a`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `b`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `a`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `b`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
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
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::displacement`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::displacement`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter {
                            contract: Pure,
                            self_ty: PathLeading(
                                HirTypePathLeading(
                                    Id {
                                        value: 5,
                                    },
                                ),
                            ),
                        },
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 2,
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
                            path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::displacement`, `MethodFn`),
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
                                        ident: `start`,
                                    },
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `end`,
                                    },
                                ],
                            },
                            comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::SelfValue,
                                            data: HirEagerRuntimeSvarData::SelfValue,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `start`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `end`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            19,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssocItem(
                                        AssocItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::displacement`, `MethodFn`),
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
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
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
                                            },
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 2,
                                                self_contract: Pure,
                                                ident: `ilen`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                item_groups: [],
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
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 4,
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
                                            },
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 6,
                                                opr: Closed(
                                                    RemEuclid,
                                                ),
                                                ropd: 7,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 5,
                                                items: [
                                                    8,
                                                ],
                                            },
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
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
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 10,
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
                                            },
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 12,
                                                opr: Closed(
                                                    RemEuclid,
                                                ),
                                                ropd: 13,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 11,
                                                items: [
                                                    14,
                                                ],
                                            },
                                            ty_place: StackPure {
                                                location: PlaceIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: Ref {
                                                guard: Left(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: Ref {
                                                guard: Left(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 16,
                                                self_contract: Pure,
                                                ident: `to`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::to`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 19,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        17,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Ref {
                                                                    guard: Left(
                                                                        PlaceIdx(
                                                                            ShiftedU32(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..5,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
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
                                            initial_value: 3,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 9,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 15,
                                            coersion: None,
                                        },
                                        Eval {
                                            expr_idx: 18,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `N`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `ct_start`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `ct_end`,
                                        },
                                    ],
                                },
                                comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::SelfValue,
                                                data: HirEagerRuntimeSvarData::SelfValue,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `start`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `end`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `N`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `ct_start`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `ct_end`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
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