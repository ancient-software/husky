EntityTreeSheet {
    module_path: `std::ops`,
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Trait(
                            TraitSynNodePath(
                                ItemSynNodePathId {
                                    data: ItemSynNodePathData::MajorItem(
                                        MajorItemSynNodePathData::Trait(
                                            TraitSynNodePathData {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitPath(`std::ops::Add`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            `std::ops`,
                        ),
                        ast_idx: 4,
                        ident_token: IdentToken {
                            ident: `Add`,
                            token_idx: TokenIdx(
                                8,
                            ),
                        },
                        block: DefnBlock::Trait {
                            path: TraitPath(`std::ops::Add`),
                            items: Some(
                                TraitItems {
                                    ast_idx_range: ArenaIdxRange(
                                        1..3,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Trait(
                        TraitSynNodePath(
                            ItemSynNodePathId {
                                data: ItemSynNodePathData::MajorItem(
                                    MajorItemSynNodePathData::Trait(
                                        TraitSynNodePathData {
                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                path: TraitPath(`std::ops::Add`),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
                ident: `Add`,
                visibility: Scope::PubUnder(
                    `std::ops`,
                ),
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `Add`,
                visibility: Scope::PubUnder(
                    `std::ops`,
                ),
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Trait(
                        TraitPath(`std::ops::Add`),
                    ),
                },
            },
        ],
    ),
    impl_block_syn_node_table: [],
    once_use_rules: OnceUseRules(
        [],
    ),
    use_all_rules: UseAllRules(
        [],
    ),
    errors: [],
}