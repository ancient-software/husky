```rust
EntityTreePresheet {
    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
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
                                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 75,
                        ident_token: IdentToken {
                            ident: `ConcaveComponent`,
                            token_idx: TokenIdx(
                                35,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `ConcaveComponent`,
                visibility: Scope::Pub,
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 78,
                        ident_token: IdentToken {
                            ident: `find_concave_components`,
                            token_idx: TokenIdx(
                                554,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FugitiveBody {
                                    ast_idx_range: ArenaIdxRange(
                                        62..70,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `find_concave_components`,
                visibility: Scope::Pub,
            },
        ],
    },
    once_use_rules: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 70,
                use_expr_idx: 3,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
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
                state: UseOneRuleState::Unresolved,
            },
            OnceUseRule {
                ast_idx: 71,
                use_expr_idx: 7,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                8,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        6..7,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Unresolved,
            },
            OnceUseRule {
                ast_idx: 72,
                use_expr_idx: 11,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                16,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        10..11,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Unresolved,
            },
            OnceUseRule {
                ast_idx: 73,
                use_expr_idx: 14,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                24,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        13..14,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Unresolved,
            },
            OnceUseRule {
                ast_idx: 74,
                use_expr_idx: 16,
                visibility: Scope::PubUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                30,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        15..16,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Unresolved,
            },
        ],
    ),
    use_all_rules: UseAllRules(
        [],
    ),
    use_expr_arena: Arena {
        data: [
            UseExpr::All {
                star_token: StarToken(
                    TokenIdx(
                        6,
                    ),
                ),
            },
            UseExpr::Parent(
                ParentUseExprData {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `line_segment_sketch`,
                            token_idx: TokenIdx(
                                4,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                5,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 1,
                        },
                    ),
                },
            ),
            UseExpr::Parent(
                ParentUseExprData {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                2,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                3,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 2,
                        },
                    ),
                },
            ),
            UseExpr::All {
                star_token: StarToken(
                    TokenIdx(
                        14,
                    ),
                ),
            },
            UseExpr::Parent(
                ParentUseExprData {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `line_segment`,
                            token_idx: TokenIdx(
                                12,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                13,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 4,
                        },
                    ),
                },
            ),
            UseExpr::Parent(
                ParentUseExprData {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `line_segment_sketch`,
                            token_idx: TokenIdx(
                                10,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                11,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 5,
                        },
                    ),
                },
            ),
            UseExpr::Parent(
                ParentUseExprData {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                8,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                9,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 6,
                        },
                    ),
                },
            ),
            UseExpr::All {
                star_token: StarToken(
                    TokenIdx(
                        22,
                    ),
                ),
            },
            UseExpr::Parent(
                ParentUseExprData {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `convexity`,
                            token_idx: TokenIdx(
                                20,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                21,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 8,
                        },
                    ),
                },
            ),
            UseExpr::Parent(
                ParentUseExprData {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `line_segment_sketch`,
                            token_idx: TokenIdx(
                                18,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                19,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 9,
                        },
                    ),
                },
            ),
            UseExpr::Parent(
                ParentUseExprData {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                16,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                17,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 10,
                        },
                    ),
                },
            ),
            UseExpr::All {
                star_token: StarToken(
                    TokenIdx(
                        28,
                    ),
                ),
            },
            UseExpr::Parent(
                ParentUseExprData {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `geom2d`,
                            token_idx: TokenIdx(
                                26,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                27,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 12,
                        },
                    ),
                },
            ),
            UseExpr::Parent(
                ParentUseExprData {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                24,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                25,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 13,
                        },
                    ),
                },
            ),
            UseExpr::All {
                star_token: StarToken(
                    TokenIdx(
                        32,
                    ),
                ),
            },
            UseExpr::Parent(
                ParentUseExprData {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                30,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                31,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 15,
                        },
                    ),
                },
            ),
        ],
    },
}
```