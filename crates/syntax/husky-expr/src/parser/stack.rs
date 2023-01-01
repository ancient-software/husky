use super::*;

#[derive(Default, Debug)]
pub(crate) struct ExprParserStack {
    unfinished_exprs: Vec<(UnfinishedExpr, Precedence)>,
    top_expr: Option<Expr>,
    base_entity_paths: Vec<BaseEntityPath>,
}

impl ExprParserStack {
    pub(super) fn prev_unfinished_expr_precedence(&self) -> Option<Precedence> {
        self.unfinished_exprs
            .last()
            .map(|(_, precedence)| *precedence)
    }

    fn take_top_expr(&mut self) -> Option<Expr> {
        std::mem::take(&mut self.top_expr)
    }
}

impl Expr {
    pub fn base_entity_path(&self) -> BaseEntityPath {
        match self {
            Expr::Literal(_) => todo!(),
            Expr::EntityPath(_) => todo!(),
            Expr::Variable {
                token_idx: TokenIdx,
                variable_idx: VariableIdx,
            } => todo!(),
            Expr::Uncertain(_) => todo!(),
            Expr::Unrecognized(_) => BaseEntityPath::Uncertain,
            Expr::Opn { opn, opds } => match opn {
                Opn::Binary(_) => todo!(),
                Opn::Prefix(_) => todo!(),
                Opn::Suffix(suffix) => match suffix {
                    SuffixPunctuation::Incr
                    | SuffixPunctuation::Decr
                    | SuffixPunctuation::Unveil => BaseEntityPath::None,
                },
                Opn::CurlBracketed => todo!(),
                Opn::List(opr) => match opr {
                    ListOpr::NewTuple => todo!(),
                    ListOpr::NewVec => BaseEntityPath::None,
                    ListOpr::NewDict => todo!(),
                    ListOpr::FunctionCall => todo!(),
                    ListOpr::Index => todo!(),
                    ListOpr::ModuloIndex => todo!(),
                    ListOpr::StructInit => todo!(),
                    ListOpr::MethodCall { ranged_ident } => todo!(),
                },
                Opn::Field(_) => todo!(),
                Opn::Abstraction => todo!(),
                Opn::Application => todo!(),
            },
            Expr::Bracketed(_) => todo!(),
            Expr::Err(_) => todo!(),
        }
    }
}

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(super) fn number_of_oprs(&self) -> usize {
        self.stack.unfinished_exprs.len()
    }

    pub(super) fn top_expr(&self) -> Option<&Expr> {
        self.stack.top_expr.as_ref()
    }

    pub(super) fn top_base_entity_path(&self) -> Option<BaseEntityPath> {
        self.stack.base_entity_paths.last().map(|v| *v)
    }

    pub(super) fn finish_batch(&mut self) -> Option<ExprIdx> {
        core::mem::take(&mut self.stack.top_expr).map(|expr| self.sheet.alloc_expr(expr))
    }

    pub(super) fn push_expr(&mut self, expr: Expr) {
        if let Some(expr) = self.take_top_expr() {
            self.push_unfinished_expr(UnfinishedExpr::Application { function: expr });
        }
        self.stack.top_expr = Some(expr)
    }

    pub(super) fn push_unfinished_expr(&mut self, unfinished_expr: UnfinishedExpr) {
        assert!(self.stack.top_expr.is_none());
        let precedence = unfinished_expr.precedence();
        self.stack
            .unfinished_exprs
            .push((unfinished_expr, precedence))
    }

    pub(super) fn last_unfinished_expr(&self) -> Option<&UnfinishedExpr> {
        self.stack.unfinished_exprs.last().map(|(opr, _)| opr)
    }

    pub(super) fn pop_opr(&mut self) -> Option<UnfinishedExpr> {
        self.stack.unfinished_exprs.pop().map(|(opr, _)| opr)
    }

    // pub(super) fn drain_exprs(&mut self, k: usize) -> (Vec<Expr>, Vec<BaseEntityPath>) {
    //     todo!()
    //     // let len = self.stack.exprs.len();
    //     // assert_eq!(len, self.stack.base_entity_paths.len());
    //     // (
    //     //     self.stack.exprs.drain((len - k)..).collect(),
    //     //     self.stack.base_entity_paths.drain((len - k)..).collect(),
    //     // )
    // }

    fn set_top_expr(&mut self, expr: Expr) {
        assert!(self.stack.top_expr.is_none());
        let path = expr.base_entity_path();
        self.stack.top_expr = Some(expr)
    }

    pub(crate) fn accept_list_end(
        &mut self,
        ket: Bracket,
        ket_token_idx: TokenIdx,
        attr: ListEndAttr,
    ) {
        self.reduce(Precedence::ListItem);
        match self.stack.unfinished_exprs.pop().unwrap().0 {
            UnfinishedExpr::List {
                opr,
                bra,
                bra_token_idx,
                mut items,
            } => {
                if let Some(expr) = self.take_top_expr() {
                    items.push(expr)
                }
                let opds = self.sheet.alloc_expr_batch(items);
                self.set_top_expr(Expr::Opn {
                    opn: Opn::List(opr),
                    opds,
                })
            }
            _ => todo!(),
        }
        // let original_number_of_oprs = self.number_of_oprs();
        // let (start_attr, bra_token) = {
        //     loop {
        //         match self.pop_opr() {
        //             Some(opr) => match opr {
        //                 UnfinishedExpr::ListItem { .. } => (),
        //                 UnfinishedExpr::ListStart {
        //                     bra,
        //                     bra_token_idx,
        //                     attr,
        //                 } => {
        //                     if ket != bra {
        //                         return Err(ExprError::MisMatchingBracket {
        //                             bra,
        //                             bra_token_idx,
        //                             ket,
        //                             ket_token_idx,
        //                         });
        //                     };
        //                     break (attr, bra_token_idx);
        //                 }
        //                 _ => return Err(ExprError::NoMatchingBra { ket, ket_token_idx }),
        //             },
        //             None => return Err(ExprError::NoMatchingBra { ket, ket_token_idx }),
        //         }
        //     }
        // };
        // let list_len = original_number_of_oprs - self.number_of_oprs() - 1;
        // let (opds, paths) = self.drain_exprs(list_len);
        // let opds = self.sheet.alloc_expr_batch(opds, paths);
        // self.push_expr(new_list_expr(ket, start_attr, attr, opds)?);
        // Ok(())
    }

    pub(super) fn synthesize_suffix(
        &mut self,
        suffix: SuffixPunctuation,
        suffix_token_idx: TokenIdx,
    ) {
        match self.take_top_expr() {
            Some(expr) => {
                let expr = self.sheet.alloc_expr(expr);
                self.stack.top_expr = Some(Expr::Opn {
                    opn: Opn::Suffix(suffix),
                    opds: ExprIdxRange::new_single(expr),
                })
            }
            None => todo!(),
        }
        // self.synthesize_opn(suffix.into(), 1)
    }

    pub(super) fn take_top_expr(&mut self) -> Option<Expr> {
        std::mem::take(&mut self.stack.top_expr)
    }

    pub(super) fn accept_prefix_opr(
        &mut self,
        prefix: PrefixPunctuation,
        prefix_token_idx: TokenIdx,
    ) {
        match self.take_top_expr() {
            Some(_) => todo!(),
            None => self.push_unfinished_expr(UnfinishedExpr::Prefix {
                prefix,
                prefix_token_idx,
            }),
        }
    }

    pub(super) fn reduce(&mut self, next_precedence: Precedence) {
        while let Some(prev_precedence) = self.stack.prev_unfinished_expr_precedence() {
            if prev_precedence < next_precedence {
                break;
            }
            match self.stack.unfinished_exprs.pop().unwrap().0 {
                UnfinishedExpr::Binary {
                    lopd,
                    binary,
                    binary_token_idx,
                } => match self.take_top_expr() {
                    Some(ropd) => {
                        self.stack.top_expr = Some(Expr::Opn {
                            opn: Opn::Binary(binary),
                            opds: self.sheet.alloc_expr_batch([lopd, ropd]),
                        })
                    }
                    None => {
                        let lopd = self.sheet.alloc_expr(lopd);
                        self.stack.top_expr =
                            Some(Expr::Err(ExprError::NoRightOperandForBinaryOperator {
                                lopd,
                                binary,
                                binary_token_idx,
                            }))
                    }
                },
                UnfinishedExpr::Application { function } => {
                    let argument = self.take_top_expr().unwrap();
                    self.stack.top_expr = Some(Expr::Opn {
                        opn: Opn::Application,
                        opds: self.sheet.alloc_expr_batch([function, argument]),
                    })
                }
                UnfinishedExpr::Prefix {
                    prefix,
                    prefix_token_idx,
                } => match self.take_top_expr() {
                    Some(opd) => {
                        self.stack.top_expr = Some(Expr::Opn {
                            opn: Opn::Prefix(prefix),
                            opds: self.sheet.alloc_expr_batch([opd]),
                        })
                    }
                    None => {
                        self.stack.top_expr =
                            Some(Expr::Err(ExprError::NoOperandForPrefixOperator {
                                prefix,
                                prefix_token_idx,
                            }))
                    }
                },
                UnfinishedExpr::ListItem {
                    separator_token_idx,
                } => todo!(),
                UnfinishedExpr::List { .. } => todo!(),
                UnfinishedExpr::LambdaHead { inputs, start } => todo!(),
                UnfinishedExpr::Dot { dot_token_idx } => todo!(),
            }
        }
    }
}
