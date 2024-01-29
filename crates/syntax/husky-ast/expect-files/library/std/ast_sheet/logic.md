AstSheet {
    ast_arena: Arena {
        data: [
            Ast::Use {
                token_group_idx: TokenGroupIdx(
                    0,
                ),
                visibility_expr: VisibilityExpr {
                    data: VisibilityExprData::Pub {
                        pub_token: PubToken {
                            token_idx: TokenIdx(
                                1,
                            ),
                        },
                    },
                    visibility: Scope::Pub,
                },
                state_after_visibility_expr: Some(
                    TokenStreamState {
                        next_token_idx: TokenIdx(
                            2,
                        ),
                        drained: false,
                    },
                ),
            },
        ],
    },
    top_level_asts: ArenaIdxRange(
        1..2,
    ),
    siblings: [
        ArenaIdxRange(
            1..2,
        ),
    ],
}