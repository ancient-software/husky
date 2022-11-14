use husky_opn_syntax::{BinaryOpr, BinaryPureClosedOpr};

use super::*;

impl<'a, 'b, 'c> AtomParser<'a, 'b, 'c> {
    pub(super) fn handle_special(
        &mut self,
        special: SpecialToken,
        token: &Token,
    ) -> AtomResult<()> {
        let text_start = token.range.start;
        match special {
            SpecialToken::BinaryOpr(BinaryOpr::ScopeResolution) => err!(
                "unexpected double colon, maybe the identifier before is not recognized as entity_route correctly",
                self.token_stream.text_range(text_start)
            )?,
            SpecialToken::DoubleVertical => self.stack.push(HuskyAtom::new(
                self.token_stream.text_range(text_start),
                if !self.stack.is_concave() {
                     BinaryOpr::ShortcuitLogic(BinaryShortcuitLogicOpr::Or).into()
                } else {
                    HuskyAtomVariant::LambdaHead(Vec::new())
                },
            )),
            SpecialToken::Vertical => {
                if self.stack.is_concave() {
                    let lambda_head = self.lambda_head()?;
                    self.stack.push(HuskyAtom::new(
                        self.token_stream.text_range(text_start),
                        HuskyAtomVariant::LambdaHead(lambda_head),
                    ))
                } else {
                    self.stack.push(HuskyAtom::new(
                        self.token_stream.text_range(text_start),
                        BinaryOpr::PureClosed(BinaryPureClosedOpr::BitOr).into(),
                    ))
                }
            }
            SpecialToken::Ambersand => self.stack.push(HuskyAtom::new(
                self.token_stream.text_range(text_start),
                if self.stack.is_concave() {
                    PrefixOpr::Shared.into()
                } else {
                    BinaryOpr::PureClosed(BinaryPureClosedOpr::BitAnd).into()
                },
            )),
            SpecialToken::Exclamation => self.stack.push(HuskyAtom::new(
                self.token_stream.text_range(text_start),
                PrefixOpr::Not.into(),
            )),
            SpecialToken::Bra(bra) => Ok(self
                .stack
                .start_list(bra, self.token_stream.text_range(text_start))),
            SpecialToken::Ket(Bracket::Par) => {
                if deprecated_try_eat!(self, SpecialToken::BinaryOpr(BinaryOpr::Curry)) {
                    let output = deprecated_get!(self, ty?);
                    self.stack.make_func_type(
                        self.atom_context,
                        output,
                        self.token_stream.text_range(text_start),
                    )
                } else {
                    self.stack.end_list_or_make_type(
                        Bracket::Par,
                        ListEndAttr::None,
                        self.token_stream.text_range(text_start),
                        self.atom_context,
                    )
                }
            }
            SpecialToken::Ket(ket) => self.stack.end_list_or_make_type(
                ket,
                ListEndAttr::None,
                self.token_stream.text_range(text_start),
                self.atom_context,
            ),
            SpecialToken::RAngle => {
                if let Some(next_token) = self.token_stream.peek() {
                    if next_token.kind == SpecialToken::RAngle.into() {
                        if text_start.j()+1== next_token.range.start.j() {
                            self.token_stream.next();
                            self.stack.push(HuskyAtom::new(
                                self.token_stream.text_range(text_start),
                                BinaryOpr::PureClosed(BinaryPureClosedOpr::Shr).into(),
                            ))?;
                            return Ok(())
                        }
                    }
                }
                self.stack.push(HuskyAtom::new(
                    self.token_stream.text_range(text_start),
                    BinaryOpr::Comparison(BinaryComparisonOpr::Greater).into(),
                ))
            }
            SpecialToken::BinaryOpr(BinaryOpr::PureClosed(BinaryPureClosedOpr::Sub)) => {
                self.stack.push(HuskyAtom::new(
                    self.token_stream.text_range(text_start),
                    BinaryOpr::PureClosed(BinaryPureClosedOpr::Sub).into(),
                ))
            }
            SpecialToken::Minus =>{
                self.stack.push(HuskyAtom::new(
                    self.token_stream.text_range(text_start),
                    PrefixOpr::Minus.into(),
                ))
            }
            SpecialToken::FieldAccess => {
                if self
                .token_stream.is_next_ident() {
                    let field_ident_token = self.token_stream.next().unwrap();
                    let is_lpar_or_langle_next = match self.token_stream.peek_next_bra() {
                        Some(Bracket::Par) | Some(Bracket::Angle) => true,
                        _ => false,
                    };
                    let semantic_token_kind = if is_lpar_or_langle_next {
                        SemanticTokenKind::Method
                    } else {
                        SemanticTokenKind::Field
                    };
                    let ranged_ident = identify_token!(self, field_ident_token, semantic_token_kind);
                    let atom_variant = if is_lpar_or_langle_next {
                        if let Some(generic_arguments) = deprecated_try_get!(self, angled_generics?) {
                            match self.token_stream.next() {
                                Some(token) => match token.kind {
                                    TokenKind::Special(SpecialToken::Bra(Bracket::Par)) => {
                                        self.token_stream.text_range(text_start);
                                    }
                                    _ => todo!(),
                                },
                                None => todo!(),
                            }
                            HuskyAtomVariant::ListStart(
                                Bracket::Par,
                                ListStartAttr::MethodAttach {
                                    ranged_ident,
                                    generic_arguments,
                                },
                            )
                        } else {
                            HuskyAtomVariant::FieldAccess(Some(ranged_ident))
                        }
                    } else {
                        HuskyAtomVariant::FieldAccess(Some(ranged_ident))
                    };
                    self.stack.push(HuskyAtom::new(
                        self.token_stream.text_range(text_start), atom_variant
                    ))
                } else {
                    self.stack.push(HuskyAtom::new(
                        self.token_stream.text_range(text_start),
                        HuskyAtomVariant::FieldAccess(None),
                    ))
                }
            }
            SpecialToken::QuestionMark => match self.stack.convexity() {
                Convexity::Convex => {
                    self.stack.push(HuskyAtom::new(
                        self.token_stream.text_range(text_start),
                        RawSuffixOpr::Unveil.into(),
                    ))
                },
                Convexity::Concave => todo!(),
                Convexity::Any => todo!(),
            },
            _ => {
                self.token_stream.text_range(text_start);
                self.stack.push(token.into())
            }
        }
    }
}
