use crate::*;

pub trait InterpreterQueryGroup {
    fn entity_opt_instruction_sheet_by_uid(&self, uid: EntityUid) -> Option<Arc<InstructionSheet>>;
}

// impl InterpreterQueryGroup for husky-compilerompileTime {
//     fn entity_opt_instruction_sheet_by_uid(
//         &self,
//         uid: husky_vm::EntityUid,
//     ) -> Option<Arc<husky_vm::InstructionSheet>> {
//         let entity_path = self.entity_route_by_uid(uid);
//         self.entity_instruction_sheet(entity_path)
//     }
// }
