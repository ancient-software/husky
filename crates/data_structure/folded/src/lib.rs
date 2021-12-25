mod folded_list;
mod folded_stack;
mod generator;
mod iter;

pub use folded_list::{FoldedList, FoldedNode, ItemToFold};
pub use folded_stack::FoldedStack;
pub use generator::Generator;
pub use iter::FoldedIter;

pub trait FoldedStorage<Value, This>
where
    Value: ?Sized,
    This: FoldedStorage<Value, This>,
{
    fn len(&self) -> usize;
    fn next_sibling(&self, index: usize) -> Option<usize>;
    fn value(&self, index: usize) -> &Value;
    fn this(&self) -> &This;

    fn folded_iter(&self, start: usize) -> FoldedIter<Value, This> {
        FoldedIter::new(self.this(), Some(start))
    }
}
