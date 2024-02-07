[
    (
        FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
        None,
    ),
    (
        FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
        Some(
            ValReprExpansion {
                hir_lazy_variable_val_repr_map: ArenaMap {
                    data: [
                        Some(
                            ValRepr(
                                Id {
                                    value: 377,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 400,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 410,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 440,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 448,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 454,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 456,
                                },
                            ),
                        ),
                        Some(
                            ValRepr(
                                Id {
                                    value: 462,
                                },
                            ),
                        ),
                    ],
                },
                hir_lazy_expr_val_repr_map: [
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 2,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 12,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: Some(
                                            1,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 372,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 3,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Literal(
                            Literal::I32(
                                1,
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 4,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 373,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 374,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 5,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 375,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 370,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 7,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 375,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 370,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 1,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 375,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        1.5,
                                    ),
                                    text: "1.5f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 9,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 375,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 377,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 379,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 10,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 381,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 370,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 12,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 381,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                TermUSizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 13,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 381,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 383,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 384,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 14,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 387,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 13,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: Some(
                                            1,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 389,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 16,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 387,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                1,
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 17,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 387,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Eq,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 390,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 391,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 18,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 370,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 20,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                TermUSizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 21,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 394,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 395,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 22,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 396,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 23,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 397,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 24,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodFn`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 398,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 25,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodFn`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 398,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 5,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        5.5,
                                    ),
                                    text: "5.5f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 27,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 400,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 402,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 28,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 404,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId {
                                    data: ItemPathData::TypeVariant(
                                        TypeVariantPathData {
                                            parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                            ident: `Yes`,
                                            index: U8(
                                                0,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 29,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::VecConstructor {
                                    element_ty: LinType::Ritchie(
                                        LinkageRitchieType {
                                            parameters: [
                                                LinkageRitchieParameter {
                                                    contract: Pure,
                                                    parameter_ty: PathLeading(
                                                        LinTypePathLeading(
                                                            Id {
                                                                value: 3,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ],
                                            return_ty: LinType::PathLeading(
                                                LinTypePathLeading {
                                                    ty_path: TypePath(`core::option::Option`, `Enum`),
                                                    template_arguments: [
                                                        LinTemplateArgument::Type(
                                                            LinType::PathLeading(
                                                                LinTypePathLeading {
                                                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                                                    template_arguments: [],
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                },
                                            ),
                                        },
                                    ),
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 32,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFn {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 33,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFn {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 9,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 410,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 36,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFn {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 9,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 410,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 38,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFn {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 9,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 410,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 40,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::I32(
                                5,
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 41,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorGn {
                                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [
                                            (
                                                Type(
                                                    Type {
                                                        attrs: HirTemplateSymbolAttrs {
                                                            class: Comptime,
                                                        },
                                                        variance: None,
                                                        disambiguator: 0,
                                                    },
                                                ),
                                                Explicit(
                                                    Type(
                                                        PathLeading(
                                                            LinTypePathLeading(
                                                                Id {
                                                                    value: 1,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Variadic(
                                [
                                    ValRepr(
                                        Id {
                                            value: 411,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 412,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 413,
                                        },
                                    ),
                                ],
                            ),
                            Keyed(
                                Some(
                                    ValRepr(
                                        Id {
                                            value: 414,
                                        },
                                    ),
                                ),
                            ),
                            RuntimeConstants(
                                [
                                    ValRuntimeConstant(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                ],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 42,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocFn {
                                    path: TraitForTypeItemPath(
                                        ItemPathId {
                                            data: ItemPathData::AssocItem(
                                                AssocItemPathData::TraitForTypeItem(
                                                    TraitForTypeItemPathData {
                                                        impl_block: TraitForTypeImplBlock {
                                                            data: TraitForTypeImplBlockPathData {
                                                                module_path: `malamute`,
                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                ty_sketch: TypeSketch::Path(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                        ident: `unveil`,
                                                        item_kind: AssocFunctionFn,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 415,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 43,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFn {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 9,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 416,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 410,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 45,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 416,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        3.0,
                                    ),
                                    text: "3.0f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 46,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 416,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Less,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 418,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 419,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 47,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 421,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 49,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 421,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 423,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 50,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 421,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                TermUSizeLiteral {
                                    value: 1,
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 51,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 421,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 424,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 425,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 52,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 54,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 430,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 55,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                TermUSizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 56,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 431,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 432,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 57,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 14,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 59,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::StructField {
                                    self_ty: LinTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                        template_arguments: [],
                                    },
                                    field: Props {
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 45,
                                                },
                                            ),
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 436,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 60,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::USize(
                                TermUSizeLiteral {
                                    value: 0,
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 61,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 437,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 438,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 62,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 437,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 438,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 14,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 440,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 64,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 441,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 65,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodFn`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 442,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 66,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Index,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 437,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 438,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 14,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Unwrap,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 440,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 68,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::bounding_box`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 444,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 69,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodFn`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 445,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 70,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 443,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 446,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 71,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 303,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 73,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodFn`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 449,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 74,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorVal {
                                    path: FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [],
                        source: ValReprSource::ValItem(
                            FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                        ),
                        caching_class: ValItem,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 303,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 76,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MethodFn {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodFn`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 451,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 77,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 450,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 452,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 78,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 443,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 446,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 15,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Sub,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 450,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 452,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 16,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Div,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 448,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 454,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 81,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Closed(
                                Div,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 448,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 454,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 17,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Literal(
                            Literal::F32(
                                TermF32Literal {
                                    value: OrderedFloat(
                                        0.4,
                                    ),
                                    text: "0.4f32",
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 83,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Binary(
                            Comparison(
                                Greater,
                            ),
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 456,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 458,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 84,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MajorFn {
                                    path: FugitivePath(`mnist_classifier::fermi::fermi_match`, `FunctionFn`),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: None,
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 20,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 408,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: LetVariable {
                                stmt: 9,
                            },
                        },
                        caching_class: Variable,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 460,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::MemoizedField {
                                    path: AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::fermi::FermiMatchResult(0)::norm`, `MemoizedField`),
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 410,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 86,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 460,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId {
                                    data: ItemPathData::TypeVariant(
                                        TypeVariantPathData {
                                            parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                            ident: `Yes`,
                                            index: U8(
                                                0,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 87,
                            },
                        },
                        caching_class: Expr,
                    },
                ],
                hir_lazy_stmt_val_repr_map: [
                    ValRepr {
                        val_domain_repr: ConditionSatisfied(
                            ValRepr(
                                Id {
                                    value: 375,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 380,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 378,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 2,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 381,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 386,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 382,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 3,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 387,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 392,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 388,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 4,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 393,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 403,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 401,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 6,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 404,
                                },
                            ),
                        ),
                        opn: ValOpn::Return,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 405,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 7,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 375,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 381,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 387,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 393,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 404,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 406,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 8,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 416,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 420,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 417,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 11,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 421,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 427,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 422,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 12,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 434,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 429,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 13,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 459,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 457,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 18,
                            },
                        },
                        caching_class: Stmt,
                    },
                ],
                root_hir_lazy_stmt_val_reprs: [
                    ValRepr {
                        val_domain_repr: Omni,
                        opn: ValOpn::Branches,
                        arguments: [
                            Branch {
                                condition: Some(
                                    ValRepr(
                                        Id {
                                            value: 375,
                                        },
                                    ),
                                ),
                                stmts: [
                                    ValRepr(
                                        Id {
                                            value: 381,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 387,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 393,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 404,
                                        },
                                    ),
                                    ValRepr(
                                        Id {
                                            value: 406,
                                        },
                                    ),
                                ],
                            },
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 8,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 407,
                                },
                            ),
                        ),
                        opn: ValOpn::Linkage(
                            Linkage {
                                data: LinkageData::UnveilAssocFn {
                                    path: TraitForTypeItemPath(
                                        ItemPathId {
                                            data: ItemPathData::AssocItem(
                                                AssocItemPathData::TraitForTypeItem(
                                                    TraitForTypeItemPathData {
                                                        impl_block: TraitForTypeImplBlock {
                                                            data: TraitForTypeImplBlockPathData {
                                                                module_path: `malamute`,
                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                ty_sketch: TypeSketch::Path(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                        ident: `unveil`,
                                                        item_kind: AssocFunctionFn,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                    instantiation: LinInstantiation {
                                        symbol_resolutions: [],
                                        separator: Some(
                                            0,
                                        ),
                                    },
                                },
                            },
                        ),
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 415,
                                    },
                                ),
                            ),
                            RuntimeConstants(
                                [],
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 43,
                            },
                        },
                        caching_class: Expr,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 416,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 420,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 417,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 11,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 421,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 427,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 422,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 12,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 428,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 434,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 429,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 13,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 435,
                                },
                            ),
                        ),
                        opn: ValOpn::Require,
                        arguments: [
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 459,
                                    },
                                ),
                            ),
                            Ordinary(
                                ValRepr(
                                    Id {
                                        value: 457,
                                    },
                                ),
                            ),
                        ],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Stmt {
                                stmt: 18,
                            },
                        },
                        caching_class: Stmt,
                    },
                    ValRepr {
                        val_domain_repr: StmtNotReturned(
                            ValRepr(
                                Id {
                                    value: 460,
                                },
                            ),
                        ),
                        opn: ValOpn::TypeVariant(
                            TypeVariantPath(
                                ItemPathId {
                                    data: ItemPathData::TypeVariant(
                                        TypeVariantPathData {
                                            parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                            ident: `Yes`,
                                            index: U8(
                                                0,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                        arguments: [],
                        source: ValReprSource::Expansion {
                            parent_val_repr: ValRepr {
                                val_domain_repr: Omni,
                                opn: ValOpn::ValItemLazilyDefined(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                arguments: [],
                                source: ValReprSource::ValItem(
                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                ),
                                caching_class: ValItem,
                            },
                            source: Expr {
                                expr: 87,
                            },
                        },
                        caching_class: Expr,
                    },
                ],
            },
        ),
    ),
]