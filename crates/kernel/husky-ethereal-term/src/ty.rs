use crate::*;
use husky_declarative_ty::{principal_item_path::ty_variant::ty_variant_path_declarative_ty, *};
use husky_vfs::Toolchain;

pub trait HasType: Copy {
    fn ty(self, db: &::salsa::Db) -> EtherealTermResult<EtherealTerm>;
}

pub trait HasTypeGivenToolchain: Copy {
    fn ty(self, db: &::salsa::Db, toolchain: Toolchain) -> EtherealTermResult<EtherealTerm>;
}

pub trait HasTypeGivenDisambiguation: Copy {
    fn ty(
        self,
        db: &::salsa::Db,
        disambiguation: TypePathDisambiguation,
    ) -> EtherealTermResult<EtherealTerm>;
}

impl HasTypeGivenDisambiguation for PrincipalEntityPath {
    fn ty(
        self,
        db: &::salsa::Db,
        disambiguation: TypePathDisambiguation,
    ) -> EtherealTermResult<EtherealTerm> {
        match self {
            PrincipalEntityPath::Module(path) => Ok(db
                .ethereal_term_menu(path.toolchain(db))
                .module_ty_ontology()),
            PrincipalEntityPath::MajorItem(path) => path.ty(db, disambiguation),
            PrincipalEntityPath::TypeVariant(path) => path.ty(db),
        }
    }
}

impl HasTypeGivenDisambiguation for MajorItemPath {
    fn ty(
        self,
        db: &::salsa::Db,
        disambiguation: TypePathDisambiguation,
    ) -> EtherealTermResult<EtherealTerm> {
        match self {
            MajorItemPath::Type(path) => path.ty(db, disambiguation),
            MajorItemPath::Trait(path) => path.ty(db),
            MajorItemPath::Fugitive(path) => path.ty(db),
        }
    }
}

impl HasType for TraitPath {
    fn ty(self, db: &::salsa::Db) -> EtherealTermResult<EtherealTerm> {
        EtherealTerm::ty_from_declarative(db, trai_path_declarative_ty(db, self)?)
    }
}

impl HasType for FugitivePath {
    fn ty(self, db: &::salsa::Db) -> EtherealTermResult<EtherealTerm> {
        EtherealTerm::ty_from_declarative(db, fugitive_path_declarative_ty(db, self)?)
    }
}

impl HasTypeGivenDisambiguation for TypePath {
    fn ty(
        self,
        db: &::salsa::Db,
        disambiguation: TypePathDisambiguation,
    ) -> EtherealTermResult<EtherealTerm> {
        match disambiguation {
            TypePathDisambiguation::OntologyConstructor => {
                EtherealTerm::ty_from_declarative(db, ty_ontology_path_declarative_ty(db, self)?)
            }
            TypePathDisambiguation::InstanceConstructor => EtherealTerm::ty_from_declarative(
                db,
                ty_instance_constructor_path_declarative_ty(db, self)?,
            ),
        }
    }
}

impl HasType for TypeVariantPath {
    fn ty(self, db: &::salsa::Db) -> EtherealTermResult<EtherealTerm> {
        EtherealTerm::ty_from_declarative(db, ty_variant_path_declarative_ty(db, self)?)
    }
}

impl HasTypeGivenToolchain for EtherealTerm {
    fn ty(self, _db: &::salsa::Db, _toolchain: Toolchain) -> EtherealTermResult<EtherealTerm> {
        todo!()
        // self.ty_unchecked(db)?.checked(db)
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum RawType {
    Prelude(PreludeTypePath),
    Declarative(DeclarativeTerm),
}

impl EtherealTerm {
    pub fn ty_unchecked(
        self,
        db: &::salsa::Db,
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

    pub fn raw_ty(self, db: &::salsa::Db) -> EtherealTermResult<RawType> {
        Ok(match self {
            EtherealTerm::Literal(literal) => RawType::Prelude(literal.ty()),
            EtherealTerm::Symbol(symbol) => {
                RawType::Declarative(symbol.ty(db).into_declarative(db))
            }
            EtherealTerm::Rune(variable) => {
                RawType::Declarative(variable.ty(db).into_declarative(db))
            }
            EtherealTerm::EntityPath(path) => match path {
                TermEntityPath::Fugitive(_) => todo!(),
                TermEntityPath::Trait(path) => {
                    RawType::Declarative(trai_path_declarative_ty(db, path)?)
                }
                TermEntityPath::TypeOntology(path) => {
                    RawType::Declarative(ty_ontology_path_declarative_ty(db, path)?)
                }
                TermEntityPath::TypeInstance(path) => {
                    RawType::Declarative(ty_instance_constructor_path_declarative_ty(db, path)?)
                }
                TermEntityPath::TypeVariant(path) => {
                    RawType::Declarative(ty_variant_path_declarative_ty(db, path)?)
                }
            },
            EtherealTerm::Category(cat) => RawType::Declarative(cat.ty()?.into()),
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(curry) => {
                let Ok(RawType::Declarative(DeclarativeTerm::Category(parameter_ty_cat))) =
                    curry.parameter_ty(db).raw_ty(db)
                else {
                    unreachable!()
                };
                let Ok(RawType::Declarative(DeclarativeTerm::Category(return_ty_cat))) =
                    curry.return_ty(db).raw_ty(db)
                else {
                    unreachable!()
                };
                RawType::Declarative(parameter_ty_cat.max(return_ty_cat).into())
            }
            EtherealTerm::Ritchie(_) => {
                DeclarativeTerm::Category(TermCategory::new(1.into())).into()
            }
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(term) => RawType::Declarative(term.declarative_ty(db)?),
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        })
    }
}
