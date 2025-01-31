```rust
EntityTreeSheet {
    module_path: ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
    major_item_node_table: MajorEntityNodeTable {
        entries: [
            ItemNodeEntry {
                node: ItemSynNode::MajorItem(
                    MajorItemSynNode {
                        syn_node_path: MajorItemSynNodePath::Form(
                            MajorFormSynNodePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Ritchie(
                                Fn,
                            )`, (0)),
                        ),
                        visibility: Scope::Pub,
                        ast_idx: 22,
                        ident_token: IdentToken {
                            ident: `is_convex`,
                            token_idx: TokenIdx(
                                21,
                            ),
                        },
                        block: DefnBlock::Form {
                            path: MajorFormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Ritchie(
                                Fn,
                            )`),
                            body: Some(
                                FormBody {
                                    ast_idx_range: ArenaIdxRange(
                                        14..19,
                                    ),
                                },
                            ),
                        },
                    },
                ),
                syn_node_path: ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Form(
                        MajorFormSynNodePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Ritchie(
                            Fn,
                        )`, (0)),
                    ),
                ),
                ident: `is_convex`,
                visibility: Scope::Pub,
            },
        ],
    },
    item_symbol_table: EntitySymbolTable(
        [
            EntitySymbolEntry {
                ident: `is_convex`,
                visible_scope: Scope::Pub,
                symbol: EntitySymbol::MajorItem {
                    major_item_path: MajorItemPath::Form(
                        MajorFormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Ritchie(
                            Fn,
                        )`),
                    ),
                },
            },
            EntitySymbolEntry {
                ident: `concave_component`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::line_segment_sketch::concave_component),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::line_segment_sketch::concave_component`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `convex_component`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::line_segment_sketch::convex_component),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::line_segment_sketch::convex_component`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `convexity`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::line_segment_sketch::convexity),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `line_segment`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::line_segment_sketch::line_segment),
                        },
                        path: PrincipalEntityPath::Module(
                            ModulePath(`mnist_classifier::line_segment_sketch::line_segment`),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegmentStroke`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegmentSketch`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `go_right`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::line_segment_sketch::go_right`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::line_segment_sketch::go_right`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `go_left`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::line_segment_sketch::go_left`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::line_segment_sketch::go_left`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `extend_end`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `extend_start`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_line_segments`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ConcaveComponent`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 163,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_concave_components`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Form(
                                        MajorFormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                            Fn,
                                        )`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Form(
                                        MajorFormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                            Fn,
                                        )`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 163,
                                use_expr_idx: 0,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegment`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::Pub,
                                ast_idx: 165,
                                use_expr_idx: 4,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `LineSegment`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::line_segment_sketch`),
                                ),
                                ast_idx: 168,
                                use_expr_idx: 12,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RawContour`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::line_segment_sketch`),
                                ),
                                ast_idx: 167,
                                use_expr_idx: 9,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Point2d`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::line_segment_sketch`),
                                ),
                                ast_idx: 166,
                                use_expr_idx: 6,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RelativePoint2d`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::line_segment_sketch`),
                                ),
                                ast_idx: 166,
                                use_expr_idx: 6,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Vector2d`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::line_segment_sketch`),
                                ),
                                ast_idx: 166,
                                use_expr_idx: 6,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ClosedRange`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::line_segment_sketch`),
                                ),
                                ast_idx: 166,
                                use_expr_idx: 6,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `BoundingBox`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::line_segment_sketch`),
                                ),
                                ast_idx: 166,
                                use_expr_idx: 6,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RelativeBoundingBox`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::Use(
                            UseSymbol {
                                original_symbol: EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                },
                                path: PrincipalEntityPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                    ),
                                ),
                                visibility: Scope::PubUnder(
                                    ModulePath(`mnist_classifier::line_segment_sketch`),
                                ),
                                ast_idx: 166,
                                use_expr_idx: 6,
                            },
                        ),
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 19,
                        use_expr_idx: 0,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RawContour`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 20,
                        use_expr_idx: 3,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `find_raw_contours`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Form(
                                MajorFormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                                    Fn,
                                )`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 20,
                        use_expr_idx: 3,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Point2d`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 21,
                        use_expr_idx: 6,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RelativePoint2d`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 21,
                        use_expr_idx: 6,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `Vector2d`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 21,
                        use_expr_idx: 6,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `ClosedRange`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 21,
                        use_expr_idx: 6,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `BoundingBox`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 21,
                        use_expr_idx: 6,
                    },
                ),
            },
            EntitySymbolEntry {
                ident: `RelativeBoundingBox`,
                visible_scope: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                symbol: EntitySymbol::Use(
                    UseSymbol {
                        original_symbol: EntitySymbol::MajorItem {
                            major_item_path: MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            ),
                        },
                        path: PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            ),
                        ),
                        visibility: Scope::PubUnder(
                            ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                        ),
                        ast_idx: 21,
                        use_expr_idx: 6,
                    },
                ),
            },
        ],
    ),
    impl_block_syn_node_table: [],
    once_use_rules: OnceUseRules(
        [
            OnceUseRule {
                ast_idx: 19,
                use_expr_idx: 2,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
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
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: ModulePath(`mnist_classifier`),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 20,
                use_expr_idx: 5,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
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
                        4..5,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: ModulePath(`mnist_classifier`),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 21,
                use_expr_idx: 8,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::CrateRoot(
                        CrateToken {
                            token_idx: TokenIdx(
                                14,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        7..8,
                    ),
                },
                parent: None,
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::CrateRoot {
                            root_module_path: ModulePath(`mnist_classifier`),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 19,
                use_expr_idx: 1,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `line_segment_sketch`,
                            token_idx: TokenIdx(
                                4,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        0..1,
                    ),
                },
                parent: Some(
                    (
                        MajorEntityPath::Module(
                            ModulePath(`mnist_classifier`),
                        ),
                        EntitySymbol::CrateRoot {
                            root_module_path: ModulePath(`mnist_classifier`),
                        },
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::line_segment_sketch),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 20,
                use_expr_idx: 4,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `raw_contour`,
                            token_idx: TokenIdx(
                                10,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        3..4,
                    ),
                },
                parent: Some(
                    (
                        MajorEntityPath::Module(
                            ModulePath(`mnist_classifier`),
                        ),
                        EntitySymbol::CrateRoot {
                            root_module_path: ModulePath(`mnist_classifier`),
                        },
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::raw_contour),
                        },
                    ),
                },
            },
            OnceUseRule {
                ast_idx: 21,
                use_expr_idx: 7,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                variant: OnceUseRuleVariant::Parent {
                    parent_name_token: PathNameToken::Ident(
                        IdentToken {
                            ident: `geom2d`,
                            token_idx: TokenIdx(
                                16,
                            ),
                        },
                    ),
                    children: ArenaIdxRange(
                        6..7,
                    ),
                },
                parent: Some(
                    (
                        MajorEntityPath::Module(
                            ModulePath(`mnist_classifier`),
                        ),
                        EntitySymbol::CrateRoot {
                            root_module_path: ModulePath(`mnist_classifier`),
                        },
                    ),
                ),
                state: UseOneRuleState::Resolved {
                    original_symbol: Some(
                        EntitySymbol::Submodule {
                            submodule_item_path: SubmoduleItemPath(`mnist_classifier::geom2d),
                        },
                    ),
                },
            },
        ],
    ),
    use_all_rules: UseAllRules(
        [
            UseAllRule {
                parent_module_path: ModulePath(`mnist_classifier::line_segment_sketch`),
                is_same_crate: true,
                ast_idx: 19,
                use_expr_idx: 0,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                progress: Ok(
                    22,
                ),
            },
            UseAllRule {
                parent_module_path: ModulePath(`mnist_classifier::raw_contour`),
                is_same_crate: true,
                ast_idx: 20,
                use_expr_idx: 3,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                progress: Ok(
                    72,
                ),
            },
            UseAllRule {
                parent_module_path: ModulePath(`mnist_classifier::geom2d`),
                is_same_crate: true,
                ast_idx: 21,
                use_expr_idx: 6,
                visibility: Scope::PubUnder(
                    ModulePath(`mnist_classifier::line_segment_sketch::convexity`),
                ),
                progress: Ok(
                    6,
                ),
            },
        ],
    ),
    errors: [],
}
```