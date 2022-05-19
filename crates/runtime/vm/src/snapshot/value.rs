use crate::*;

#[derive(Clone)]
pub enum StackValueSnapshot<'eval> {
    Primitive(CopyableValue),
    GlobalPure(Arc<dyn AnyValueDyn<'eval>>),
    Boxed(OwnedValue<'eval>),
    Ref {
        value: Arc<dyn AnyValueDyn<'eval>>,
        owner: StackIdx,
        gen: MutRefGenerator,
    },
    MutRef {
        value: Arc<dyn AnyValueDyn<'eval>>,
        owner: StackIdx,
        gen: MutRefGenerator,
    },
    Uninitialized,
}

impl<'eval> StackValueSnapshot<'eval> {
    pub fn any_ref(&self) -> &dyn AnyValueDyn<'eval> {
        match self {
            StackValueSnapshot::Primitive(value) => value.any_ref(),
            StackValueSnapshot::GlobalPure(value) => &**value,
            StackValueSnapshot::Boxed(boxed_value) => boxed_value.any_ref(),
            StackValueSnapshot::MutRef { value, .. } => &**value,
            StackValueSnapshot::Ref { value, .. } => &**value,
            StackValueSnapshot::Uninitialized => todo!(),
        }
    }
}

impl<'eval> std::fmt::Debug for StackValueSnapshot<'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackValueSnapshot::Primitive(arg0) => f
                .debug_tuple("StackValueSnapshot::Primitive")
                .field(arg0)
                .finish(),
            StackValueSnapshot::MutRef { value, owner, .. } => f
                .debug_struct("StackValueSnapshot::MutRef")
                .field("value", value)
                .field("owner", owner)
                .finish(),
            StackValueSnapshot::GlobalPure(value) => f
                .debug_struct("StackValueSnapshot::GlobalPure")
                .field("value", value)
                .finish(),
            StackValueSnapshot::Boxed(value) => f
                .debug_struct("StackValueSnapshot::Boxed")
                .field("value", value)
                .finish(),
            StackValueSnapshot::Ref { value, owner, gen } => todo!(),
            StackValueSnapshot::Uninitialized => todo!(),
        }
    }
}

impl<'eval> From<CopyableValue> for StackValueSnapshot<'eval> {
    fn from(value: CopyableValue) -> Self {
        Self::Primitive(value)
    }
}

impl<'stack, 'eval: 'stack> Into<StackValue<'stack, 'eval>> for &StackValueSnapshot<'eval> {
    fn into(self) -> StackValue<'stack, 'eval> {
        match self {
            StackValueSnapshot::Primitive(value) => StackValue::Copyable(*value),
            StackValueSnapshot::MutRef { owner, gen, .. } => todo!(),
            StackValueSnapshot::GlobalPure(value) => StackValue::GlobalPure(value.clone()),
            StackValueSnapshot::Boxed(value) => StackValue::Owned(value.clone()),
            StackValueSnapshot::Ref { value, owner, gen } => todo!(),
            StackValueSnapshot::Uninitialized => todo!(),
        }
    }
}
