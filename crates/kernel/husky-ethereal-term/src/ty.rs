use crate::*;
use husky_declarative_ty::*;
use husky_ty_expectation::TypePathDisambiguation;
use husky_vfs::Toolchain;

pub trait HasType: Copy {
    fn ty(self, db: &dyn EtherealTermDb) -> EtherealTermResult<EtherealTerm>;
}

pub trait HasTypeGivenToolchain: Copy {
    fn ty(self, db: &dyn EtherealTermDb, toolchain: Toolchain) -> EtherealTermResult<EtherealTerm>;
}

pub trait HasTypeGivenDisambiguation: Copy {
    fn ty(
        self,
        db: &dyn EtherealTermDb,
        disambiguation: TypePathDisambiguation,
    ) -> EtherealTermResult<EtherealTerm>;
}

impl HasTypeGivenDisambiguation for EntityPath {
    fn ty(
        self,
        db: &dyn EtherealTermDb,
        disambiguation: TypePathDisambiguation,
    ) -> EtherealTermResult<EtherealTerm> {
        match self {
            EntityPath::Module(path) => Ok(db
                .ethereal_term_menu(path.toolchain(db))
                .module_ty_ontology()),
            EntityPath::ModuleItem(path) => path.ty(db, disambiguation),
            EntityPath::AssociatedItem(path) => path.ty(db),
            EntityPath::TypeVariant(path) => path.ty(db),
            EntityPath::ImplBlock(_) => todo!(),
        }
    }
}

impl HasTypeGivenDisambiguation for ModuleItemPath {
    fn ty(
        self,
        db: &dyn EtherealTermDb,
        disambiguation: TypePathDisambiguation,
    ) -> EtherealTermResult<EtherealTerm> {
        match self {
            ModuleItemPath::Type(path) => path.ty(db, disambiguation),
            ModuleItemPath::Trait(path) => path.ty(db),
            ModuleItemPath::Fugitive(path) => path.ty(db),
        }
    }
}

impl HasType for TraitPath {
    fn ty(self, db: &dyn EtherealTermDb) -> EtherealTermResult<EtherealTerm> {
        EtherealTerm::ty_from_declarative(db, trai_path_declarative_ty(db, self)?)
    }
}

impl HasType for FugitivePath {
    fn ty(self, db: &dyn EtherealTermDb) -> EtherealTermResult<EtherealTerm> {
        EtherealTerm::ty_from_declarative(db, form_path_declarative_ty(db, self)?)
    }
}

impl HasTypeGivenDisambiguation for TypePath {
    fn ty(
        self,
        db: &dyn EtherealTermDb,
        disambiguation: TypePathDisambiguation,
    ) -> EtherealTermResult<EtherealTerm> {
        match disambiguation {
            TypePathDisambiguation::Ontology => {
                EtherealTerm::ty_from_declarative(db, ty_ontology_path_declarative_ty(db, self)?)
            }
            TypePathDisambiguation::Constructor => {
                EtherealTerm::ty_from_declarative(db, ty_constructor_path_declarative_ty(db, self)?)
            }
        }
    }
}

impl HasType for AssociatedItemPath {
    fn ty(self, db: &dyn EtherealTermDb) -> EtherealTermResult<EtherealTerm> {
        match self {
            AssociatedItemPath::TypeItem(path) => path.ty(db),
            AssociatedItemPath::TraitItem(path) => path.ty(db),
            AssociatedItemPath::TraitForTypeItem(path) => path.ty(db),
        }
    }
}

impl HasType for TypeItemPath {
    fn ty(self, db: &dyn EtherealTermDb) -> EtherealTermResult<EtherealTerm> {
        EtherealTerm::ty_from_declarative(db, self.declarative_ty(db)?)
    }
}

impl HasType for TraitItemPath {
    fn ty(self, db: &dyn EtherealTermDb) -> EtherealTermResult<EtherealTerm> {
        todo!()
    }
}

impl HasType for TraitForTypeItemPath {
    fn ty(self, db: &dyn EtherealTermDb) -> EtherealTermResult<EtherealTerm> {
        todo!()
    }
}

impl HasType for TypeVariantPath {
    fn ty(self, db: &dyn EtherealTermDb) -> EtherealTermResult<EtherealTerm> {
        EtherealTerm::ty_from_declarative(db, ty_variant_path_declarative_ty(db, self)?)
    }
}

impl HasTypeGivenToolchain for EtherealTerm {
    fn ty(self, db: &dyn EtherealTermDb, toolchain: Toolchain) -> EtherealTermResult<EtherealTerm> {
        todo!()
        // self.ty_unchecked(db)?.checked(db)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EtherealTermDb)]
pub enum RawType {
    Prelude(PreludeTypePath),
    Declarative(DeclarativeTerm),
}

impl EtherealTerm {
    pub fn ty_unchecked(
        self,
        db: &dyn EtherealTermDb,
    ) -> EtherealTermResult<Either<EtherealTerm, PreludeTypePath>> {
        Ok(match self.raw_ty(db)? {
            RawType::Declarative(declarative_ty) => Left(EtherealTerm::from_declarative(
                db,
                declarative_ty,
                TermTypeExpectation::FinalDestinationEqsSort,
            )?),
            RawType::Prelude(prelude_ty_path) => Right(prelude_ty_path),
        })
    }

    pub fn raw_ty(self, db: &dyn EtherealTermDb) -> EtherealTermResult<RawType> {
        Ok(match self {
            EtherealTerm::Literal(literal) => RawType::Prelude(literal.ty()),
            EtherealTerm::Symbol(symbol) => {
                RawType::Declarative(symbol.ty(db).into_declarative(db))
            }
            EtherealTerm::Variable(variable) => {
                RawType::Declarative(variable.ty(db).into_declarative(db))
            }
            EtherealTerm::EntityPath(path) => match path {
                TermEntityPath::Form(_) => todo!(),
                TermEntityPath::Trait(path) => {
                    RawType::Declarative(trai_path_declarative_ty(db, path)?)
                }
                TermEntityPath::TypeOntology(path) => {
                    RawType::Declarative(ty_ontology_path_declarative_ty(db, path)?)
                }
                TermEntityPath::TypeInstance(path) => {
                    RawType::Declarative(ty_constructor_path_declarative_ty(db, path)?)
                }
            },
            EtherealTerm::Category(cat) => RawType::Declarative(cat.ty()?.into()),
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(term) => RawType::Declarative(term.declarative_ty(db)?),
            EtherealTerm::Subentity(_) => todo!(),
            EtherealTerm::AsTraitSubentity(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        })
    }
}
