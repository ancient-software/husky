// mod runtime_storage;

// use self::runtime_storage::*;
// use husky_hir_deps::HirDepsDb;
// use husky_mono_linktime::MonoLinktime;
// use husky_standard_value::RegularValue;
// use std::marker::PhantomData;

// pub struct MlTask< VisualProtocol>
// where
//     ComptimeDb: HirDepsDb,
//     VisualProtocol: IsVisualProtocol,
// {
//     _marker: PhantomData<(ComptimeDb, VisualProtocol)>,
// }

// pub struct MlDevAscension<VisualProtocol>(PhantomData<(ComptimeDb, VisualProtocol)>)
// where
//     ComptimeDb: HirDepsDb,
//     VisualProtocol: IsVisualProtocol;

// impl< VisualProtocol> IsDevAscension for MlDevAscension<VisualProtocol>
// where
//     ComptimeDb: HirDepsDb,
//     VisualProtocol: IsVisualProtocol,
// {
//     type Base = DevInput;

//     type LinkTime = MonoLinktime<ComptimeDb, MlLinkage>;

//     type Value = RegularValue;

//     type RuntimeStorage = MlDevRuntimeStorage;

//     type RuntimeTaskSpecificConfig = ();

//     type VisualProtocol = VisualProtocol;
// }

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
// pub enum DevInput {
//     Train(),
//     Ki(),
// }

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum MlLinkage {}

// impl IsLinkage for MlLinkage {
//     type Value = RegularValue;

//     fn eval_fn() -> Self::Value {
//         todo!()
//     }

//     fn eval_gn() -> Self::Value {
//         todo!()
//     }
// }
