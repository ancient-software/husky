use crate::{
    amazon::package_amazon_javelins, instantiation::JavelinInstantiation, javelin::JavelinData,
    path::JavelinPath, *,
};
use fxhash::FxHashMap;
use husky_entity_path::{ItemPathId, MajorItemPath, PrincipalEntityPath};
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use husky_hir_decl::{parameter::template::HirTemplateParameters, HasHirDecl};
use husky_hir_defn::HasHirDefn;
use husky_hir_eager_expr::{HirEagerExprData, HirEagerExprRegion};
use husky_hir_expr::HirExprRegion;
use husky_hir_lazy_expr::{HirLazyExprData, HirLazyExprRegion};
use husky_hir_ty::instantiation::HirInstantiation;
use husky_manifest::HasPackageManifest;
use husky_vfs::PackagePath;
use smallvec::ToSmallVec;
use vec_like::VecSet;

/// can be instantiated to a path leading javelin given JavelinInstantiation
#[derive(Debug, PartialEq, Eq)]
pub struct ValkyrieRide {
    javelin_item_path: JavelinPath,
    hir_instantiation: HirInstantiation,
}

pub struct ValkyrieJavelin(Javelin);

impl ValkyrieRide {
    fn to_javelin(
        &self,
        javelin_instantiation: &JavelinInstantiation,
        db: &::salsa::Db,
    ) -> Javelin {
        Javelin::new(
            db,
            JavelinData::PathLeading {
                path: self.javelin_item_path,
                instantiation: JavelinInstantiation::from_hir(
                    &self.hir_instantiation,
                    Some(javelin_instantiation),
                    db,
                ),
            },
        )
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct ValkyrieRides {
    hir_template_parameters: Option<HirTemplateParameters>,
    rides: VecSet<ValkyrieRide>,
}

#[salsa::tracked(jar = JavelinJar, return_ref)]
fn item_valkyrie_rides(db: &::salsa::Db, id: ItemPathId) -> Option<ValkyrieRides> {
    let item_path = id.item_path(db);
    let hir_decl = item_path.hir_decl(db)?;
    let hir_template_parameters = hir_decl.template_parameters(db).map(Clone::clone);
    let mut valkyrie_ride = ValkyrieRides {
        hir_template_parameters,
        rides: Default::default(),
    };
    valkyrie_ride.add_hir_expr_region(hir_decl.hir_expr_region(db)?, db);
    if let Some(hir_expr_region) = item_path.hir_defn(db)?.hir_expr_region(db) {
        valkyrie_ride.add_hir_expr_region(hir_expr_region, db);
    }
    Some(valkyrie_ride)
}

impl ValkyrieRides {
    fn add_hir_expr_region(&mut self, hir_expr_region: HirExprRegion, db: &::salsa::Db) {
        match hir_expr_region {
            HirExprRegion::Eager(hir_eager_expr_region) => {
                self.add_hir_eager_expr_region(hir_eager_expr_region, db)
            }
            HirExprRegion::Lazy(hir_lazy_expr_region) => {
                self.add_hir_lazy_expr_region(hir_lazy_expr_region, db)
            }
        }
    }

    fn add_hir_eager_expr_region(
        &mut self,
        hir_eager_expr_region: HirEagerExprRegion,
        db: &::salsa::Db,
    ) {
        for data in hir_eager_expr_region.expr_arena(db) {
            #[deprecated(note = "incomplete")]
            match *data {
                HirEagerExprData::AssociatedFn {
                    associated_item_path,
                } => (), // ad hoc
                HirEagerExprData::PrincipalEntityPath(path) => match path {
                    PrincipalEntityPath::Module(_) => unreachable!(),
                    PrincipalEntityPath::MajorItem(path) => match path {
                        MajorItemPath::Type(_) => (),
                        MajorItemPath::Trait(_) => (),
                        MajorItemPath::Fugitive(_) => (),
                    },
                    PrincipalEntityPath::TypeVariant(path) => (),
                },
                HirEagerExprData::Be { src, ref target } => (),
                HirEagerExprData::TypeConstructorFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirEagerExprData::TypeVariantConstructorCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirEagerExprData::FunctionFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirEagerExprData::AssociatedFunctionFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirEagerExprData::PropsStructField {
                    owner_hir_expr_idx,
                    ident,
                } => (),
                HirEagerExprData::MemoizedField {
                    owner_hir_expr_idx,
                    ident,
                    path,
                } =>
                /* ad hoc */
                {
                    ()
                }
                HirEagerExprData::MethodFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirEagerExprData::NewTuple { ref items } => (),
                HirEagerExprData::Index {
                    owner_hir_expr_idx,
                    ref items,
                } => (),
                HirEagerExprData::NewList { ref items } => (),
                _ => (),
            }
        }
    }

    fn add_hir_lazy_expr_region(
        &mut self,
        hir_lazy_expr_region: HirLazyExprRegion,
        db: &::salsa::Db,
    ) {
        for data in hir_lazy_expr_region.hir_lazy_expr_arena(db) {
            match *data {
                HirLazyExprData::AssociatedFn { path } => (),
                HirLazyExprData::PrincipalEntityPath(_) => (),
                HirLazyExprData::Be { src, ref target } =>
                /* ad hoc */
                {
                    ()
                }
                HirLazyExprData::TypeConstructorFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirLazyExprData::TypeVariantConstructorFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirLazyExprData::FunctionFnItemCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirLazyExprData::FunctionGnItemCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirLazyExprData::AssociatedFunctionFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirLazyExprData::PropsStructField { owner, ident } => (), // ad hoc
                HirLazyExprData::MemoizedField {
                    owner,
                    ident,
                    path,
                    ref indirections,
                    ref instantiation,
                } => (), // ad hoc
                HirLazyExprData::MethodFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation), // todo: handle indirections
                HirLazyExprData::NewTuple { ref items } => (),
                HirLazyExprData::Index { owner, ref items } => (),
                HirLazyExprData::NewList { ref items } => (),
                _ => (),
            }
        }
    }

    fn try_add_ride(&mut self, javelin_path: JavelinPath, instantiation: &HirInstantiation) {
        if !instantiation.is_empty() {
            self.rides.insert_move(ValkyrieRide {
                javelin_item_path: javelin_path,
                hir_instantiation: instantiation.clone(),
            })
        }
    }
}

#[test]
fn item_valkyrie_rides_works() {
    DB::default().ast_expect_test_debug_with_db(
        |db, module_path| -> Vec<_> {
            module_item_paths(db, module_path)
                .iter()
                .map(|&item_path| item_valkyrie_rides(db, *item_path))
                .collect()
        },
        &AstTestConfig::new("item_valkyrie_rides"),
    )
}

#[salsa::tracked(jar = JavelinJar, return_ref)]
pub(crate) fn javelin_valkyrie_javelins(db: &::salsa::Db, javelin: Javelin) -> VecSet<Javelin> {
    match *javelin.data(db) {
        JavelinData::Coersion {} => Default::default(),
        JavelinData::PathLeading {
            path,
            ref instantiation,
        } => {
            let Some(valkyrie_rides) = item_valkyrie_rides(db, *path) else {
                return Default::default();
            };
            valkyrie_rides
                .rides
                .iter()
                .map(|ride| ride.to_javelin(instantiation, db))
                .collect()
        }
        JavelinData::PropsStructField => todo!(),
        JavelinData::MemoizedField => todo!(),
        JavelinData::Index => todo!(),
        JavelinData::Method => todo!(),
    }
}

pub struct PackageValkyrieJavelinsBuilder<'db> {
    db: &'db ::salsa::Db,
    javelins: VecSet<Javelin>,
}

