#![feature(try_trait_v2_residual)]
#![feature(try_trait_v2)]

pub use MaybeResult::*;

use original_error::OriginalError;
use std::convert::Infallible;

/// composition of option and result
#[must_use]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MaybeResult<T, E> {
    JustOk(T),
    JustErr(E),
    Nothing,
}

impl<T, E> MaybeResult<T, E> {
    #[track_caller]
    pub fn expect(self, message: &str) -> T {
        match self {
            JustOk(t) => t,
            JustErr(_) => panic!("{message}"),
            Nothing => panic!("{message}"),
        }
    }

    pub fn map_err_or_none<E2>(self, f: impl FnOnce(Option<E>) -> E2) -> Result<T, E2> {
        match self {
            JustOk(t) => Ok(t),
            JustErr(e) => Err(f(Some(e))),
            Nothing => Err(f(None)),
        }
    }
}

impl<T, E> From<Result<T, E>> for MaybeResult<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(t) => JustOk(t),
            Err(e) => JustErr(e),
        }
    }
}

type MaybeResultResidual<E> = MaybeResult<Infallible, E>;

impl<T, E> std::ops::Try for MaybeResult<T, E> {
    type Output = T;

    type Residual = MaybeResultResidual<E>;

    fn from_output(t: Self::Output) -> Self {
        JustOk(t)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            JustOk(t) => std::ops::ControlFlow::Continue(t),
            JustErr(e) => std::ops::ControlFlow::Break(MaybeResultResidual::JustErr(e)),
            Nothing => std::ops::ControlFlow::Break(MaybeResultResidual::Nothing),
        }
    }
}

impl<T, E1, E2> std::ops::FromResidual<MaybeResultResidual<E1>> for MaybeResult<T, E2>
where
    E2: From<E1>,
{
    fn from_residual(residual: MaybeResultResidual<E1>) -> Self {
        match residual {
            JustOk(_) => unreachable!(),
            JustErr(e) => JustErr(e.into()),
            Nothing => Nothing,
        }
    }
}

impl<T, E1, E2> std::ops::FromResidual<Result<Infallible, E1>> for MaybeResult<T, E2>
where
    E2: From<E1>,
{
    fn from_residual(residual: Result<Infallible, E1>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => JustErr(e.into()),
        }
    }
}

impl<T, E> std::ops::FromResidual<Option<Infallible>> for MaybeResult<T, E> {
    fn from_residual(_residual: Option<Infallible>) -> Self {
        Nothing
    }
}

#[test]
fn maybe_result_works() {
    assert_eq!(
        || -> MaybeResult<i32, ()> {
            JustOk(0)?;
            JustOk(1)
        }(),
        JustOk(1)
    );
    assert_eq!(
        || -> MaybeResult<i32, ()> {
            JustErr(())?;
            JustOk(1)
        }(),
        JustErr(())
    );
    assert_eq!(
        || -> MaybeResult<i32, ()> {
            Nothing?;
            JustOk(1)
        }(),
        Nothing
    );
}

impl<T, E> From<Result<T, Option<E>>> for MaybeResult<T, E> {
    fn from(value: Result<T, Option<E>>) -> Self {
        match value {
            Ok(t) => JustOk(t),
            Err(Some(e)) => JustErr(e),
            Err(None) => Nothing,
        }
    }
}

impl<T, E> MaybeResult<T, E> {
    pub fn ok(self) -> Option<T> {
        match self {
            JustOk(t) => Some(t),
            JustErr(_) | Nothing => None,
        }
    }

    /// convert into `Result<Option<T>, E>`
    ///
    /// ```
    /// use maybe_result::*;
    /// let a: MaybeResult<i32, ()> = JustOk(1);
    /// assert_eq!(a.into_result_option(), Ok(Some(1)));
    /// let b: MaybeResult<i32, ()> = JustErr(());
    /// assert_eq!(b.into_result_option(), Err(()));
    /// let c: MaybeResult<i32, ()> = Nothing;
    /// assert_eq!(c.into_result_option(), Ok(None));
    /// ```
    pub fn into_result_option(self) -> Result<Option<T>, E> {
        match self {
            JustOk(t) => Ok(Some(t)),
            JustErr(e) => Err(e),
            Nothing => Ok(None),
        }
    }

    pub fn into_option_result(self) -> Option<Result<T, E>> {
        match self {
            JustOk(t) => Some(Ok(t)),
            JustErr(e) => Some(Err(e)),
            Nothing => None,
        }
    }

    pub fn into_result(self) -> Result<T, Option<E>> {
        match self {
            JustOk(t) => Ok(t),
            JustErr(e) => Err(Some(e)),
            Nothing => Err(None),
        }
    }

    pub fn into_result_or<OE>(self, nothing_e: OE) -> Result<T, <OE as OriginalError>::Error>
    where
        OE: OriginalError,
        E: Into<<OE as OriginalError>::Error>,
    {
        match self {
            JustOk(t) => Ok(t),
            JustErr(e) => Err(e.into()),
            Nothing => Err(nothing_e.into()),
        }
    }

    pub fn just_ok_as_ref(&self) -> MaybeResult<&T, E>
    where
        E: Copy,
    {
        match self {
            JustOk(t) => JustOk(t),
            JustErr(e) => JustErr(*e),
            Nothing => Nothing,
        }
    }

    pub fn just_ok_as_ref2<S: ?Sized>(&self) -> MaybeResult<&S, E>
    where
        E: Copy,
        T: AsRef<S>,
    {
        match self {
            JustOk(t) => JustOk(t.as_ref()),
            JustErr(e) => JustErr(*e),
            Nothing => Nothing,
        }
    }

    pub fn map<S>(self, f: impl FnOnce(T) -> S) -> MaybeResult<S, E> {
        match self {
            JustOk(t) => JustOk(f(t)),
            JustErr(e) => JustErr(e),
            Nothing => Nothing,
        }
    }
}

impl<T, E> std::ops::Residual<T> for MaybeResult<Infallible, E> {
    type TryType = MaybeResult<T, E>;
}

impl<A, E, V> FromIterator<MaybeResult<A, E>> for MaybeResult<V, E>
where
    V: FromIterator<A>,
{
    #[inline(always)]
    fn from_iter<T: IntoIterator<Item = MaybeResult<A, E>>>(iter: T) -> Self {
        iter.into_iter()
            .map(MaybeResult::into_result)
            .collect::<Result<V, Option<E>>>()
            .into()
    }
}
