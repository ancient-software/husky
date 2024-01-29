Ok(
    TomlAstSheet {
        expr_arena: Arena {
            data: [
                TomlExpr::String(
                    "syntax-errors",
                ),
                TomlExpr::String(
                    "0.1.0",
                ),
                TomlExpr::String(
                    "example mnist classifier",
                ),
                TomlExpr::String(
                    "MIT OR Apache-2.0",
                ),
                TomlExpr::String(
                    "0.1.0",
                ),
                TomlExpr::String(
                    "0.1.0",
                ),
            ],
        },
        section_sheet: TomlSectionSheet {
            arena: Arena {
                data: [
                    TomlSection {
                        title: TomlSectionTitle(
                            [
                                Coword(
                                    Id {
                                        value: 2,
                                    },
                                ),
                            ],
                        ),
                        kind: TomlSectionKind::Normal,
                        entries: [
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    1,
                                ),
                                key: Coword(
                                    Id {
                                        value: 3,
                                    },
                                ),
                                value: Some(
                                    1,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    2,
                                ),
                                key: Coword(
                                    Id {
                                        value: 4,
                                    },
                                ),
                                value: Some(
                                    2,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    3,
                                ),
                                key: Coword(
                                    Id {
                                        value: 7,
                                    },
                                ),
                                value: Some(
                                    3,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    4,
                                ),
                                key: Coword(
                                    Id {
                                        value: 5,
                                    },
                                ),
                                value: Some(
                                    4,
                                ),
                            },
                        ],
                    },
                    TomlSection {
                        title: TomlSectionTitle(
                            [
                                Coword(
                                    Id {
                                        value: 18,
                                    },
                                ),
                            ],
                        ),
                        kind: TomlSectionKind::Normal,
                        entries: [
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    6,
                                ),
                                key: Coword(
                                    Id {
                                        value: 19,
                                    },
                                ),
                                value: Some(
                                    5,
                                ),
                            },
                            TomlSectionEntry {
                                line_group_idx: TomlLineGroupIdx(
                                    7,
                                ),
                                key: Coword(
                                    Id {
                                        value: 20,
                                    },
                                ),
                                value: Some(
                                    6,
                                ),
                            },
                        ],
                    },
                ],
            },
            errors: [],
        },
        line_groups: [
            TomlLineGroup::SectionTitle {
                title: [
                    Word(
                        "package",
                    ),
                ],
                kind: TomlSectionKind::Normal,
            },
            TomlLineGroup::KeyValue(
                Word(
                    "name",
                ),
                Some(
                    1,
                ),
            ),
            TomlLineGroup::KeyValue(
                Word(
                    "version",
                ),
                Some(
                    2,
                ),
            ),
            TomlLineGroup::KeyValue(
                Word(
                    "description",
                ),
                Some(
                    3,
                ),
            ),
            TomlLineGroup::KeyValue(
                Word(
                    "license",
                ),
                Some(
                    4,
                ),
            ),
            TomlLineGroup::SectionTitle {
                title: [
                    Word(
                        "dependencies",
                    ),
                ],
                kind: TomlSectionKind::Normal,
            },
            TomlLineGroup::KeyValue(
                Word(
                    "malamute",
                ),
                Some(
                    5,
                ),
            ),
            TomlLineGroup::KeyValue(
                Word(
                    "mnist",
                ),
                Some(
                    6,
                ),
            ),
        ],
        table: TomlTable {
            data: {
                Coword(
                    Id {
                        value: 2,
                    },
                ): Section(
                    1,
                ),
                Coword(
                    Id {
                        value: 18,
                    },
                ): Section(
                    2,
                ),
            },
        },
    },
)