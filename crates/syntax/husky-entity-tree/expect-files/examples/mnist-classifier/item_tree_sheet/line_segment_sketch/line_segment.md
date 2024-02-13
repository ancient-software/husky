EntityTreeSheet {
    module_path: `mnist_classifier::line_segment_sketch::line_segment`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath(
                                ItemSynNodePathId(
                                    Id {
                                        value: 47,
                                    },
                                ),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 17,
                        ident_token: IdentToken {
                            ident: `LineSegment`,
                            token_idx: TokenIdx(
                                9,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            variants: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(
                        TypeSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 47,
                                },
                            ),
                        ),
                    ),
                ),
                ident: `LineSegment`,
                visibility: Scope::Pub,
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `LineSegment`,
                visibility: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Type(
                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `Point2d`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::line_segment`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::line_segment`,
                        ),
                        ast_idx: 16,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RelativePoint2d`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::line_segment`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::line_segment`,
                        ),
                        ast_idx: 16,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Vector2d`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::line_segment`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::line_segment`,
                        ),
                        ast_idx: 16,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ClosedRange`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::line_segment`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::line_segment`,
                        ),
                        ast_idx: 16,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `BoundingBox`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::line_segment`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::line_segment`,
                        ),
                        ast_idx: 16,
                        use_expr_idx: 1,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RelativeBoundingBox`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::line_segment`,
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::line_segment_sketch::line_segment`,
                        ),
                        ast_idx: 16,
                        use_expr_idx: 1,
                    },
                ),
            },
        ],
    ),
    impl_block_syn_node_table: [
        (
            ImplBlockSynNodePath::TypeImplBlock(
                TypeImplBlockSynNodePath(
                    ItemSynNodePathId(
                        Id {
                            value: 270,
                        },
                    ),
                ),
            ),
            ImplBlockSynNode::TypeImplBlock(
                TypeImplBlockSynNode {
                    syn_node_path: TypeImplBlockSynNodePath(
                        ItemSynNodePathId(
                            Id {
                                value: 270,
                            },
                        ),
                    ),
                    ast_idx: 18,
                    impl_token: ImplToken {
                        token_idx: TokenIdx(
                            20,
                        ),
                    },
                    ty_expr: 23,
                    items: TypeItems {
                        ast_idx_range: ArenaIdxRange(
                            14..16,
                        ),
                    },
                },
            ),
        ),
    ],
    once_use_rules: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 16,
                use_expr_idx: 3,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::line_segment`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        2..3,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 16,
                use_expr_idx: 2,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::line_segment`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `geom2d`,
                            token_idx: TokenIdx(
                                4,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        1..2,
                    ),
                },
                parent: Some(
                    (
                        MajorEntityPath::Module(
                            `mnist_classifier`,
                        ),
                        EntitySymbol::CrateRoot {
                            root_module_path: `mnist_classifier`,
                        },
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(
                                ItemPathId {
                                    data: ItemPathData::SubmoduleItem(
                                        SubmoduleItemPathData {
                                            submodule_path: SubmodulePath(
                                                `mnist_classifier::geom2d`,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
        ],
    ),
    use_all_rules: UseAllRules(
        [
            UseAllRule {
                parent_module_path: `mnist_classifier::geom2d`,
                is_same_crate: true,
                ast_idx: 16,
                use_expr_idx: 1,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::line_segment`,
                ),
                progress: Ok(
                    6,
                ),
            },
        ],
    ),
    errors: [],
}