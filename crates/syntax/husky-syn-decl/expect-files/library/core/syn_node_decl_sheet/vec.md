SynNodeDeclSheet {
    decls: [
        (
            ItemSynNodePath::MajorItem(
                MajorItemSynNodePath::Type(
                    TypeSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::MajorItem(
                                MajorItemSynNodePathData::Type(
                                    TypeSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypePath(`core::vec::Vec`, `Extern`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::MajorItem(
                MajorItemSynNodeDecl::Type(
                    TypeSynNodeDecl::Extern(
                        ExternTypeSynNodeDecl {
                            syn_node_path: TypeSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Type(
                                            TypeSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypePath(`core::vec::Vec`, `Extern`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameter_decl_list: Ok(
                                Some(
                                    SynTemplateParameterSyndicateList {
                                        langle: LaOrLtRegionalToken(
                                            RegionalTokenIdx(
                                                4,
                                            ),
                                        ),
                                        template_parameters: [
                                            TemplateSynParameterData {
                                                annotated_variance_token: Some(
                                                    VarianceRegionalToken::Covariant(
                                                        CovariantRegionalToken {
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                symbol: 1,
                                                variant: TemplateParameterSyndicateVariant::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `E`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                    traits: None,
                                                },
                                            },
                                        ],
                                        commas: [],
                                        decl_list_result: Ok(
                                            (),
                                        ),
                                        rangle: RaOrGtRegionalToken(
                                            RegionalTokenIdx(
                                                7,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Type(
                                                                TypeSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypePath(`core::vec::Vec`, `Extern`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: Some(
                                                            VarianceRegionalToken::Covariant(
                                                                CovariantRegionalToken {
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `E`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    6,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::ImplBlock(
                ImplBlockSynNodePath::TypeImplBlock(
                    TypeImplBlockSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::ImplBlock(
                                ImplBlockSynNodePathData::TypeImplBlock(
                                    TypeImplBlockSynNodePathData {
                                        path: TypeImplBlockPath(
                                            ItemPathId(
                                                Id {
                                                    value: 123,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::ImplBlock(
                ImplBlockSynNodeDecl::Type(
                    TypeImplBlockSynNodeDecl {
                        syn_node_path: TypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TypeImplBlock(
                                        TypeImplBlockSynNodePathData {
                                            path: TypeImplBlockPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 123,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            },
                        ),
                        impl_regional_token: ImplRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                1,
                            ),
                        },
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            2,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateSynParameterData {
                                            annotated_variance_token: None,
                                            symbol: 1,
                                            variant: TemplateParameterSyndicateVariant::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `E`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        3,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        self_ty_expr: SelfTypeSyndicate {
                            expr: 3,
                        },
                        eol_colon: Ok(
                            EolRegionalToken::Colon(
                                EolColonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::Decl(
                                    ItemSynNodePath::ImplBlock(
                                        ImplBlockSynNodePath::TypeImplBlock(
                                            TypeImplBlockSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::ImplBlock(
                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                            TypeImplBlockSynNodePathData {
                                                                path: TypeImplBlockPath(
                                                                    ItemPathId(
                                                                        Id {
                                                                            value: 123,
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                syn_expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::vec::Vec`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `E`,
                                            regional_token_idx: RegionalTokenIdx(
                                                6,
                                            ),
                                            current_syn_symbol_idx: 1,
                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `E`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 1,
                                            argument_expr_idx: 2,
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `Vec`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::vec::Vec`, `Extern`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: SynPatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [],
                                    },
                                    pattern_symbol_arena: Arena {
                                        data: [],
                                    },
                                    pattern_symbol_maps: [],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                symbol_region: VariableRegionData {
                                    inherited_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_syn_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    4,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `E`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                1..2,
                                            ),
                                        ),
                                    ],
                                },
                                syn_pattern_expr_roots: [],
                                syn_expr_roots: [
                                    SynExprRoot {
                                        kind: SynExprRootKind::SelfType,
                                        syn_expr_idx: 3,
                                    },
                                ],
                                has_self_lifetime: false,
                                has_self_place: false,
                                syn_pattern_to_current_syn_symbol_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemSynNodePath::AssocItem(
                AssocItemSynNodePath::TypeItem(
                    TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssocItem(
                                AssocItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                                                Fn,
                                            )`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MethodFn(
                        TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameters: Ok(
                                None,
                            ),
                            parenate_parameters: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            5,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 1,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Semicolon(
                                    EolSemicolonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            8,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 123,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                syn_expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::vec::Vec`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Vec`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::vec::Vec`, `Extern`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: SynPatternExprRegion {
                                                    pattern_expr_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `E`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                3,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 3,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                                                                            Fn,
                                                                        )`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `E`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::AssocItem(
                AssocItemSynNodePath::TypeItem(
                    TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssocItem(
                                AssocItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(`(core::vec::Vec(0)::push`, `MethodRitchie(
                                                Fn,
                                            )`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MethodFn(
                        TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(`(core::vec::Vec(0)::push`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameters: Ok(
                                None,
                            ),
                            parenate_parameters: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    self_value_parameter: Some(
                                        SelfValueParameterSyndicate {
                                            ephem_symbol_modifier_token_verse: Some(
                                                AmbersandMut(
                                                    AmbersandRegionalToken(
                                                        RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    ),
                                                    None,
                                                    MutRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            self_value_token: SelfValueRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                        },
                                    ),
                                    comma_after_self_parameter: Some(
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                8,
                                            ),
                                        ),
                                    ),
                                    parenate_parameters: [
                                        ParenateParameterSyndicate::Simple {
                                            syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                syn_pattern_expr_idx: 1,
                                            },
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    11,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            13,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                None,
                            ),
                            return_ty: Ok(
                                None,
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Semicolon(
                                    EolSemicolonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            14,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 123,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                syn_expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::vec::Vec`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Vec`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::vec::Vec`, `Extern`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: SynPatternExprRegion {
                                                    pattern_expr_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `E`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                3,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 3,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(core::vec::Vec(0)::push`, `MethodRitchie(
                                                                            Fn,
                                                                        )`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [
                                            SynExprData::InheritedSynSymbol {
                                                ident: `E`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternExprData::Ident {
                                                    symbol_modifier_tokens: Some(
                                                        Owned(
                                                            DoubleExclamationRegionalToken(
                                                                RegionalTokenIdx(
                                                                    9,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    ident_token: IdentRegionalToken {
                                                        ident: `e`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                Move,
                                            ],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                SynPatternSymbol::Atom(
                                                    1,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `e`,
                                                    1,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                Owned,
                                            ],
                                        },
                                    },
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `E`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: Owned,
                                                    access_start: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::SimpleParenateParameter {
                                                        ident: `e`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty: 1,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 1,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: true,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [
                                        (
                                            1,
                                            1,
                                        ),
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::AssocItem(
                AssocItemSynNodePath::TypeItem(
                    TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssocItem(
                                AssocItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(`(core::vec::Vec(0)::first`, `MethodRitchie(
                                                Fn,
                                            )`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MethodFn(
                        TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(`(core::vec::Vec(0)::first`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameters: Ok(
                                None,
                            ),
                            parenate_parameters: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    self_value_parameter: Some(
                                        SelfValueParameterSyndicate {
                                            ephem_symbol_modifier_token_verse: Some(
                                                At(
                                                    AtRegionalToken(
                                                        RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    ),
                                                    None,
                                                ),
                                            ),
                                            self_value_token: SelfValueRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                            },
                                        },
                                    ),
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            8,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 5,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Semicolon(
                                    EolSemicolonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 123,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                syn_expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::vec::Vec`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Vec`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::vec::Vec`, `Extern`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: SynPatternExprRegion {
                                                    pattern_expr_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `E`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                3,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 3,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(core::vec::Vec(0)::first`, `MethodRitchie(
                                                                            Fn,
                                                                        )`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::option::Option`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::At {
                                                at_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                                place_label_regional_token: None,
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `E`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 3,
                                                argument_expr_idx: 4,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Option`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::option::Option`, `Enum`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `E`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 5,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: true,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::AssocItem(
                AssocItemSynNodePath::TypeItem(
                    TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssocItem(
                                AssocItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(`(core::vec::Vec(0)::last`, `MethodRitchie(
                                                Fn,
                                            )`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MethodFn(
                        TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(`(core::vec::Vec(0)::last`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameters: Ok(
                                None,
                            ),
                            parenate_parameters: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    self_value_parameter: Some(
                                        SelfValueParameterSyndicate {
                                            ephem_symbol_modifier_token_verse: Some(
                                                At(
                                                    AtRegionalToken(
                                                        RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    ),
                                                    None,
                                                ),
                                            ),
                                            self_value_token: SelfValueRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                            },
                                        },
                                    ),
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            8,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 5,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Semicolon(
                                    EolSemicolonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 123,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                syn_expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::vec::Vec`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Vec`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::vec::Vec`, `Extern`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: SynPatternExprRegion {
                                                    pattern_expr_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `E`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                3,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 3,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(core::vec::Vec(0)::last`, `MethodRitchie(
                                                                            Fn,
                                                                        )`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::option::Option`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::At {
                                                at_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                                place_label_regional_token: None,
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `E`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 3,
                                                argument_expr_idx: 4,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Option`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::option::Option`, `Enum`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `E`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 5,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: true,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::AssocItem(
                AssocItemSynNodePath::TypeItem(
                    TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssocItem(
                                AssocItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodRitchie(
                                                Fn,
                                            )`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MethodFn(
                        TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameters: Ok(
                                None,
                            ),
                            parenate_parameters: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    self_value_parameter: Some(
                                        SelfValueParameterSyndicate {
                                            ephem_symbol_modifier_token_verse: Some(
                                                AmbersandMut(
                                                    AmbersandRegionalToken(
                                                        RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    ),
                                                    None,
                                                    MutRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            self_value_token: SelfValueRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                        },
                                    ),
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            8,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            9,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 3,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Semicolon(
                                    EolSemicolonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            12,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 123,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                syn_expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::vec::Vec`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Vec`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::vec::Vec`, `Extern`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: SynPatternExprRegion {
                                                    pattern_expr_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `E`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                3,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 3,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodRitchie(
                                                                            Fn,
                                                                        )`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::option::Option`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `E`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Option`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::option::Option`, `Enum`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `E`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 3,
                                        },
                                    ],
                                    has_self_lifetime: true,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::AssocItem(
                AssocItemSynNodePath::TypeItem(
                    TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssocItem(
                                AssocItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(`(core::vec::Vec(0)::collect_leashes`, `MethodRitchie(
                                                Fn,
                                            )`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MethodFn(
                        TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(`(core::vec::Vec(0)::collect_leashes`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameters: Ok(
                                None,
                            ),
                            parenate_parameters: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    self_value_parameter: Some(
                                        SelfValueParameterSyndicate {
                                            ephem_symbol_modifier_token_verse: Some(
                                                Tilde(
                                                    TildeRegionalToken(
                                                        RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            self_value_token: SelfValueRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                            },
                                        },
                                    ),
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            8,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 4,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Semicolon(
                                    EolSemicolonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            13,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 123,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                syn_expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::vec::Vec`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Vec`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::vec::Vec`, `Extern`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: SynPatternExprRegion {
                                                    pattern_expr_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `E`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                3,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 3,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(core::vec::Vec(0)::collect_leashes`, `MethodRitchie(
                                                                            Fn,
                                                                        )`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [
                                            SynExprData::InheritedSynSymbol {
                                                ident: `E`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
                                            },
                                            SynExprData::List {
                                                lbox_regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                                items: [],
                                                rbox_regional_token_idx: RegionalTokenIdx(
                                                    10,
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                                opd: 1,
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `E`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 4,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::AssocItem(
                AssocItemSynNodePath::TypeItem(
                    TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssocItem(
                                AssocItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(`(core::vec::Vec(0)::cyclic_slice_leashed`, `MethodRitchie(
                                                Fn,
                                            )`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MethodFn(
                        TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(`(core::vec::Vec(0)::cyclic_slice_leashed`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameters: Ok(
                                None,
                            ),
                            parenate_parameters: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    self_value_parameter: Some(
                                        SelfValueParameterSyndicate {
                                            ephem_symbol_modifier_token_verse: Some(
                                                Tilde(
                                                    TildeRegionalToken(
                                                        RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            self_value_token: SelfValueRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    6,
                                                ),
                                            },
                                        },
                                    ),
                                    comma_after_self_parameter: Some(
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                7,
                                            ),
                                        ),
                                    ),
                                    parenate_parameters: [
                                        ParenateParameterSyndicate::Simple {
                                            syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                syn_pattern_expr_idx: 1,
                                            },
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                            ty: 1,
                                        },
                                        ParenateParameterSyndicate::Simple {
                                            syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                syn_pattern_expr_idx: 2,
                                            },
                                            variables: ArenaIdxRange(
                                                2..3,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    13,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                    ],
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                11,
                                            ),
                                        ),
                                    ],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            15,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            16,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 6,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Semicolon(
                                    EolSemicolonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 123,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                syn_expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::vec::Vec`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Vec`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::vec::Vec`, `Extern`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: SynPatternExprRegion {
                                                    pattern_expr_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `E`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                3,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 3,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(core::vec::Vec(0)::cyclic_slice_leashed`, `MethodRitchie(
                                                                            Fn,
                                                                        )`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `E`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    19,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 3,
                                                argument_expr_idx: 4,
                                            },
                                            SynExprData::Prefix {
                                                opr: Tilde,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    17,
                                                ),
                                                opd: 5,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            14,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `CyclicSlice`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            18,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternExprData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `start`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                },
                                                SynPatternExprData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `end`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                SynPatternSymbol::Atom(
                                                    1,
                                                ),
                                                SynPatternSymbol::Atom(
                                                    2,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `start`,
                                                    1,
                                                ),
                                            ],
                                            [
                                                (
                                                    `end`,
                                                    2,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                                Pure,
                                            ],
                                        },
                                    },
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `E`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::SimpleParenateParameter {
                                                        ident: `start`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::SimpleParenateParameter {
                                                        ident: `end`,
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty: 1,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 2,
                                                    },
                                                    ty: 2,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 1,
                                        },
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 2,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 6,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [
                                        (
                                            1,
                                            1,
                                        ),
                                        (
                                            2,
                                            2,
                                        ),
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
        (
            ItemSynNodePath::AssocItem(
                AssocItemSynNodePath::TypeItem(
                    TypeItemSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::AssocItem(
                                AssocItemSynNodePathData::TypeItem(
                                    TypeItemSynNodePathData {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: TypeItemPath(`(core::vec::Vec(0)::pop_with_largest_opt_f32`, `MethodRitchie(
                                                Fn,
                                            )`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
            ItemSynNodeDecl::AssocItem(
                AssocItemSynNodeDecl::TypeItem(
                    TypeItemSynNodeDecl::MethodFn(
                        TypeMethodFnSynNodeDecl {
                            syn_node_path: TypeItemSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::AssocItem(
                                        AssocItemSynNodePathData::TypeItem(
                                            TypeItemSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath(`(core::vec::Vec(0)::pop_with_largest_opt_f32`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                            template_parameters: Ok(
                                None,
                            ),
                            parenate_parameters: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    self_value_parameter: Some(
                                        SelfValueParameterSyndicate {
                                            ephem_symbol_modifier_token_verse: Some(
                                                AmbersandMut(
                                                    AmbersandRegionalToken(
                                                        RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    ),
                                                    None,
                                                    MutRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            self_value_token: SelfValueRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                        },
                                    ),
                                    comma_after_self_parameter: Some(
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                8,
                                            ),
                                        ),
                                    ),
                                    parenate_parameters: [
                                        ParenateParameterSyndicate::Simple {
                                            syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                syn_pattern_expr_idx: 1,
                                            },
                                            variables: ArenaIdxRange(
                                                1..2,
                                            ),
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    10,
                                                ),
                                            ),
                                            ty: 4,
                                        },
                                    ],
                                    commas: [],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            18,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            19,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 6,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Semicolon(
                                    EolSemicolonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                    },
                                ),
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TypeImplBlock(
                                                            TypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TypeImplBlock(
                                                                            TypeImplBlockSynNodePathData {
                                                                                path: TypeImplBlockPath(
                                                                                    ItemPathId(
                                                                                        Id {
                                                                                            value: 123,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                syn_expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 1,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::vec::Vec`, `Extern`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Vec`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::vec::Vec`, `Extern`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: SynPatternExprRegion {
                                                    pattern_expr_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `E`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                3,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 3,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TypeItem(
                                                TypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TypeItem(
                                                                TypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypeItemPath(`(core::vec::Vec(0)::pop_with_largest_opt_f32`, `MethodRitchie(
                                                                            Fn,
                                                                        )`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [
                                            SynExprData::InheritedSynSymbol {
                                                ident: `E`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    13,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Option,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    16,
                                                ),
                                                opd: 2,
                                            },
                                            SynExprData::Ritchie {
                                                ritchie_kind: Type(
                                                    Item(
                                                        Fn,
                                                    ),
                                                ),
                                                ritchie_kind_regional_token_idx: RegionalTokenIdx(
                                                    11,
                                                ),
                                                lpar_token: LparRegionalToken(
                                                    RegionalTokenIdx(
                                                        12,
                                                    ),
                                                ),
                                                parameter_ty_items: [
                                                    SynCommaListItem {
                                                        syn_expr_idx: 1,
                                                        comma_regional_token_idx: None,
                                                    },
                                                ],
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    14,
                                                ),
                                                light_arrow_token: Some(
                                                    LightArrowRegionalToken(
                                                        RegionalTokenIdx(
                                                            15,
                                                        ),
                                                    ),
                                                ),
                                                return_ty_syn_expr_idx: Some(
                                                    3,
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `E`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    21,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
                                            },
                                            SynExprData::Prefix {
                                                opr: Option,
                                                opr_regional_token_idx: RegionalTokenIdx(
                                                    20,
                                                ),
                                                opd: 5,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `f32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            17,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternExprData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `f`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                Pure,
                                            ],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                SynPatternSymbol::Atom(
                                                    1,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `f`,
                                                    1,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                            ],
                                        },
                                    },
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `E`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        10,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::SimpleParenateParameter {
                                                        ident: `f`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty: 4,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 1,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 6,
                                        },
                                    ],
                                    has_self_lifetime: true,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [
                                        (
                                            1,
                                            1,
                                        ),
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
    ],
}