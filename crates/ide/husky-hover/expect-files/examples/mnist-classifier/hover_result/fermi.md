[
    (
        TokenIdx(
            1,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Keyword(\n    Keyword::Use,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 0,
                            },
                            end: Position {
                                line: 0,
                                character: 3,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            12,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 11;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Delimiter::Box,\n        ),\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                1,\n            ),\n        ),\n        data: TokenInfoData::VecFunctorBoxPrefix,\n    },\n);\n\nvec functor box prefix\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 3,
                                character: 14,
                            },
                            end: Position {
                                line: 3,
                                character: 15,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            23,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 22;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 4,
                                character: 31,
                            },
                            end: Position {
                                line: 4,
                                character: 32,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            34,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 33;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Keyword(\n    Keyword::Modifier(\n        Mut,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 10,
                                character: 12,
                            },
                            end: Position {
                                line: 10,
                                character: 15,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            45,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 44;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Ident(\n    `others`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                4,\n            ),\n        ),\n        data: TokenInfoData::Field,\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 11,
                                character: 21,
                            },
                            end: Position {
                                line: 11,
                                character: 27,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            56,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 55;\n\ntoken_line_group_idx = 6\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Delimiter::Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 12,
                                character: 27,
                            },
                            end: Position {
                                line: 12,
                                character: 28,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            67,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 66;\n\ntoken_line_group_idx = 7\n\ntoken = TokenData::Ident(\n    `norm`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                16,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 1,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 1,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbolEntry {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        4,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                36,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `norm`,\n        pattern_symbol_idx: 1,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 13,
                                character: 15,
                            },
                            end: Position {
                                line: 13,
                                character: 19,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            78,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 77;\n\ntoken_line_group_idx = 9\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 16,
                                character: 26,
                            },
                            end: Position {
                                line: 16,
                                character: 27,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            89,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 88;\n\ntoken_line_group_idx = 10\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Delimiter::Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 17,
                                character: 33,
                            },
                            end: Position {
                                line: 17,
                                character: 34,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            100,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 99;\n\ntoken_line_group_idx = 11\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Delimiter::Box,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 18,
                                character: 39,
                            },
                            end: Position {
                                line: 18,
                                character: 40,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            111,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 110;\n\ntoken_line_group_idx = 13\n\ntoken = TokenData::Ident(\n    `f32`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            1,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::f32`, `Extern`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::f32`, `Extern`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 21,
                                character: 28,
                            },
                            end: Position {
                                line: 21,
                                character: 31,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            122,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 121;\n\ntoken_line_group_idx = 15\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::LaOrLt,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 23,
                                character: 14,
                            },
                            end: Position {
                                line: 23,
                                character: 15,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            133,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 132;\n\ntoken_line_group_idx = 16\n\ntoken = TokenData::Ident(\n    `norm`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                8,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 1,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 1,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbolEntry {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        4,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                40,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `norm`,\n        pattern_symbol_idx: 1,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 24,
                                character: 19,
                            },
                            end: Position {
                                line: 24,
                                character: 23,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            144,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 143;\n\ntoken_line_group_idx = 16\n\ntoken = TokenData::Ident(\n    `angle_change`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                13,\n            ),\n        ),\n        data: TokenInfoData::Field,\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 24,
                                character: 43,
                            },
                            end: Position {
                                line: 24,
                                character: 55,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            155,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 154;\n\ntoken_line_group_idx = 18\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Delimiter::Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 27,
                                character: 18,
                            },
                            end: Position {
                                line: 27,
                                character: 19,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            166,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 165;\n\ntoken_line_group_idx = 18\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Delimiter::Box,\n        ),\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                5,\n            ),\n        ),\n        data: TokenInfoData::VecFunctorBoxPrefix,\n    },\n);\n\nvec functor box prefix\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 29,
                                character: 16,
                            },
                            end: Position {
                                line: 29,
                                character: 17,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            177,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 176;\n\ntoken_line_group_idx = 18\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Delimiter::Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 30,
                                character: 0,
                            },
                            end: Position {
                                line: 30,
                                character: 1,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            188,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 187;\n\ntoken_line_group_idx = 19\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Delimiter::Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 31,
                                character: 55,
                            },
                            end: Position {
                                line: 31,
                                character: 56,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            199,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 198;\n\ntoken_line_group_idx = 20\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 32,
                                character: 42,
                            },
                            end: Position {
                                line: 32,
                                character: 43,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            210,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 209;\n\ntoken_line_group_idx = 21\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 34,
                                character: 28,
                            },
                            end: Position {
                                line: 34,
                                character: 29,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            221,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 220;\n\ntoken_line_group_idx = 23\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Delimiter::Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 36,
                                character: 20,
                            },
                            end: Position {
                                line: 36,
                                character: 21,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
    (
        TokenIdx(
            232,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 231;\n\ntoken_line_group_idx = 24\n\ntoken = TokenData::Ident(\n    `matches`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                21,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 2,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 2,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbolEntry {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        13,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                56,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `matches`,\n        pattern_symbol_idx: 2,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 37,
                                character: 28,
                            },
                            end: Position {
                                line: 37,
                                character: 35,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
]