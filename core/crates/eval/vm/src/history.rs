mod entry;

use std::collections::HashMap;

use crate::*;

use check_utils::should;
pub use entry::HistoryEntry;
use print_utils::p;

#[derive(Debug, Default)]
pub struct History<'eval> {
    entries: HashMap<InstructionId, HistoryEntry<'eval>>,
}

impl<'eval> History<'eval> {
    pub fn get<T: InstructionSource>(&self, t: &T) -> Option<&HistoryEntry<'eval>> {
        self.entries.get(&t.instruction_id())
    }

    pub fn contains<T: InstructionSource>(&self, t: &T) -> bool {
        self.entries.contains_key(&t.instruction_id())
    }

    pub fn value<T: InstructionSource>(&self, t: &T) -> EvalValueResult<'eval> {
        if let Some(entry) = self.entries.get(&t.instruction_id()) {
            entry.result()
        } else {
            Ok(EvalValue::Undefined)
        }
    }

    pub fn write(&mut self, ins: &Instruction, entry: HistoryEntry<'eval>) {
        if let Some(old_value) = self.entries.insert(ins.ins_id(), entry) {
            p!(ins.src.file(), ins.src.text_range());
            p!(old_value);
            panic!()
        }
    }
}
