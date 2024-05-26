use crate::{error::InsertEntryRepeatError, *};
use smallvec::*;

/// always keep the elements in order,
/// useful for keys
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub struct OrderedSmallVecSet<K, const N: usize>
where
    [K; N]: Array<Item = K>,
{
    data: SmallVec<[K; N]>,
}

impl<K, const N: usize> OrderedSmallVecSet<K, N>
where
    [K; N]: Array<Item = K>,
{
    pub fn new_one_elem_set(elem: K) -> Self {
        Self {
            data: smallvec![elem],
        }
    }
}

impl<K, const N: usize> Default for OrderedSmallVecSet<K, N>
where
    [K; N]: Array<Item = K>,
{
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<K, const N: usize> Deref for OrderedSmallVecSet<K, N>
where
    [K; N]: Array<Item = K>,
{
    type Target = [K];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<K, const N: usize> AsRef<[K]> for OrderedSmallVecSet<K, N>
where
    K: Eq + Ord + Copy,
    [K; N]: Array<Item = K>,
{
    fn as_ref(&self) -> &[K] {
        &self.data
    }
}

impl<K, const N: usize> FromIterator<K> for OrderedSmallVecSet<K, N>
where
    K: Eq + Ord + Copy,
    [K; N]: Array<Item = K>,
{
    fn from_iter<T: IntoIterator<Item = K>>(t: T) -> Self {
        let mut data: SmallVec<[K; N]> = t.into_iter().collect();
        data.sort();
        Self { data }
    }
}

impl<K, const N: usize, const M: usize> From<[K; M]> for OrderedSmallVecSet<K, N>
where
    K: Eq + Ord + Copy,
    [K; N]: Array<Item = K>,
{
    fn from(value: [K; M]) -> Self {
        Self::from_iter(value.into_iter())
    }
}

impl<K, const N: usize> From<Vec<K>> for OrderedSmallVecSet<K, N>
where
    K: Eq + Ord + Copy,
    [K; N]: Array<Item = K>,
{
    fn from(value: Vec<K>) -> Self {
        Self::from_iter(value.into_iter())
    }
}

impl<K, const N: usize> OrderedSmallVecSet<K, N>
where
    [K; N]: Array<Item = K>,
{
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn has(&self, key: K) -> bool
    where
        K: Copy + Eq,
    {
        self.data.iter().find(|entry| **entry == key).is_some()
    }
    pub fn has_ref(&self, key: &K) -> bool
    where
        K: Eq,
    {
        self.data.iter().find(|entry| *entry == key).is_some()
    }

    pub fn contains(&self, key: &K) -> bool
    where
        K: Eq,
    {
        self.data.iter().find(|entry| *entry == key).is_some()
    }

    pub fn insert_new(&mut self, new: K) -> Result<(), InsertEntryRepeatError<K>>
    where
        K: Copy + Ord,
    {
        match self.data.binary_search(&new) {
            Ok(old) => Err(InsertEntryRepeatError { old, new }),
            Err(pos) => {
                self.data.insert(pos, new);
                Ok(())
            }
        }
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn toggle(&mut self, value: K)
    where
        K: Ord,
    {
        match self.data.binary_search(&value) {
            Ok(old) => {
                self.data.remove(old);
            }
            Err(pos) => self.data.insert(pos, value),
        }
    }

    pub fn remove(&mut self, value: K)
    where
        K: Ord,
    {
        match self.data.binary_search(&value) {
            Ok(old) => {
                self.data.remove(old);
            }
            Err(_pos) => (),
        }
    }

    pub fn to_vec(&self) -> SmallVec<[K; N]>
    where
        K: Copy,
    {
        self.data.clone()
    }

    pub fn insert(&mut self, value: K)
    where
        K: Copy + Ord,
    {
        match self.data.binary_search(&value) {
            Ok(_old) => (),
            Err(pos) => self.data.insert(pos, value),
        }
    }

    pub fn extend(&mut self, other: &Self)
    where
        K: Copy + Ord,
    {
        for &element in &other.data {
            self.insert(element)
        }
    }

    pub fn data(&self) -> &[K] {
        self.data.as_ref()
    }
}

#[test]
fn ordered_small_vec_set_toggle_works() {
    let mut set: OrderedSmallVecSet<i32, 2> = OrderedSmallVecSet::new_one_elem_set(1);
    assert_eq!(&set.data as &[_], &[1]);
    set.toggle(2);
    assert_eq!(&set.data as &[_], &[1, 2]);
    set.toggle(0);
    assert_eq!(&set.data as &[_], &[0, 1, 2]);
    set.toggle(1);
    assert_eq!(&set.data as &[_], &[0, 2]);
    set.toggle(2);
    assert_eq!(&set.data as &[_], &[0]);
}

#[test]
fn ordered_small_vec_set_remove_works() {
    let mut set: OrderedSmallVecSet<i32, 2> = OrderedSmallVecSet::new_one_elem_set(1);
    assert_eq!(&set.data as &[_], &[1]);
    set.remove(2);
    assert_eq!(&set.data as &[_], &[1,]);
    set.remove(1);
    assert_eq!(&set.data as &[_], &[]);
}
