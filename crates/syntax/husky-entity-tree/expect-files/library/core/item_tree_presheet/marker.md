EntityTreePresheet {
    module_path: `core::marker`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId(
                                    Id {
                                        value: 36,
                                    },
                                ),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 2,
                        ident_token: IdentToken {
                            ident: `Copy`,
                            token_idx: TokenIdx(
                                7,
                            ),
                        },
                        block: DefnBlock::Trait {
                            path: TraitPath(`core::marker::Copy`),
                            items: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 36,
                                },
                            ),
                        ),
                    ),
                ),
                ident: `Copy`,
                visibility: Scope::Pub,
            },
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId(
                                    Id {
                                        value: 37,
                                    },
                                ),
                            ),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 4,
                        ident_token: IdentToken {
                            ident: `Sized`,
                            token_idx: TokenIdx(
                                18,
                            ),
                        },
                        block: DefnBlock::Trait {
                            path: TraitPath(`core::marker::Sized`),
                            items: None,
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId(
                                Id {
                                    value: 37,
                                },
                            ),
                        ),
                    ),
                ),
                ident: `Sized`,
                visibility: Scope::Pub,
            },
        ],
    },
    once_use_rules: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 1,
                use_expr_idx: 2,
                visibility: Scope::PubUnder(
                    `core::marker`,
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
        ],
    },
}