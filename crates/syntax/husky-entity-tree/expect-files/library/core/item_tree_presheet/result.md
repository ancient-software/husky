EntityTreePresheet {
    module_path: `core::result`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Type(
                            TypeSynNodePath(
                                ItemSynNodePathId(
                                    Id {
                                        value: 76,
                                    },
                                ),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 7,
                        ident_token: IdentToken {
                            ident: `Result`,
                            token_idx: TokenIdx(
                                12,
                            ),
                        },
                        block: DefnBlock::Type {
                            path: TypePath(`core::result::Result`, `Enum`),
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
                            ItemSynNodePathId(
                                Id {
                                    value: 76,
                                },
                            ),
                        ),
                    ),
                ),
                ident: `Result`,
                visibility: Scope::Pub,
            },
        ],
    },
    once_use_rules: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 5,
                use_expr_idx: 2,
                visibility: Scope::PubUnder(
                    `core::result`,
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
                        1..2,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Unresolved,
            },
            OnceUseRule {
                ast_idx: 6,
                use_expr_idx: 4,
                visibility: Scope::Pub,
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `Result`,
                            token_idx: TokenIdx(
                                7,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        3..4,
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
                        4,
                    ),
                ),
            },
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
                            child: 1,
                        },
                    ),
                },
            ),
            UseExpr::All {
                star_token: StarToken(
                    TokenIdx(
                        9,
                    ),
                ),
            },
            UseExpr::Parent(
                ParentUseExprData {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `Result`,
                            token_idx: TokenIdx(
                                7,
                            ),
                        },
                    ),
                    colon_colon_token: Ok(
                        ColonColonToken(
                            TokenIdx(
                                8,
                            ),
                        ),
                    ),
                    children: Ok(
                        UseExprChildren::Single {
                            child: 3,
                        },
                    ),
                },
            ),
        ],
    },
}