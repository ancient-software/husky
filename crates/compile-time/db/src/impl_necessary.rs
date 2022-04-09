use infer_total::InferQueryGroup;
use linkage_table::{FpTable, HasFpTable};
use semantics_entity::{EntityRouteStore, StoreEntityRoute};
use upcast::Upcast;
use vm::InterpreterQueryGroup;

use crate::*;

impl fmt::Debug for HuskyLangCompileTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HuskyLangCompileTime").finish()
    }
}

impl salsa::Database for HuskyLangCompileTime {}

impl salsa::ParallelDatabase for HuskyLangCompileTime {
    fn snapshot(&self) -> salsa::Snapshot<HuskyLangCompileTime> {
        salsa::Snapshot::new(HuskyLangCompileTime {
            storage: self.storage.snapshot(),
            file_unique_allocator: self.file_unique_allocator.clone(),
            word_unique_allocator: self.word_unique_allocator.clone(),
            scope_unique_allocator: self.scope_unique_allocator.clone(),
            live_docs: self.live_docs.clone(),
            features: self.features.clone(),
            linkage_table: self.linkage_table.clone(),
            entity_route_store: self.entity_route_store.clone(),
        })
    }
}

impl Default for HuskyLangCompileTime {
    fn default() -> Self {
        Self {
            storage: Default::default(),
            file_unique_allocator: file::new_file_unique_allocator(),
            word_unique_allocator: word::new_word_unique_allocator(),
            scope_unique_allocator: entity_route::new_scope_unique_allocator(),
            live_docs: Default::default(),
            features: feature::new_feature_unique_allocator(),
            linkage_table: Default::default(),
            entity_route_store: Default::default(),
        }
    }
}

impl AllocateUniqueFile for HuskyLangCompileTime {
    fn file_unique_allocator(&self) -> &file::UniqueFileAllocator {
        &self.file_unique_allocator
    }
}

impl InternWord for HuskyLangCompileTime {
    fn word_allocator(&self) -> &word::WordAllocator {
        &self.word_unique_allocator
    }
}

impl LiveFiles for HuskyLangCompileTime {
    fn get_live_files(&self) -> &ARwLock<HashMap<file::FilePtr, ARwLock<String>>> {
        &self.live_docs
    }

    fn did_change_source(&mut self, id: file::FilePtr) {
        file::FileContentQuery.in_db_mut(self).invalidate(&id);
    }
}

impl FileQueryGroup for HuskyLangCompileTime {}

impl AllocateUniqueScope for HuskyLangCompileTime {
    fn scope_unique_allocator(&self) -> &entity_route::EntityRouteInterner {
        &self.scope_unique_allocator
    }
}

impl TokenQueryGroup for HuskyLangCompileTime {}

impl EntityRouteQueryGroup for HuskyLangCompileTime {}

impl AstQueryGroup for HuskyLangCompileTime {}

impl Upcast<dyn InferQueryGroup> for HuskyLangCompileTime {
    fn upcast(&self) -> &(dyn infer_total::InferQueryGroup + 'static) {
        self
    }
}

impl Upcast<dyn semantics_entity::EntityQueryGroup> for HuskyLangCompileTime {
    fn upcast(&self) -> &(dyn semantics_entity::EntityQueryGroup + 'static) {
        self
    }
}

impl AllocateUniqueFeature for HuskyLangCompileTime {
    fn features(&self) -> &feature::FeatureUniqueAllocator {
        &self.features
    }
}

impl Upcast<dyn entity_route_query::EntityRouteSalsaQueryGroup> for HuskyLangCompileTime {
    fn upcast(&self) -> &(dyn entity_route_query::EntityRouteSalsaQueryGroup + 'static) {
        self
    }
}

impl infer_ty::InferTyQueryGroup for HuskyLangCompileTime {}

impl infer_contract::InferContractQueryGroup for HuskyLangCompileTime {}

impl infer_total::InferQueryGroup for HuskyLangCompileTime {}

impl HasFpTable for HuskyLangCompileTime {
    fn linkage_table(&self) -> &FpTable {
        &self.linkage_table
    }
}

impl InterpreterQueryGroup for HuskyLangCompileTime {
    fn entity_instruction_sheet_by_uid(&self, uid: vm::EntityUid) -> Arc<vm::InstructionSheet> {
        let entity_route = self.entity_route_by_uid(uid);
        self.entity_instruction_sheet(entity_route)
    }
}

impl Upcast<dyn InterpreterQueryGroup> for HuskyLangCompileTime {
    fn upcast(&self) -> &(dyn InterpreterQueryGroup + 'static) {
        self
    }
}

impl StoreEntityRoute for HuskyLangCompileTime {
    fn entity_route_store(&self) -> &EntityRouteStore {
        &self.entity_route_store
    }
}
