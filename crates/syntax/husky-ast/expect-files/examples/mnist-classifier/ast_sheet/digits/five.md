AstSheet {
    ast_arena: Arena {
        data: [
            Ast::BasicStmtOrBranch {
                token_group_idx: TokenGroupIdx(
                    2,
                ),
                body: None,
            },
            Ast::Use {
                token_group_idx: TokenGroupIdx(
                    0,
                ),
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Protected,
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits::five`,
                    ),
                },
                state_after_visibility_expr: None,
            },
            Ast::Identifiable {
                token_group_idx: TokenGroupIdx(
                    1,
                ),
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::PubUnder {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                5,
                            ),
                        },
                        lpar: LparToken(
                            TokenIdx(
                                6,
                            ),
                        ),
                        visibility: Super(
                            SuperToken {
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        ),
                        rpar: RparToken(
                            TokenIdx(
                                8,
                            ),
                        ),
                    },
                    visibility: Scope::PubUnder(
                        `mnist_classifier::digits`,
                    ),
                },
                item_kind: MajorItem {
                    module_item_kind: Fugitive(
                        Val,
                    ),
                    connection: Connected,
                },
                ident_token: IdentToken {
                    ident: `is_five`,
                    token_idx: TokenIdx(
                        10,
                    ),
                },
                is_generic: false,
                saved_stream_state: TokenStreamState {
                    next_token_idx: TokenIdx(
                        11,
                    ),
                    drained: false,
                },
                block: DefnBlock::Fugitive {
                    path: FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                    body: Some(
                        FugitiveBody {
                            ast_idx_range: ArenaIdxRange(
                                1..2,
                            ),
                        },
                    ),
                },
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        2..4,
    ),
    siblings: [
        ArenaIdxRange(
            1..1,
        ),
        ArenaIdxRange(
            1..2,
        ),
        ArenaIdxRange(
            2..4,
        ),
    ],
}