use super::*;
use lean_entity_path::LnItemPath;
use lean_mir_expr::{
    expr::LnMirExprEntry,
    tactic::{LnMirTacticData, LnMirTacticIdxRange},
};

impl<'a, S> VdLeanTranspilationBuilder<'a, S>
where
    S: IsVdLeanTranspilationScheme,
{
    pub fn default_tactics(&mut self) -> LnMirTacticIdxRange {
        let tactic_data = self.default_tactic_data();
        let tactic = self.alloc_tactic(tactic_data);
        LnMirTacticIdxRange::new_single(tactic)
    }

    pub fn default_tactic_data(&mut self) -> LnMirTacticData {
        LnMirTacticData::Obvious
    }

    pub fn ad_hoc_tactic_data(&mut self, name: &'static str) -> LnMirTacticData {
        LnMirTacticData::AdHoc { name }
    }

    pub fn exact_unit(&mut self) -> LnMirTacticData {
        let data = LnMirExprData::ItemPath(LnItemPath::UNIT);
        let entry = LnMirExprEntry::new(data, None);
        let unit = self.alloc_expr(entry);
        LnMirTacticData::Exact { term: unit }
    }
}
