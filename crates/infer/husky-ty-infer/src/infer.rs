use crate::*;
use husky_expr_syntax::{RawAtom, RawExprIdx, RawExprRange};
use husky_primitive_literal_syntax::RawLiteralData;
use husky_print_utils::p;
use husky_symbol_syntax::SymbolKind;
use husky_term::Ty;

impl<'a> TyInferContext<'a> {
    pub(crate) fn infer(&mut self) -> TyInferResult<Ty> {
        match self.normalized_expr() {
            NormalizedExpr::Atom(atom) => self.infer_atom(atom),
            NormalizedExpr::Opn { opn_kind, opds } => self.infer_opr_opn(opds),
        }
    }

    fn infer_subexpr(&mut self, subexpr: RawExprIdx) -> TyInferResult<Ty> {
        self.subexpr_context(subexpr).infer()
    }

    fn infer_atom(&self, atom: &RawAtom) -> TyInferResult<Ty> {
        match atom {
            RawAtom::Literal(literal) => Ok(self.infer_literal(literal)),
            RawAtom::Symbol(symbol) => match symbol.kind {
                SymbolKind::EntityPath(_) => todo!(),
                SymbolKind::LocalVariable { init_range } => todo!(),
                SymbolKind::FrameVariable { init_range } => todo!(),
                SymbolKind::Unrecognized => Err(TyInferError::IdentUnrecognized),
                SymbolKind::ThisValue => todo!(),
                SymbolKind::ThisMethod => todo!(),
                SymbolKind::ThisField => todo!(),
            },
            RawAtom::Uncertain => todo!(),
        }
    }

    fn infer_opr_opn(&mut self, opds: RawExprRange) -> TyInferResult<Ty> {
        let this_ty = self.infer_subexpr(opds.start);
        p!(this_ty);
        todo!()
    }

    fn infer_literal(&self, literal: &RawLiteralData) -> Ty {
        let term_menu = self.term_menu();
        match literal {
            RawLiteralData::Void => todo!(),
            RawLiteralData::Integer(_) => term_menu.i32(),
            RawLiteralData::I32(_) => todo!(),
            RawLiteralData::I64(_) => todo!(),
            RawLiteralData::Float(_) => todo!(),
            RawLiteralData::F32(_) => todo!(),
            RawLiteralData::F64(_) => todo!(),
            RawLiteralData::Bits(_) => todo!(),
            RawLiteralData::B32(_) => todo!(),
            RawLiteralData::B64(_) => todo!(),
            RawLiteralData::Bool(_) => todo!(),
        }
    }
}
