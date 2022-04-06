use std::sync::{Arc, Mutex};

use crate::dependence::*;
use crate::*;
use check_utils::should_eq;
use entity_route::{EntityRoutePtr, EntitySource};
use infer_total::InferQueryGroup;
use semantics_error::*;
use sync_utils::ARwLock;
use upcast::Upcast;
use vm::EntityUid;

#[salsa::query_group(EntityQueryGroupStorage)]
pub trait EntityQueryGroup:
    InferQueryGroup + ast::AstQueryGroup + Upcast<dyn InferQueryGroup> + StoreEntityRoute
{
    fn main_defn(&self, main_file: file::FilePtr) -> SemanticResultArc<MainDefn>;
    fn opt_entity_defn(&self, scope: EntityRoutePtr) -> SemanticResult<Option<Arc<EntityDefn>>>;
    fn entity_immediate_dependees(&self, scope: EntityRoutePtr) -> SemanticResultArc<DependeeMap>;
    fn entity_dependees(&self, scope: EntityRoutePtr) -> SemanticResultArc<DependeeMap>;
    fn subentity_defns(&self, scope: EntityRoutePtr) -> SemanticResultArc<Vec<Arc<EntityDefn>>>;
    fn entity_defn_uid(&self, scope: EntityRoutePtr) -> EntityDefnUid;
    fn entity_uid(&self, scope: EntityRoutePtr) -> EntityUid;
}

pub trait StoreEntityRoute {
    fn entity_route_store(&self) -> &EntityRouteStore;

    fn entity_route_by_uid(&self, uid: EntityUid) -> EntityRoutePtr {
        self.entity_route_store().get(uid)
    }
}

#[derive(Debug, Default, Clone)]
pub struct EntityRouteStore {
    internal: ARwLock<Vec<EntityRoutePtr>>,
}

impl EntityRouteStore {
    fn add(&self, uid: EntityUid, entity_route: EntityRoutePtr) {
        self.internal.write(|internal: &mut Vec<EntityRoutePtr>| {
            should_eq!(uid.raw(), internal.len());
            internal.push(entity_route)
        })
    }

    fn get(&self, uid: EntityUid) -> EntityRoutePtr {
        self.internal.read(|internal| internal[uid.raw()])
    }
}

pub(crate) fn entity_uid(db: &dyn EntityQueryGroup, entity_route: EntityRoutePtr) -> EntityUid {
    // responds to changes in either defn or defns of dependees
    let entity_source = db.entity_source(entity_route).unwrap();
    match entity_source {
        // in the future, we should make a difference between entity in current pack and depending packs
        EntitySource::Builtin(_) => (),
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::Module { file } => todo!(),
        EntitySource::Contextual { main, ident } => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => {
            let _defn = db.opt_entity_defn(entity_route);
            let _dependees = db.entity_dependees(entity_route);
        }
    }
    let uid = EntityUid::new();
    db.entity_route_store().add(uid, entity_route);
    uid
}
