mod context;

pub use context::*;

use crate::*;

/// representing term `x -> y`
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TermCurry {
    curry_variant: TermCurryVariant,
    x: Ty,
    y: Ty,
    // ty: Ty,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TermCurryVariant {
    Physical {
        physical_curry_kind: TermPhysicalCurryVariant,
        modifier: PhysicalParameterModifier,
    },
    Conceptual,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TermPhysicalCurryVariant {
    Fp,
    Closure {
        opt_ctx: Option<TermCurryContextItd>,
        lifetimes: Vec<Lifetime>,
    },
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Lifetime(TermItd);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PhysicalParameterModifier {
    None,
    Move,
    MoveMut,
}

impl TermCurry {
    pub fn curry_variant(&self) -> &TermCurryVariant {
        &self.curry_variant
    }

    pub fn x(&self) -> Ty {
        self.x
    }

    pub fn y(&self) -> Ty {
        self.y
    }
}

impl<'a> TermContext<'a> {
    pub(crate) fn curry(&self, curry_kind: TermCurryVariant, x: Ty, y: Ty) -> TermResult<Ty> {
        // TODO: check type
        Ty::new(
            self.it_term(
                TermCurry {
                    curry_variant: curry_kind,
                    x,
                    y,
                    // ty: Ty::new(self.sort(x.universe().max(y.universe())))?,
                }
                .into(),
            ),
        )
        // if self.ty_family(x)? == TyFamily::Monadic {
        //     return Err(TermError::MonadIsNotInput);
        // }
        // msg_once!("check compatibility of y");
    }
}

impl std::fmt::Display for TermCurry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.curry_variant() {
            curry::TermCurryVariant::Physical {
                physical_curry_kind: _,
                modifier: _,
            } => todo!(),
            curry::TermCurryVariant::Conceptual => {
                write!(f, "{} -> {}", self.x(), self.y())
            }
        }
    }
}

#[test]
fn test_curry() {
    let db = TermTestsDb::new();
    let menu = db.term_menu();
    let ctx = TermContext::new(&db, &menu);
    let i32_to_i32 = ctx
        .curry(TermCurryVariant::Conceptual, menu.i32(), menu.i32())
        .unwrap();
    assert_eq!(i32_to_i32.to_string(), "i32 -> i32");
    let i64_to_i64 = ctx
        .curry(TermCurryVariant::Conceptual, menu.i64(), menu.i64())
        .unwrap();
    assert_eq!(i64_to_i64.to_string(), "i64 -> i64");
    let f32_to_f32 = ctx
        .curry(TermCurryVariant::Conceptual, menu.f32(), menu.f32())
        .unwrap();
    assert_eq!(f32_to_f32.to_string(), "f32 -> f32");
    let f64_to_f64 = ctx
        .curry(TermCurryVariant::Conceptual, menu.f64(), menu.f64())
        .unwrap();
    assert_eq!(f64_to_f64.to_string(), "f64 -> f64");
    let b32_to_b32 = ctx
        .curry(TermCurryVariant::Conceptual, menu.b32(), menu.b32())
        .unwrap();
    assert_eq!(b32_to_b32.to_string(), "b32 -> b32");
    let b64_to_b64 = ctx
        .curry(TermCurryVariant::Conceptual, menu.b64(), menu.b64())
        .unwrap();
    assert_eq!(b64_to_b64.to_string(), "b64 -> b64");
    let bool_to_bool = ctx
        .curry(TermCurryVariant::Conceptual, menu.bool(), menu.bool())
        .unwrap();
    assert_eq!(bool_to_bool.to_string(), "bool -> bool");
}

impl Into<Term> for TermCurry {
    fn into(self) -> Term {
        Term::Curry(self)
    }
}
