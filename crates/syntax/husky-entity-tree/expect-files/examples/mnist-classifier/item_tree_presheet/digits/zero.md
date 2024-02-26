EntityTreePresheet {
    module_path: `mnist_classifier::digits::zero`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(`mnist_classifier::digits::zero::open_one_match`, `Val`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                        ast_idx: 26,
                        ident_token: IdentToken {
                            ident: `open_one_match`,
                            token_idx: TokenIdx(
                                6,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                            body: Some(
                                FugitiveBody {
                                    ast_idx_range: ArenaIdxRange(
                                        1..2,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(`mnist_classifier::digits::zero::open_one_match`, `Val`, (0)),
                    ),
                ),
                ident: `open_one_match`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::digits::zero`,
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::digits::zero`,
                        ),
                        ast_idx: 27,
                        ident_token: IdentToken {
                            ident: `almost_closed`,
                            token_idx: TokenIdx(
                                19,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FugitiveBody {
                                    ast_idx_range: ArenaIdxRange(
                                        2..4,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `almost_closed`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::digits::zero`,
                ),
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Fugitive(
                            FugitiveSynNodePath(`mnist_classifier::digits::zero::is_zero`, `Val`, (0)),
                        ),
                        visibility: Scope::PubUnder(
                            `mnist_classifier::digits`,
                        ),
                        ast_idx: 28,
                        ident_token: IdentToken {
                            ident: `is_zero`,
                            token_idx: TokenIdx(
                                50,
                            ),
                        },
                        block: DefnBlock::Fugitive {
                            path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                            body: Some(
                                FugitiveBody {
                                    ast_idx_range: ArenaIdxRange(
                                        12..25,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Fugitive(
                        FugitiveSynNodePath(`mnist_classifier::digits::zero::is_zero`, `Val`, (0)),
                    ),
                ),
                ident: `is_zero`,
                visibility: Scope::PubUnder(
                    `mnist_classifier::digits`,
                ),
            },
        ],
    },
    once_use_rules: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 25,
                use_expr_idx: 2,
                visibility: Scope::PubUnder(
                    `mnist_classifier::digits::zero`,
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Super(
                        SuperToken {
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
                    parent_name_token: PathNameToken::Super(
                        SuperToken {
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
        ],
    },
}