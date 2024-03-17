```rust
EntityTreeCrateBundle {
    sheets: [
        EntityTreeSheet {
            module_path: `malamute`,
            major_item_node_table: MajorEntityNodeTable {
                entries: [
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Type(
                                    TypeSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Type(
                                                    TypeSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`malamute::Class`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 24,
                                ident_token: IdentToken {
                                    ident: `Class`,
                                    token_idx: TokenIdx(
                                        12,
                                    ),
                                },
                                block: DefnBlock::Type {
                                    path: TypePath(`malamute::Class`, `Enum`),
                                    variants: Some(
                                        TypeVariants {
                                            ast_idx_range: ArenaIdxRange(
                                                1..3,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Type(
                                                TypeSynNodePathData {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`malamute::Class`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        ident: `Class`,
                        visibility: Scope::Pub,
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Type(
                                    TypeSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Type(
                                                    TypeSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 26,
                                ident_token: IdentToken {
                                    ident: `OneVsAll`,
                                    token_idx: TokenIdx(
                                        34,
                                    ),
                                },
                                block: DefnBlock::Type {
                                    path: TypePath(`malamute::OneVsAll`, `Enum`),
                                    variants: Some(
                                        TypeVariants {
                                            ast_idx_range: ArenaIdxRange(
                                                3..5,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Type(
                                                TypeSynNodePathData {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        ident: `OneVsAll`,
                        visibility: Scope::Pub,
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Type(
                                    TypeSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Type(
                                                    TypeSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 30,
                                ident_token: IdentToken {
                                    ident: `OneVsAllResult`,
                                    token_idx: TokenIdx(
                                        179,
                                    ),
                                },
                                block: DefnBlock::Type {
                                    path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                    variants: Some(
                                        TypeVariants {
                                            ast_idx_range: ArenaIdxRange(
                                                13..16,
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Type(
                                                TypeSynNodePathData {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        ident: `OneVsAllResult`,
                        visibility: Scope::Pub,
                    },
                    ItemNodeEntry {
                        node: ItemSynNode::MajorItem(
                            MajorItemSynNode {
                                syn_node_path: MajorItemSynNodePath::Fugitive(
                                    FugitiveSynNodePath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`, (0)),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 32,
                                ident_token: IdentToken {
                                    ident: `narrow_down`,
                                    token_idx: TokenIdx(
                                        304,
                                    ),
                                },
                                block: DefnBlock::Fugitive {
                                    path: FugitivePath(`malamute::narrow_down`, `Ritchie(
                                        Gn,
                                    )`),
                                    body: None,
                                },
                            },
                        ),
                        syn_node_path: ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Fugitive(
                                FugitiveSynNodePath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`, (0)),
                            ),
                        ),
                        ident: `narrow_down`,
                        visibility: Scope::Pub,
                    },
                ],
            },
            item_symbol_table: EntitySymbolTable(
                [
                    EntitySymbolEntry {
                        ident: `Class`,
                        visibility: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`malamute::Class`, `Enum`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `OneVsAll`,
                        visibility: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`malamute::OneVsAll`, `Enum`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `OneVsAllResult`,
                        visibility: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                            ),
                        },
                    },
                    EntitySymbolEntry {
                        ident: `narrow_down`,
                        visibility: Scope::Pub,
                        symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Fugitive(
                                FugitivePath(`malamute::narrow_down`, `Ritchie(
                                    Gn,
                                )`),
                            ),
                        },
                    },
                ],
            ),
            impl_block_syn_node_table: [
                (
                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePathData {
                                            path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::default::Default(0)`),
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    ImplBlockSynNode::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNode {
                            syn_node_path: TraitForTypeImplBlockSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::ImplBlock(
                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                            TraitForTypeImplBlockSynNodePathData {
                                                path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::default::Default(0)`),
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 27,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    51,
                                ),
                            },
                            trai_expr: 1,
                            for_token: TokenIdx(
                                65,
                            ),
                            ty_sketch_expr: Path(
                                2,
                            ),
                            items: Some(
                                TraitForType(
                                    TraitForTypeItems {
                                        ast_idx_range: ArenaIdxRange(
                                            6..7,
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
                (
                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePathData {
                                            path: TraitForTypeImplBlockPath(`malamute::Class as core::ops::Unveil(0)`),
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    ImplBlockSynNode::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNode {
                            syn_node_path: TraitForTypeImplBlockSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::ImplBlock(
                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                            TraitForTypeImplBlockSynNodePathData {
                                                path: TraitForTypeImplBlockPath(`malamute::Class as core::ops::Unveil(0)`),
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 28,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    81,
                                ),
                            },
                            trai_expr: 5,
                            for_token: TokenIdx(
                                100,
                            ),
                            ty_sketch_expr: Path(
                                6,
                            ),
                            items: Some(
                                TraitForType(
                                    TraitForTypeItems {
                                        ast_idx_range: ArenaIdxRange(
                                            11..13,
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
                (
                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::ImplBlock(
                                    ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePathData {
                                            path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                    ImplBlockSynNode::TraitForTypeImplBlock(
                        TraitForTypeImplBlockSynNode {
                            syn_node_path: TraitForTypeImplBlockSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::ImplBlock(
                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                            TraitForTypeImplBlockSynNodePathData {
                                                path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                                            },
                                        ),
                                    ),
                                },
                            ),
                            ast_idx: 31,
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    198,
                                ),
                            },
                            trai_expr: 9,
                            for_token: TokenIdx(
                                219,
                            ),
                            ty_sketch_expr: Path(
                                10,
                            ),
                            items: Some(
                                TraitForType(
                                    TraitForTypeItems {
                                        ast_idx_range: ArenaIdxRange(
                                            21..23,
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ],
            once_use_rules: OnceUseRules(
                [],
            ),
            use_all_rules: UseAllRules(
                [],
            ),
            errors: [],
        },
    ],
    principal_item_path_expr_arena: Arena {
        data: [
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `Default`,
                        token_idx: TokenIdx(
                            64,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Trait(
                        TraitPath(`core::default::Default`),
                    ),
                ),
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `OneVsAll`,
                        token_idx: TokenIdx(
                            66,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`malamute::OneVsAll`, `Enum`),
                    ),
                ),
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `Unveil`,
                        token_idx: TokenIdx(
                            96,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Trait(
                        TraitPath(`core::ops::Unveil`),
                    ),
                ),
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `ops`,
                        token_idx: TokenIdx(
                            94,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        95,
                    ),
                ),
                subexpr: 3,
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `core`,
                        token_idx: TokenIdx(
                            92,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        93,
                    ),
                ),
                subexpr: 4,
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `Class`,
                        token_idx: TokenIdx(
                            101,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`malamute::Class`, `Enum`),
                    ),
                ),
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `Unveil`,
                        token_idx: TokenIdx(
                            215,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Trait(
                        TraitPath(`core::ops::Unveil`),
                    ),
                ),
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `ops`,
                        token_idx: TokenIdx(
                            213,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        214,
                    ),
                ),
                subexpr: 7,
            },
            MajorItemPathExpr::Subitem {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `core`,
                        token_idx: TokenIdx(
                            211,
                        ),
                    },
                ),
                colon_colon_token: ColonColonToken(
                    TokenIdx(
                        212,
                    ),
                ),
                subexpr: 8,
            },
            MajorItemPathExpr::Root {
                name_token: PathNameToken::Ident(
                    IdentToken {
                        ident: `OneVsAll`,
                        token_idx: TokenIdx(
                            220,
                        ),
                    },
                ),
                major_path: MajorEntityPath::MajorItem(
                    MajorItemPath::Type(
                        TypePath(`malamute::OneVsAll`, `Enum`),
                    ),
                ),
            },
        ],
    },
}
```