use crate::*;
use husky_entity_path::EntityPath;
use husky_identifier::IdentifierDb;
use husky_path::FileResultArc;
use husky_source_path::SourcePath;
use husky_term::{Decl, TermDb, Ty};
use salsa::DbWithJar;
use std::sync::Arc;
use upcast::Upcast;

pub trait TermInferDb:
    DbWithJar<TermInferJar> + TyInferQueries + TermDb + Upcast<dyn TermDb> + IdentifierDb
{
    fn entity_ty(&self, entity: EntityPath) -> Ty;

    fn term_sheet(&self, file: SourcePath) -> FileResultArc<TermSheet>;
}

impl<T> TermInferDb for T
where
    T: DbWithJar<TermInferJar> + TyInferQueries + TermDb + Upcast<dyn TermDb> + IdentifierDb,
{
    fn entity_ty(&self, entity: EntityPath) -> Ty {
        todo!()
    }

    fn term_sheet(&self, file: SourcePath) -> FileResultArc<TermSheet> {
        todo!()
    }
}

fn entity_ty(db: &dyn TermInferDb, entity: EntityPath) -> Ty {
    db.infer_entity_ty(entity)
}

fn term_sheet(db: &dyn TermInferDb, file: SourcePath) -> FileResultArc<TermSheet> {
    todo!()
}

pub trait TyInferQueries {
    fn infer_entity_ty(&self, entity: EntityPath) -> Ty;
}

pub trait TyInferQueryImpls {}

impl<T> TyInferQueries for T
where
    T: TyInferQueryImpls,
{
    fn infer_entity_ty(&self, entity: EntityPath) -> Ty {
        todo!()
    }
}
