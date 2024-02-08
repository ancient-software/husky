use husky_coword::Ident;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[deprecated(note = "to be replaced by Vmirs in husky-vmir")]
pub struct Vmirs {
    pub instructions: Vec<Vmir>,
    pub variable_stack: VariableStack,
}

impl Vmirs {
    pub fn new(inputs: impl Iterator<Item = Ident>, has_this: bool) -> Self {
        Self {
            instructions: vec![],
            variable_stack: VariableStack::new(inputs, has_this),
        }
    }

    pub fn init_subsheet(&self) -> Self {
        Self {
            instructions: vec![],
            variable_stack: self.variable_stack.clone(),
        }
    }
}