impl<'db> PackageValkyrieJavelinsBuilder<'db> {}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct JavelinPantheon {
    package_path: PackagePath,
    // map each javelin to a package where it's instantiated
    instantiation_map: FxHashMap<Javelin, PackagePath>,
    new_javelins: Vec<Javelin>,
}

#[salsa::tracked(jar = JavelinJar, return_ref)]
pub(crate) fn package_valkyrie_javelin_pantheon(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> JavelinPantheon {
    let mut pantheon = JavelinPantheon {
        package_path,
        instantiation_map: Default::default(),
        new_javelins: Default::default(),
    };
    for dep in package_path
        .package_dependencies(db)
        .expect("no error at this stage")
    {
        pantheon.add_valkyrie_javelins_instantiated_by_package(package_valkyrie_javelin_pantheon(
            db,
            dep.package_path(),
        ))
    }
    let mut bar = 0;
    for &javelin in package_amazon_javelins(db, package_path) {
        pantheon.try_add_valkyrie_javelins_instantiated_by_javelin(javelin, db)
    }
    while bar < pantheon.new_javelins.len() {
        let new_bar = pantheon.new_javelins.len();
        for i in bar..pantheon.new_javelins.len() {
            pantheon.try_add_valkyrie_javelins_instantiated_by_javelin(pantheon.new_javelins[i], db)
        }
        bar = new_bar
    }
    pantheon
}

impl JavelinPantheon {
    pub fn new_javelins<'a>(&'a self) -> impl Iterator<Item = Javelin> + 'a {
        self.instantiation_map
            .iter()
            .filter_map(|(&javelin, &package_path)| {
                (package_path == self.package_path).then_some(javelin)
            })
    }

    fn add_valkyrie_javelins_instantiated_by_package(
        &mut self,
        other_package_javelin_pantheon: &Self,
    ) {
        for (&javelin, &package_path) in other_package_javelin_pantheon.instantiation_map.iter() {
            self.instantiation_map
                .entry(javelin)
                .or_insert(package_path);
        }
    }

    // do nothing if already instantiated
    fn try_add_valkyrie_javelins_instantiated_by_javelin(
        &mut self,
        javelin: Javelin,
        db: &::salsa::Db,
    ) {
        for &valkyrie_javelin in javelin_valkyrie_javelins(db, javelin) {
            match self.instantiation_map.get(&valkyrie_javelin) {
                Some(_) => (),
                None => self.add_new_javelin(valkyrie_javelin),
            }
        }
    }

    fn add_new_javelin(&mut self, javelin: Javelin) {
        self.instantiation_map.insert(javelin, self.package_path);
        self.new_javelins.push(javelin)
    }
}

#[test]
fn package_javelin_pantheon_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug_with_db(
        |db, package_path| package_valkyrie_javelin_pantheon(db, package_path),
        &AstTestConfig::new("package_javelin_pantheon"),
    )
}
