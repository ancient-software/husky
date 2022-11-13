use crate::*;
use interner::{Internable, Interner};



pub type TermInterner = Interner<Term>;

impl Internable for Term {
    type Ref<'a> = TermRef<'a>;

    type Interned = TermItd;

    fn new_itr() -> Interner<Self> {
        Interner::new_empty()
    }

    fn try_direct(&self) -> Option<Self::Interned> {
        match self {
            Term::Atom(atom) => Some(TermItd(TermRef::Atom(*atom))),
            _ => None,
        }
    }

    fn itd_to_borrowed(_itd: Self::Interned) -> Self::Ref<'static> {
        todo!()
    }

    fn as_ref<'a>(&'a self) -> Self::Ref<'a> {
        match self {
            Term::Atom(atom) => TermRef::Atom(*atom),
            Term::Curry(curry) => TermRef::Curry(curry),
            Term::Abstraction(abs) => TermRef::Abstraction(abs),
            Term::Application(app) => TermRef::Application(app),
            Term::Subentity(subentity) => TermRef::Subentity(subentity),
            Term::TraitImpl(trait_impl) => TermRef::TraitImpl(trait_impl),
        }
    }

    fn new_itd(&'static self, _id: usize) -> Self::Interned {
        TermItd(self.as_ref())
    }

    fn try_direct_from_ref<'a>(_r: Self::Ref<'a>) -> Option<Self::Interned> {
        todo!()
    }

    unsafe fn cast_to_static_ref<'a>(_r: Self::Ref<'a>) -> Self::Ref<'static> {
        todo!()
    }
}

pub fn new_term_itr() -> TermInterner {
    TermInterner::new_empty()
}

pub trait InternTerm {
    fn term_itr(&self) -> &TermInterner;

    fn it_term(&self, term: Term) -> TermItd {
        self.term_itr().intern(term)
    }
}

impl From<i32> for TermItd {
    fn from(value: i32) -> Self {
        TermItd(TermRef::Atom(value.into()))
    }
}

impl From<i64> for TermItd {
    fn from(value: i64) -> Self {
        TermItd(TermRef::Atom(value.into()))
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct TermItd(TermRef<'static>);

impl std::ops::Deref for TermItd {
    type Target = TermRef<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Debug for TermItd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::fmt::Display for TermItd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(target_arch = "x86_64")]
impl std::hash::Hash for TermItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe { std::mem::transmute::<Self, [u8; 24]>(*self) }.hash(state)
    }
}

#[cfg(target_arch = "x86")]
impl std::hash::Hash for TermItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // HELP
        todo!()
    }
}

#[cfg(target_arch = "mips")]
impl std::hash::Hash for TermItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // HELP
        todo!()
    }
}

#[cfg(target_arch = "powerpc")]
impl std::hash::Hash for TermItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // HELP
        todo!()
    }
}

#[cfg(target_arch = "powerpc64")]
impl std::hash::Hash for TermItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // HELP
        todo!()
    }
}

#[cfg(target_arch = "arm")]
impl std::hash::Hash for TermItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // HELP
        todo!()
    }
}

#[cfg(target_arch = "aarch64")]
impl std::hash::Hash for TermItd {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // HELP
        todo!()
    }
}

impl TermItd {
    pub(crate) fn borrowed(self) -> TermRef<'static> {
        self.0
    }
}
