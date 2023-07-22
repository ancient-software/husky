use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
pub struct SubmoduleSynNodeDefn {
    syn_node_decl: SubmoduleNodeDecl,
}

impl SubmoduleSynNodeDefn {
    pub fn syn_node_decl(self) -> SubmoduleNodeDecl {
        self.syn_node_decl
    }
}

impl HasSynNodeDefn for SubmoduleSynNodePath {
    type NodeDefn = SubmoduleSynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::NodeDefn {
        SubmoduleSynNodeDefn {
            syn_node_decl: self.syn_node_decl(db),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
pub struct SubmoduleDefn {
    decl: SubmoduleDecl,
}

impl SubmoduleDefn {
    pub fn decl(self) -> SubmoduleDecl {
        self.decl
    }
}

impl HasDefn for ModulePath {
    type Defn = SubmoduleDefn;

    fn defn(self, db: &dyn SynDefnDb) -> DefnResult<Self::Defn> {
        Ok(SubmoduleDefn {
            decl: self.decl(db)?,
        })
    }
}
