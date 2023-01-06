use crate::*;

#[salsa::tracked(jar = DefnJar)]
pub struct TupleStructTypeDefn {
    #[id]
    pub path: TypePath,
    pub decl: TupleStructTypeDecl,
}
