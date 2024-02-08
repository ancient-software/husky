use husky_vm::VmirData;

use crate::*;

impl<'a> VmirBlockBuilder<'a> {
    pub(super) fn compile_xml_expr(&mut self, expr: Arc<HtmlExpr>) {
        match expr.variant {
            HtmlExprVariant::Value(ref value_expr) => {
                self.compile_eager_expr(value_expr, self.sheet.variable_stack.next_stack_idx());
                self.push_instruction(Vmir::new(
                    VmirData::NewHtmlFromValue {
                        ty: value_expr.ty(),
                    },
                    expr,
                ))
            }
            HtmlExprVariant::Tag {
                tag_kind,
                ref props,
            } => {
                for (i, (_, argument)) in props.iter().enumerate() {
                    self.compile_eager_expr(
                        argument,
                        self.sheet.variable_stack.next_stack_idx() + i,
                    )
                }
                self.push_instruction(Vmir::new(
                    VmirData::NewHtmlFromTag {
                        tag_kind,
                        props: props.keys().collect(),
                        n_child_expr: 0,
                    },
                    expr.clone(),
                ))
            }
        }
    }
}
