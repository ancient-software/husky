use super::*;

/// A read-only [`Signal`].
#[derive(Default)]
pub struct ReadSignal<T> {
    pub(super) value: RefCell<Rc<T>>,
    pub(super) emitter: SignalEmitter,
}

impl<T> ReadSignal<T> {
    /// Get the current value of the state. When called inside a reactive scope, calling this will
    /// add itself to the scope's dependencies.
    ///
    /// # Example
    /// ```rust
    /// # use sycamore_reactive::*;
    /// # create_scope_immediate(|cx| {
    /// let state = create_signal(cx, 0);
    /// assert_eq!(*state.get(), 0);
    ///
    /// state.set(1);
    /// assert_eq!(*state.get(), 1);
    /// # });
    /// ```
    #[must_use = "to only subscribe the signal without using the value, use .track() instead"]
    pub fn get(&self) -> Rc<T> {
        self.emitter.track();
        self.value.borrow().clone()
    }

    /// Get the current value of the state without Rc.
    pub fn get_cloned(&self) -> T
    where
        T: Clone,
    {
        (**self.value.borrow()).clone()
    }

    /// Get the current value of the state, without tracking this as a dependency if inside a
    /// reactive context.
    ///
    /// # Example
    ///
    /// ```
    /// # use sycamore_reactive::*;
    /// # create_scope_immediate(|cx| {
    /// let state = create_signal(cx, 1);
    /// let double = create_memo(cx, || *state.get_untracked() * 2);
    /// assert_eq!(*double.get(), 2);
    ///
    /// state.set(2);
    /// // double value should still be old value because state was untracked
    /// assert_eq!(*double.get(), 2);
    /// # });
    /// ```
    #[must_use = "discarding the returned value does nothing"]
    pub fn get_untracked(&self) -> Rc<T> {
        self.value.borrow().clone()
    }

    /// Creates a mapped [`ReadSignal`]. This is equivalent to using
    /// [`create_memo`].
    ///
    /// # Example
    /// ```rust
    /// # use sycamore_reactive::*;
    /// # create_scope_immediate(|cx| {
    /// let state = create_signal(cx, 1);
    /// let double = state.map(cx, |&x| x * 2);
    /// assert_eq!(*double.get(), 2);
    ///
    /// state.set(2);
    /// assert_eq!(*double.get(), 4);
    /// # });
    /// ```
    #[must_use]
    pub fn map<'a, U>(
        &'a self,
        cx: Scope<'a>,
        mut f: impl FnMut(&T) -> U + 'a,
    ) -> &'a ReadSignal<U> {
        create_memo(cx, move || f(&self.get()))
    }

    /// When called inside a reactive scope, calling this will add itself to the scope's
    /// dependencies.
    ///
    /// To both track and get the value of the signal, use [`ReadSignal::get`] instead.
    pub fn track(&self) {
        self.emitter.track();
    }
}
