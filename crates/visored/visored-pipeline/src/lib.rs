/// builder builds one instance of the pipeline, returns a tracker
mod builder;
pub mod config;
pub mod error;
pub mod input;
/// smallest unit of the pipeline
mod instance;
/// runner orchestrates all instances
pub mod runner;
#[cfg(test)]
mod tests;
/// tracker keeps the values of instance execution
mod tracker;

use self::{config::*, error::*};
