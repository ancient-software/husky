mod entry;

use std::collections::HashMap;

use crate::*;

pub use entry::HistoryEntry;
use husky_print_utils::p;

#[derive(Debug, Default)]
pub struct History<'eval> {
    entries: HashMap<InstructionId, HistoryEntry<'eval>>,
}

impl<'eval> History<'eval> {
    pub fn write(&mut self, ins: &Instruction, entry: HistoryEntry<'eval>) {
        if let Some(old_value) = self.entries.insert(ins.ins_id(), entry) {
            p!(ins.src.source(), ins.src.text_range(), ins.src);
            p!(old_value);
            panic!()
        }
    }
}

impl History<'static> {
    pub fn register_result<T: InstructionSource>(
        &self,
        t: &T,
    ) -> Option<__VMResult<__Register<'static>>> {
        self.entries
            .get(&t.instruction_id())
            .map(|entry| entry.result())
    }

    pub fn get<T: InstructionSource>(&self, t: &T) -> Option<&HistoryEntry<'static>> {
        self.entries.get(&t.instruction_id())
    }

    pub fn contains<T: InstructionSource>(&self, t: &T) -> bool {
        self.entries.contains_key(&t.instruction_id())
    }
}
