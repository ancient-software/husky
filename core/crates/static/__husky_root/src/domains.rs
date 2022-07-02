pub mod gui;
pub mod ml;
pub mod prover;
pub mod rl;

use ml::*;

use crate::*;

pub static DOMAINS_MODULE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "domains",
    items: &[&ML_MODULE_DEFN],
    variant: EntityStaticDefnVariant::Module,
    dev_src: __static_dev_src!(),
};
