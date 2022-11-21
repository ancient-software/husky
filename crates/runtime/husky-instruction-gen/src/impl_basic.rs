use crate::*;

use husky_vm::{Instruction, VMStackIdx};

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn push_instruction(&mut self, instr: Instruction) {
        self.sheet.instructions.push(instr);
    }

    pub(super) fn def_variable(&mut self, varname: Identifier) {
        self.sheet.variable_stack.push(varname);
    }

    pub(super) fn varidx(&self, varname: Identifier) -> VMStackIdx {
        self.sheet.variable_stack.stack_idx(varname)
    }
}
