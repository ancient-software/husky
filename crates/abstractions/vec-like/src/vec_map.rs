use crate::{
    error::{FromVecEntryRepeatError, InsertEntryRepeatError},
    *,
};

pub trait AsVecMapEntry {
    type K;
    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        *self.key_ref()
    }

    fn key_ref(&self) -> &Self::K;
}

impl<K, T> AsVecMapEntry for &T
where
    T: AsVecMapEntry<K = K>,
{
    type K = K;
    fn key(&self) -> K
    where
        K: Copy,
    {
        <T as AsVecMapEntry>::key(self)
    }

    fn key_ref(&self) -> &K {
        <T as AsVecMapEntry>::key_ref(self)
    }
}

pub trait DefaultVecMapEntry<K> {
    fn default_from_key(key: K) -> Self;
}

impl<K, T> DefaultVecMapEntry<K> for (K, T)
where
    T: Default,
{
    fn default_from_key(key: K) -> Self {
        (key, T::default())
    }
}

impl<K, T> AsVecMapEntry for (K, T) {
    type K = K;

    fn key(&self) -> K
    where
        K: Copy,
    {
        self.0
    }

    fn key_ref(&self) -> &K {
        &self.0
    }
}

impl<K, M, T> AsVecMapEntry for (K, M, T) {
    type K = K;

    fn key(&self) -> K
    where
        K: Copy,
    {
        self.0
    }

    fn key_ref(&self) -> &K {
        &self.0
    }
}

impl<K, T> AsVecMapEntry for Arc<T>
where
    T: AsVecMapEntry<K = K>,
{
    type K = K;
    fn key(&self) -> K
    where
        K: Copy,
    {
        (**self).key()
    }

    fn key_ref(&self) -> &K {
        (**self).key_ref()
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct VecMap<V> {
    entries: Vec<V>,
}

impl<'a, V> IntoIterator for &'a VecMap<V> {
    type Item = &'a V;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

pub trait VecMapGetEntry<V>
where
    V: AsVecMapEntry,
    V::K: Copy,
{
    fn get_entry<'a>(&'a self, k: <V as AsVecMapEntry>::K) -> Option<&'a V>;
}

impl<V> VecMapGetEntry<V> for [V]
where
    V: AsVecMapEntry,
    V::K: Copy + Eq,
{
    fn get_entry<'a>(&'a self, k: <V as AsVecMapEntry>::K) -> Option<&'a V> {
        self.iter().find(|v| v.key() == k)
    }
}

impl<K, V> IntoIterator for VecMap<V>
where
    K: PartialEq + Eq,
    V: AsVecMapEntry<K = K>,
{
    type Item = V;

    type IntoIter = std::vec::IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<K, V> std::fmt::Debug for VecMap<V>
where
    K: PartialEq + Eq + std::fmt::Debug,
    V: AsVecMapEntry<K = K> + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.entries.fmt(f)
    }
}

pub type VecPairMap<K, V> = VecMap<(K, V)>;

