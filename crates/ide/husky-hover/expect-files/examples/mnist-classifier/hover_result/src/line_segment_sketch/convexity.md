```rust
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
                            value: "\ntoken_idx = 11;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Star,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 1,
                                character: 24,
                            },
                            end: Position {
                                line: 1,
                                character: 25,
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
                            value: "\ntoken_idx = 22;\n\ntoken_line_group_idx = 3\n\ntoken = TokenData::Ident(\n    `line_segment_sketch`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::Pattern(\n            0,\n        ),\n        data: TokenInfoData::CurrentVariable {\n            current_variable_idx: 0,\n            current_variable_kind: CurrentVariableKind::SimpleParenateParameter {\n                pattern_variable_idx: 0,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Pure,\n    access_start: RegionalTokenIdx(\n        6,\n    ),\n    access_end: None,\n    data: CurrentVariableData::SimpleParenateParameter {\n        ident: `line_segment_sketch`,\n        pattern_variable_idx: 0,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 4,
                                character: 17,
                            },
                            end: Position {
                                line: 4,
                                character: 36,
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
                            value: "Other\ntoken_idx = 33;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Keyword(\n    Keyword::Stmt(\n        Let,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 5,
                                character: 4,
                            },
                            end: Position {
                                line: 5,
                                character: 7,
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
                            value: "\ntoken_idx = 44;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Ident(\n    `current_displacement`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::Pattern(\n            1,\n        ),\n        data: TokenInfoData::CurrentVariable {\n            current_variable_idx: 1,\n            current_variable_kind: CurrentVariableKind::LetVariable {\n                pattern_variable_idx: 1,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Pure,\n    access_start: RegionalTokenIdx(\n        13,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                201,\n            ),\n        ),\n    ),\n    data: CurrentVariableData::LetVariable {\n        ident: `current_displacement`,\n        pattern_variable_idx: 1,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 6,
                                character: 8,
                            },
                            end: Position {
                                line: 6,
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
            56,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 55;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Ident(\n    `displacement`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                9,\n            ),\n        ),\n        data: TokenInfoData::Method,\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 6,
                                character: 68,
                            },
                            end: Position {
                                line: 6,
                                character: 80,
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
                            value: "\ntoken_idx = 66;\n\ntoken_line_group_idx = 6\n\ntoken = TokenData::Ident(\n    `index`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                12,\n            ),\n        ),\n        data: TokenInfoData::InheritedVariable {\n            inherited_variable_idx: 1,\n            inherited_variable_kind: InheritedVariableKind::Parenate {\n                ident: `index`,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nInheritedVariableEntry {\n    modifier: Pure,\n    kind: InheritedVariableKind::Parenate {\n        ident: `index`,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 7,
                                character: 61,
                            },
                            end: Position {
                                line: 7,
                                character: 66,
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
                            value: "Other\ntoken_idx = 77;\n\ntoken_line_group_idx = 7\n\ntoken = TokenData::Keyword(\n    Keyword::Stmt(\n        Let,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 8,
                                character: 4,
                            },
                            end: Position {
                                line: 8,
                                character: 7,
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
                            value: "\ntoken_idx = 88;\n\ntoken_line_group_idx = 8\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            SynBinaryOpr::Comparison(\n                BinaryComparisonOpr::Eq,\n            ),\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 10,
                                character: 43,
                            },
                            end: Position {
                                line: 10,
                                character: 45,
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
                            value: "\ntoken_idx = 99;\n\ntoken_line_group_idx = 10\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 12,
                                character: 30,
                            },
                            end: Position {
                                line: 12,
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
            111,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 110;\n\ntoken_line_group_idx = 10\n\ntoken = TokenData::Ident(\n    `L`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                34,\n            ),\n        ),\n        data: TokenInfoData::CurrentVariable {\n            current_variable_idx: 0,\n            current_variable_kind: CurrentVariableKind::LetVariable {\n                pattern_variable_idx: 0,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Pure,\n    access_start: RegionalTokenIdx(\n        3,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                201,\n            ),\n        ),\n    ),\n    data: CurrentVariableData::LetVariable {\n        ident: `L`,\n        pattern_variable_idx: 0,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 12,
                                character: 70,
                            },
                            end: Position {
                                line: 12,
                                character: 71,
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
                            value: "\ntoken_idx = 121;\n\ntoken_line_group_idx = 11\n\ntoken = TokenData::Ident(\n    `i1`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                42,\n            ),\n        ),\n        data: TokenInfoData::CurrentVariable {\n            current_variable_idx: 6,\n            current_variable_kind: CurrentVariableKind::LoopVariable(\n                40,\n            ),\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Pure,\n    access_start: RegionalTokenIdx(\n        97,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                127,\n            ),\n        ),\n    ),\n    data: CurrentVariableData::LoopVariable {\n        ident: `i1`,\n        expr_idx: 40,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 13,
                                character: 41,
                            },
                            end: Position {
                                line: 13,
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
            133,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 132;\n\ntoken_line_group_idx = 12\n\ntoken = TokenData::Ident(\n    `line_segment_sketch`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                43,\n            ),\n        ),\n        data: TokenInfoData::InheritedVariable {\n            inherited_variable_idx: 0,\n            inherited_variable_kind: InheritedVariableKind::Parenate {\n                ident: `line_segment_sketch`,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nInheritedVariableEntry {\n    modifier: Pure,\n    kind: InheritedVariableKind::Parenate {\n        ident: `line_segment_sketch`,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 14,
                                character: 31,
                            },
                            end: Position {
                                line: 14,
                                character: 50,
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
                            value: "\ntoken_idx = 143;\n\ntoken_line_group_idx = 12\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 14,
                                character: 97,
                            },
                            end: Position {
                                line: 14,
                                character: 98,
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
                            value: "\ntoken_idx = 154;\n\ntoken_line_group_idx = 13\n\ntoken = TokenData::Ident(\n    `cross`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                53,\n            ),\n        ),\n        data: TokenInfoData::Method,\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 15,
                                character: 77,
                            },
                            end: Position {
                                line: 15,
                                character: 82,
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
                            value: "Other\ntoken_idx = 165;\n\ntoken_line_group_idx = 15\n\ntoken = TokenData::Keyword(\n    Keyword::Stmt(\n        Let,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 18,
                                character: 8,
                            },
                            end: Position {
                                line: 18,
                                character: 11,
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
                            value: "\ntoken_idx = 176;\n\ntoken_line_group_idx = 15\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Dot,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 18,
                                character: 67,
                            },
                            end: Position {
                                line: 18,
                                character: 68,
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
                            value: "\ntoken_idx = 187;\n\ntoken_line_group_idx = 16\n\ntoken = TokenData::Ident(\n    `current_interval`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                67,\n            ),\n        ),\n        data: TokenInfoData::CurrentVariable {\n            current_variable_idx: 9,\n            current_variable_kind: CurrentVariableKind::LetVariable {\n                pattern_variable_idx: 8,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Pure,\n    access_start: RegionalTokenIdx(\n        135,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                195,\n            ),\n        ),\n    ),\n    data: CurrentVariableData::LetVariable {\n        ident: `current_interval`,\n        pattern_variable_idx: 8,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 19,
                                character: 45,
                            },
                            end: Position {
                                line: 19,
                                character: 61,
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
                            value: "\ntoken_idx = 198;\n\ntoken_line_group_idx = 17\n\ntoken = TokenData::Ident(\n    `contour`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                71,\n            ),\n        ),\n        data: TokenInfoData::Field,\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 20,
                                character: 51,
                            },
                            end: Position {
                                line: 20,
                                character: 58,
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
                            value: "\ntoken_idx = 209;\n\ntoken_line_group_idx = 17\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::RightDelimiter(\n            Delimiter::Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 20,
                                character: 101,
                            },
                            end: Position {
                                line: 20,
                                character: 102,
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
                            value: "\ntoken_idx = 220;\n\ntoken_line_group_idx = 18\n\ntoken = TokenData::Ident(\n    `displacement`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                79,\n            ),\n        ),\n        data: TokenInfoData::CurrentVariable {\n            current_variable_idx: 11,\n            current_variable_kind: CurrentVariableKind::LetVariable {\n                pattern_variable_idx: 9,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Pure,\n    access_start: RegionalTokenIdx(\n        163,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                191,\n            ),\n        ),\n    ),\n    data: CurrentVariableData::LetVariable {\n        ident: `displacement`,\n        pattern_variable_idx: 9,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 21,
                                character: 81,
                            },
                            end: Position {
                                line: 21,
                                character: 93,
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
                            value: "\ntoken_idx = 231;\n\ntoken_line_group_idx = 21\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::RaOrGt,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 24,
                                character: 51,
                            },
                            end: Position {
                                line: 24,
                                character: 52,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
]
```