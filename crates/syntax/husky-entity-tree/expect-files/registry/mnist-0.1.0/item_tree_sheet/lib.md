EntitySynTreeSheet {
    module_path: `mnist`,
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
                                                    path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 17,
                        ident_token: IdentToken {
                            ident: `MnistLabel`,
                            token_idx: TokenIdx(
                                3,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist::MnistLabel`, `Enum`),
                            variants: Some(
                                TypeVariants {
                                    ast_idx_range: ArenaIdxRange(
                                        1..11,
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
                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `MnistLabel`,
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
                                                    path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 19,
                        ident_token: IdentToken {
                            ident: `BinaryImage28`,
                            token_idx: TokenIdx(
                                33,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist::BinaryImage28`, `Extern`),
                            variants: None,
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
                                                path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `BinaryImage28`,
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
                                                    path: TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 23,
                        ident_token: IdentToken {
                            ident: `BinaryGrid28`,
                            token_idx: TokenIdx(
                                74,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist::BinaryGrid28`, `Extern`),
                            variants: None,
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
                                                path: TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `BinaryGrid28`,
                visibility: Scope::Pub,
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Fugitive(
                                            FugitiveSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist::input`, `Val`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 27,
                        ident_token: IdentToken {
                            ident: `input`,
                            token_idx: TokenIdx(
                                115,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist::input`, `Val`),
                            body: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Fugitive(
                                        FugitiveSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: FugitivePath(`mnist::input`, `Val`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `input`,
                visibility: Scope::Pub,
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `MnistLabel`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Type(
                        TypePath(`mnist::MnistLabel`, `Enum`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `BinaryImage28`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Type(
                        TypePath(`mnist::BinaryImage28`, `Extern`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `BinaryGrid28`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Type(
                        TypePath(`mnist::BinaryGrid28`, `Extern`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `input`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Fugitive(
                        FugitivePath(`mnist::input`, `Val`),
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
                                    path: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist::BinaryImage28`, `Extern`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
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
                                        path: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `mnist`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist::BinaryImage28`, `Extern`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 20,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            35,
                        ),
                    },
                    trai_expr: 1,
                    for_token: TokenIdx(
                        37,
                    ),
                    ty_sketch_expr: Path(
                        2,
                    ),
                    items: Some(
                        TraitForType(
                            TraitForTypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    11..12,
                                ),
                            },
                        ),
                    ),
                },
            ),
        ),
        (
            ImplBlockSynNodePath::TypeImplBlock(
                TypeImplBlockSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::ImplBlock(
                            ImplBlockSynNodePathData::TypeImplBlock(
                                TypeImplBlockSynNodePathData {
                                    path: TypeImplBlockPath(
                                        ItemPathId {
                                            data: ItemPathData::ImplBlock(
                                                ImplBlockPathData::TypeImplBlock(
                                                    TypeImplBlockPathData {
                                                        module_path: `mnist`,
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    },
                ),
            ),
            ImplBlockSynNode::TypeImplBlock(
                TypeImplBlockSynNode {
                    syn_node_path: TypeImplBlockSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::ImplBlock(
                                ImplBlockSynNodePathData::TypeImplBlock(
                                    TypeImplBlockSynNodePathData {
                                        path: TypeImplBlockPath(
                                            ItemPathId {
                                                data: ItemPathData::ImplBlock(
                                                    ImplBlockPathData::TypeImplBlock(
                                                        TypeImplBlockPathData {
                                                            module_path: `mnist`,
                                                            ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 21,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            47,
                        ),
                    },
                    ty_expr: 3,
                    items: TypeItems {
                        ast_idx_range: ArenaIdxRange(
                            12..13,
                        ),
                    },
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
                                    path: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist`,
                                            trai_path: TraitPath(`core::ops::IntIndex`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist::BinaryImage28`, `Extern`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
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
                                        path: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `mnist`,
                                                trai_path: TraitPath(`core::ops::IntIndex`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist::BinaryImage28`, `Extern`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 22,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            59,
                        ),
                    },
                    trai_expr: 6,
                    for_token: TokenIdx(
                        65,
                    ),
                    ty_sketch_expr: Path(
                        7,
                    ),
                    items: Some(
                        TraitForType(
                            TraitForTypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    13..14,
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
                                    path: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist::BinaryGrid28`, `Extern`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
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
                                        path: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `mnist`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 24,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            76,
                        ),
                    },
                    trai_expr: 8,
                    for_token: TokenIdx(
                        78,
                    ),
                    ty_sketch_expr: Path(
                        9,
                    ),
                    items: Some(
                        TraitForType(
                            TraitForTypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    14..15,
                                ),
                            },
                        ),
                    ),
                },
            ),
        ),
        (
            ImplBlockSynNodePath::TypeImplBlock(
                TypeImplBlockSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::ImplBlock(
                            ImplBlockSynNodePathData::TypeImplBlock(
                                TypeImplBlockSynNodePathData {
                                    path: TypeImplBlockPath(
                                        ItemPathId {
                                            data: ItemPathData::ImplBlock(
                                                ImplBlockPathData::TypeImplBlock(
                                                    TypeImplBlockPathData {
                                                        module_path: `mnist`,
                                                        ty_path: TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                    },
                ),
            ),
            ImplBlockSynNode::TypeImplBlock(
                TypeImplBlockSynNode {
                    syn_node_path: TypeImplBlockSynNodePath(
                        ItemSynNodePathId {
                            data: ItemSynNodePathData::ImplBlock(
                                ImplBlockSynNodePathData::TypeImplBlock(
                                    TypeImplBlockSynNodePathData {
                                        path: TypeImplBlockPath(
                                            ItemPathId {
                                                data: ItemPathData::ImplBlock(
                                                    ImplBlockPathData::TypeImplBlock(
                                                        TypeImplBlockPathData {
                                                            module_path: `mnist`,
                                                            ty_path: TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 25,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            88,
                        ),
                    },
                    ty_expr: 10,
                    items: TypeItems {
                        ast_idx_range: ArenaIdxRange(
                            15..16,
                        ),
                    },
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
                                    path: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist`,
                                            trai_path: TraitPath(`core::ops::IntIndex`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist::BinaryGrid28`, `Extern`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
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
                                        path: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `mnist`,
                                                trai_path: TraitPath(`core::ops::IntIndex`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist::BinaryGrid28`, `Extern`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    },
                                ),
                            ),
                        },
                    ),
                    ast_idx: 26,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            100,
                        ),
                    },
                    trai_expr: 13,
                    for_token: TokenIdx(
                        106,
                    ),
                    ty_sketch_expr: Path(
                        14,
                    ),
                    items: Some(
                        TraitForType(
                            TraitForTypeItems {
                                ast_idx_range: ArenaIdxRange(
                                    16..17,
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
}