impl<K, Entry> VecMap<Entry>
where
    K: PartialEq + Eq,
    Entry: AsVecMapEntry<K = K>,
{
    pub fn clear(&mut self) {
        self.entries.clear()
    }

    pub fn take_data(self) -> Vec<Entry> {
        self.entries
    }
    pub fn data(&self) -> &[Entry] {
        &self.entries
    }
    pub fn data_mut(&mut self) -> &mut [Entry] {
        &mut self.entries
    }

    pub fn new_one_element_map(entry: Entry) -> Self {
        Self {
            entries: vec![entry],
        }
    }

    pub fn from_vec(data: Vec<Entry>) -> Result<Self, FromVecEntryRepeatError>
    where
        K: Copy,
    {
        for i in 0..data.len() {
            for j in (i + 1)..data.len() {
                if data[i].key() == data[j].key() {
                    return Err(FromVecEntryRepeatError { i, j });
                }
            }
        }
        Ok(Self { entries: data })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn get_entry(&self, key: K) -> Option<&Entry>
    where
        K: Copy,
    {
        self.entries.iter().find(|entry| entry.key() == key)
    }

    pub unsafe fn get_entry_mut(&mut self, key: K) -> Option<&mut Entry>
    where
        K: Copy,
    {
        self.entries.iter_mut().find(|entry| entry.key() == key)
    }

    pub fn iget_entry(&self, key: K) -> Option<(usize, &Entry)>
    where
        K: Copy,
    {
        self.entries
            .iter()
            .enumerate()
            .find(|(_, entry)| entry.key() == key)
    }

    pub fn contains(&self, key: K) -> bool
    where
        K: Copy,
    {
        self.entries
            .iter()
            .find(|entry| entry.key() == key)
            .is_some()
    }

    pub fn keys<'a>(&'a self) -> impl Iterator<Item = K> + 'a
    where
        K: Copy,
    {
        self.entries.iter().map(|entry| entry.key())
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut Entry>
    where
        K: Copy,
    {
        self.entries.iter_mut().find(|entry| entry.key() == key)
    }

    pub fn insert_new(&mut self, new: Entry) -> Result<(), InsertEntryRepeatError<Entry>>
    where
        K: Copy,
    {
        if self.contains(new.key()) {
            let new_key = new.key();
            Err(InsertEntryRepeatError {
                old: self
                    .entries
                    .iter()
                    .position(|entry| entry.key() == new_key)
                    .unwrap()
                    .into(),
                new,
            })
        } else {
            self.entries.push(new);
            Ok(())
        }
    }

    /// use this when you are absoluately sure!
    pub unsafe fn insert_new_unchecked(&mut self, new: Entry) {
        self.entries.push(new)
    }

    pub fn insert(&mut self, value: Entry)
    where
        K: Copy,
    {
        if self.contains(value.key()) {
            ()
        } else {
            self.entries.push(value)
        }
    }
    pub fn insert_from_ref(&mut self, value: &Entry)
    where
        Entry: Clone,
        K: Copy,
    {
        if self.contains(value.key()) {
            ()
        } else {
            self.entries.push(value.clone())
        }
    }

    pub fn position(&self, key: K) -> Option<usize>
    where
        K: Copy,
    {
        self.entries.iter().position(|entry| entry.key() == key)
    }

    pub fn extend(
        &mut self,
        iter: impl Iterator<Item = Entry>,
    ) -> Result<(), InsertEntryRepeatError<Entry>>
    where
        K: Copy,
    {
        for v in iter {
            self.insert_new(v)?
        }
        Ok(())
    }

    pub fn extend_from_other(&mut self, other: Self) -> Result<(), InsertEntryRepeatError<Entry>>
    where
        K: Copy,
    {
        for v in other.entries {
            self.insert_new(v)?
        }
        Ok(())
    }

    pub fn extend_from_ref(&mut self, other: &Self)
    where
        Entry: Clone,
        K: Copy,
    {
        for entry in &other.entries {
            self.insert_from_ref(entry)
        }
    }

    pub fn toggle(&mut self, key: K)
    where
        Entry: DefaultVecMapEntry<K>,
        K: Copy,
    {
        if let Some(position) = self.entries.iter().position(|entry| entry.key() == key) {
            self.entries.remove(position);
        } else {
            self.entries.push(Entry::default_from_key(key))
        }
    }
}

impl<K, V> VecPairMap<K, V> {
    pub fn get_value(&self, key: K) -> Option<&V>
    where
        K: Copy + Eq,
    {
        self.get_entry(key).map(|(_, v)| v)
    }

    pub fn get_value_mut_or_insert_default(&mut self, key: K) -> &mut V
    where
        K: Eq,
        V: Default,
    {
        match self.entries.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => unsafe { husky_wild_utils::arb_lifetime_mut(&mut entry.1) },
            None => {
                self.entries.push((key, V::default()));
                &mut unsafe { self.entries.last_mut().unwrap_unchecked() }.1
            }
        }
    }

    pub fn get_value_mut_or_insert(&mut self, key: K, v: V) -> &mut V
    where
        K: Eq,
    {
        match self.entries.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => unsafe { husky_wild_utils::arb_lifetime_mut(&mut entry.1) },
            None => {
                self.entries.push((key, v));
                &mut unsafe { self.entries.last_mut().unwrap_unchecked() }.1
            }
        }
    }

    pub fn get_value_mut_or_insert_with(&mut self, key: K, f: impl FnOnce() -> V) -> &mut V
    where
        K: Copy + Eq,
    {
        match self.entries.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => unsafe { husky_wild_utils::arb_lifetime_mut(&mut entry.1) },
            None => {
                self.entries.push((key, f()));
                &mut unsafe { self.entries.last_mut().unwrap_unchecked() }.1
            }
        }
    }

    pub fn get_value_copied_or_insert_with(&mut self, key: K, f: impl FnOnce() -> V) -> V
    where
        K: Copy + Eq,
        V: Copy,
    {
        match self.entries.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => entry.1,
            None => {
                let v = f();
                self.entries.push((key, v));
                v
            }
        }
    }

    #[inline(always)]
    pub fn update_value_or_insert(&mut self, key: K, update: impl FnOnce(&mut V), v: V)
    where
        K: Copy + Eq,
    {
        match self.entries.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => update(&mut entry.1),
            None => self.entries.push((key, v)),
        }
    }

    #[inline(always)]
    pub fn update_value_or_insert_with(
        &mut self,
        key: K,
        update: impl FnOnce(&mut V),
        f: impl FnOnce() -> V,
    ) where
        K: Copy + Eq,
    {
        match self.entries.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => update(&mut entry.1),
            None => self.entries.push((key, f())),
        }
    }
}

impl<K, Entry> VecMap<Entry>
where
    K: PartialEq + Eq + std::fmt::Debug,
    Entry: AsVecMapEntry<K = K>,
{
    pub fn from_iter_assuming_no_repetitions<T: IntoIterator<Item = Entry>>(
        iter: T,
    ) -> Result<Self, InsertEntryRepeatError<Entry>>
    where
        K: Copy,
    {
        let mut map = Self::default();
        for v in iter {
            // ignore errors
            map.insert_new(v)?
        }
        Ok(map)
    }

    /// use this when you are absoluately sure!
    pub unsafe fn from_iter_assuming_no_repetitions_unchecked<T: IntoIterator<Item = Entry>>(
        iter: T,
    ) -> Self {
        let mut map = Self::default();
        for v in iter {
            map.insert_new_unchecked(v)
        }
        map
    }

    /// if there are repetitive keys, take the first value
    /// deprecated
    pub fn from_iter_ignoring_following_repetitions<T: IntoIterator<Item = Entry>>(iter: T) -> Self
    where
        K: Copy,
    {
        let mut map = Self::default();
        for v in iter {
            // ignore errors
            let _ = map.insert_new(v);
        }
        map
    }
}

impl<V> Deref for VecMap<V> {
    type Target = [V];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl<V> Default for VecMap<V> {
    fn default() -> Self {
        Self { entries: vec![] }
    }
}

impl<K, V> std::ops::Index<K> for VecMap<V>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    V: AsVecMapEntry<K = K>,
{
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.get_entry(index).unwrap()
    }
}

impl<K, V> std::ops::IndexMut<K> for VecMap<V>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    V: AsVecMapEntry<K = K>,
{
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
