use super::*;
use smallvec::{smallvec, SmallVec};
use visored_global_dispatch::dispatch::separator::VdSeparatorGlobalDispatch;
use visored_syn_expr::expr::VdSynExprIdxRange;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSemSeparator {
    Base(LxTokenIdxRange, VdBaseSeparator),
    Composite(VdSemExprIdx, VdSeparatorClass),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdSemSeparatedListDispatch {
    NatAdd,
    NatMul,
    Eq,
    In,
}
impl VdSemSeparatedListDispatch {
    fn expr_ty(&self) -> VdZfcType {
        todo!()
    }
}

impl<'db> VdSemExprBuilder<'db> {
    pub(super) fn build_separated_list(
        &mut self,
        separator_class: VdSeparatorClass,
        items: VdSynExprIdxRange,
        separators: &[VdSynSeparator],
    ) -> VdSemExprEntry {
        let (fst, others) = match separator_class {
            VdSeparatorClass::Space => self.build_space_separated_list_aux(items, separators),
            _ => self.build_non_space_separated_list_aux(items, separators),
        };
        if others.is_empty() {
            return fst;
        }
        let dispatch = self.calc_separated_list_dispatch(&fst, &others);
        let items = self.alloc_exprs(
            [fst]
                .into_iter()
                .chain(others.into_iter().map(|(_, entry)| entry)),
        );
        let ty = dispatch.expr_ty();
        let data = VdSemExprData::SeparatedList {
            separator_class,
            items,
            dispatch,
        };
        VdSemExprEntry::new(data, ty)
    }

    fn build_space_separated_list_aux(
        &mut self,
        items: VdSynExprIdxRange,
        separators: &[VdSynSeparator],
    ) -> (
        VdSemExprEntry,
        SmallVec<[(VdSemSeparator, VdSemExprEntry); 4]>,
    ) {
        todo!()
    }

    fn build_non_space_separated_list_aux(
        &mut self,
        items: VdSynExprIdxRange,
        separators: &[VdSynSeparator],
    ) -> (
        VdSemExprEntry,
        SmallVec<[(VdSemSeparator, VdSemExprEntry); 4]>,
    ) {
        let mut item_iter = items.into_iter();
        let fst = self.build_expr_entry(item_iter.next().unwrap());
        let others = separators
            .iter()
            .copied()
            .zip(item_iter)
            .map(|(sep, item)| (sep.to_vd_sem(self), self.build_expr_entry(item)))
            .collect();
        (fst, others)
    }

    fn calc_separated_list_dispatch(
        &mut self,
        fst: &VdSemExprEntry,
        others: &[(VdSemSeparator, VdSemExprEntry)],
    ) -> VdSemSeparatedListDispatch {
        let mut prev_item_ty = fst.ty();
        let (separator, ref expr) = others[0];
        let mut dispatch =
            self.calc_separated_list_dispatch_step(prev_item_ty, separator, expr.ty());
        for &(separator, ref expr) in &others[1..] {
            // TODO: should we check compatibility?
            dispatch = self.calc_separated_list_dispatch_step(prev_item_ty, separator, expr.ty());
        }
        todo!()
    }

    fn calc_separated_list_dispatch_step(
        &mut self,
        prev_item_ty: VdZfcType,
        separator: VdSemSeparator,
        next_item_ty: VdZfcType,
    ) -> &VdSeparatorGlobalDispatch {
        let base_separator = match separator {
            VdSemSeparator::Base(_, base_separator) => base_separator,
            VdSemSeparator::Composite(_, _) => todo!(),
        };
        if let Some(default_dispatch) = self
            .default_global_dispatch_table()
            .base_separator_default_dispatch(base_separator, prev_item_ty, next_item_ty)
        {
            return default_dispatch;
        }
        use salsa::DebugWithDb;
        todo!(
            "no default dispatch for prev_item_ty = {:?}, separator = {:?}, next_item_ty = {:?}",
            prev_item_ty.debug(self.db()),
            separator,
            next_item_ty.debug(self.db())
        )
    }

    fn infer_separated_list_ty(&mut self, dispatch: VdSemSeparatedListDispatch) -> VdZfcType {
        match dispatch {
            VdSemSeparatedListDispatch::NatAdd => self.zfc_ty_menu().natural_number_ty(),
            VdSemSeparatedListDispatch::NatMul => todo!(),
            VdSemSeparatedListDispatch::Eq | VdSemSeparatedListDispatch::In => {
                self.zfc_ty_menu().natural_number_ty()
            }
        }
    }
}
