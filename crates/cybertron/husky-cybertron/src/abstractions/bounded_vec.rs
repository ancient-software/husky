#[derive(Debug, Clone, Copy)]
pub struct BoundedVec<T, const U: usize> {
    len: usize,
    data: [Option<T>; U],
}

impl<T, const U: usize> Default for BoundedVec<T, U> {
    fn default() -> Self {
        Self {
            len: 0,
            data: [(); U].map(|_| None),
        }
    }
}

impl<T, const U: usize> IntoIterator for BoundedVec<T, U> {
    type Item = T;
    type IntoIter = impl Iterator<Item = T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().filter_map(|opt| opt)
    }
}

impl<T: Copy, const U: usize> BoundedVec<T, U> {
    pub fn append(self, t: T) -> Self {
        assert!(self.len + 1 < U);
        let mut data = self.data;
        data[self.len] = Some(t);
        Self {
            len: self.len + 1,
            data,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().filter_map(|opt| opt.as_ref())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().filter_map(|opt| opt.as_mut())
    }

    #[cfg(test)]
    pub fn assert_eqs_slice(self, expected: &[T])
    where
        T: std::fmt::Debug + Eq,
    {
        assert_eq!(self.len, expected.len());
        for i in 0..self.len {
            assert_eq!(self.data[i], Some(expected[i]));
        }
        for i in self.len..U {
            assert!(self.data[i].is_none());
        }
    }

    pub fn is_empty(self) -> bool {
        self.len == 0
    }
}

#[test]
fn bounded_vec_append_works() {
    let mut v: BoundedVec<i32, 8> = Default::default();
    v.assert_eqs_slice(&[]);
    v = v.append(1);
    v.assert_eqs_slice(&[1]);
    v = v.append(2);
    v.assert_eqs_slice(&[1, 2]);
}
