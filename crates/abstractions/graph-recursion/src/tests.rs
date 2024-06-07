mod linear_graph;

use crate::context::{IsGraphRecursionContext, IsGraphRecursionScheme};
use crate::cycle_group::{CycleGroup, CycleGroupMap};

#[salsa::jar]
pub struct Jar(
    self::linear_graph::LinearGraphNode,
    self::linear_graph::LinearGraphCycleGroupItd,
    self::linear_graph::linear_graph_full_deps,
    self::linear_graph::linear_graph_cycle_group_recursion_values,
);

#[salsa::db(Jar)]
pub struct DB();
