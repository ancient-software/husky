mod any;
mod copyable;
mod enum_kind;
mod eval;
mod member;
mod owned;
mod ref_;
mod xml;

pub use any::*;
pub use copyable::*;
pub use enum_kind::*;
pub use eval::*;
pub use member::*;
pub use owned::*;
pub use ref_::*;
pub use xml::*;

use crate::*;
use print_utils::{msg_once, p, ps};
use std::fmt::Write;
use std::sync::Arc;
use word::CustomIdentifier;

// the primary concerns are safety and stability
// this whole vm thing will be replaced by JIT for fast evaluation purposes
// so we don't need to worry too much about speed here
pub enum __TempValue<'temp, 'eval: 'temp> {
    Moved,
    Copyable(CopyableValue),
    OwnedEval(__OwnedValue<'eval, 'eval>),
    OwnedTemp(__OwnedValue<'temp, 'eval>),
    EvalPure(Arc<dyn AnyValueDyn<'eval> + 'eval>),
    EvalRef(EvalRef<'eval>),
    TempRefEval(&'temp (dyn AnyValueDyn<'eval> + 'eval)),
    TempRefTemp(&'temp (dyn AnyValueDyn<'eval> + 'temp)),
    TempRefMutEval {
        value: &'temp mut (dyn AnyValueDyn<'eval> + 'eval),
        owner: VMStackIdx,
        gen: MutRefGenerator,
    },
    TempRefMutTemp {
        value: &'temp mut (dyn AnyValueDyn<'eval> + 'temp),
        owner: VMStackIdx,
        gen: MutRefGenerator,
    },
}

pub type MutRefGenerator = ();

impl<'temp, 'eval: 'temp> std::fmt::Debug for __TempValue<'temp, 'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            __TempValue::Copyable(arg0) => {
                f.write_str("Primitive ")?;
                arg0.fmt(f)
            }
            __TempValue::OwnedEval(arg0) => f.debug_tuple("OwnedEval").field(arg0).finish(),
            __TempValue::OwnedTemp(arg0) => f.debug_tuple("OwnedTemp").field(arg0).finish(),
            __TempValue::EvalPure(arg0) => f.debug_tuple("EvalPure").field(arg0).finish(),
            __TempValue::EvalRef(arg0) => f.debug_tuple("EvalRef").field(arg0).finish(),
            __TempValue::TempRefEval(value) => f.debug_tuple("TempRefEval").field(value).finish(),
            __TempValue::TempRefTemp(value) => f.debug_tuple("TempRefTemp").field(value).finish(),
            __TempValue::TempRefMutEval { value, .. } => f
                .debug_tuple("CopyableMutOrTempRefMutEval")
                .field(value)
                .finish(),
            __TempValue::TempRefMutTemp { value, owner, gen } => {
                f.debug_tuple("TempRefMutTemp").field(value).finish()
            }
            __TempValue::Moved => f.write_str("Moved"),
        }
    }
}

impl<'temp, 'eval: 'temp> __TempValue<'temp, 'eval> {
    pub fn print_short(&self) -> String {
        let mut result = String::new();
        match self {
            __TempValue::Moved => result.push_str("Moved"),
            __TempValue::Copyable(value) => {
                result.push_str("Primitive ");
                result.push_str(&value.any_ref().print_short())
            }
            __TempValue::OwnedEval(value) => {
                result.push_str("Boxed ");
                result.push_str(&value.any_ref().print_short())
            }
            __TempValue::OwnedTemp(value) => {
                result.push_str("OwnedTemp ");
                result.push_str(&value.any_ref().print_short());
            }
            __TempValue::EvalPure(value) => {
                result.push_str("EvalPure ");
                result.push_str(&value.print_short())
            }
            __TempValue::EvalRef(value) => {
                result.push_str("EvalRef ");
                result.push_str(&value.print_short());
            }
            __TempValue::TempRefEval(value) => {
                result.push_str("TempRefEval ");
                result.push_str(&value.print_short());
            }
            __TempValue::TempRefTemp(value) => {
                result.push_str("TempRefTemp ");
                result.push_str(&value.print_short());
            }
            __TempValue::TempRefMutEval { value, owner, gen } => {
                result.push_str("CopyableOrTempMutEval ");
                result.push_str(&value.print_short());
                write!(result, " Owner({:?}) ", owner);
            }
            __TempValue::TempRefMutTemp { value, owner, gen } => {
                result.push_str("TempRefMutTemp ");
                result.push_str(&value.print_short());
                write!(result, " Owner({:?}) ", owner);
            }
        }
        result
    }
}

impl<'temp, 'eval: 'temp> From<CopyableValue> for __TempValue<'temp, 'eval> {
    fn from(value: CopyableValue) -> Self {
        __TempValue::Copyable(value)
    }
}

impl<'temp, 'eval: 'temp> From<&CopyableValue> for __TempValue<'temp, 'eval> {
    fn from(value: &CopyableValue) -> Self {
        __TempValue::Copyable(*value)
    }
}

impl<'temp, 'eval: 'temp> __TempValue<'temp, 'eval> {
    pub fn from_eval(eval_value: EvalValue<'eval>) -> __EvalResult<Self> {
        Ok(match eval_value {
            EvalValue::Copyable(value) => Self::Copyable(value),
            EvalValue::Owned(_) => todo!(),
            EvalValue::EvalPure(value) => __TempValue::EvalPure(value),
            EvalValue::EvalRef(value) => Self::EvalRef(value),
            EvalValue::Undefined => todo!(),
        })
    }

    pub fn into_eval(self) -> EvalValue<'eval> {
        match self {
            __TempValue::Copyable(copyable_value) => EvalValue::Copyable(copyable_value),
            __TempValue::OwnedEval(boxed_value) => EvalValue::Owned(boxed_value),
            __TempValue::EvalPure(_) => todo!(),
            __TempValue::EvalRef(value) => EvalValue::EvalRef(value),
            __TempValue::TempRefEval { .. }
            | __TempValue::TempRefMutEval { .. }
            | __TempValue::Moved => {
                panic!()
            }
            __TempValue::OwnedTemp(_) => todo!(),
            __TempValue::TempRefTemp(_) => todo!(),
            __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub fn eval(&self) -> EvalValue<'eval> {
        match self {
            __TempValue::Copyable(primitive_value) => EvalValue::Copyable(*primitive_value),
            __TempValue::OwnedEval(boxed_value) => EvalValue::Owned(boxed_value.clone()),
            __TempValue::EvalPure(_) => todo!(),
            __TempValue::EvalRef(value) => EvalValue::EvalRef(*value),
            __TempValue::TempRefEval(value) => EvalValue::Owned(value.clone_into_box_dyn().into()),
            __TempValue::OwnedTemp(_) => todo!(),
            __TempValue::TempRefTemp(_) => todo!(),
            __TempValue::TempRefMutEval { value, owner, gen } => {
                EvalValue::Owned(value.clone_into_box_dyn().into())
            }
            __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
            _ => panic!(),
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            __TempValue::Copyable(v) => v.to_bool(),
            __TempValue::TempRefMutEval { value, owner, gen } => {
                value.take_copyable_dyn().to_bool()
            }
            _ => panic!(),
        }
    }

    pub fn into_member(&mut self) -> MemberValue<'eval> {
        match self {
            __TempValue::Copyable(primitive_value) => MemberValue::Copyable(*primitive_value),
            __TempValue::OwnedEval(boxed_value) => {
                match std::mem::replace(self, __TempValue::Moved) {
                    __TempValue::OwnedEval(boxed_value) => MemberValue::Boxed(boxed_value),
                    _ => panic!(),
                }
            }
            __TempValue::EvalPure(_) => todo!(),
            __TempValue::EvalRef(value) => MemberValue::EvalRef(*value),
            __TempValue::TempRefEval { .. }
            | __TempValue::TempRefMutEval { .. }
            | __TempValue::Moved => {
                panic!()
            }
            __TempValue::OwnedTemp(_) => todo!(),
            __TempValue::TempRefTemp(_) => todo!(),
            __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub(crate) unsafe fn bind(&mut self, binding: Binding, stack_idx: VMStackIdx) -> Self {
        match binding {
            Binding::EvalRef => self.bind_eval_ref(),
            Binding::TempRef => self.bind_temp_ref(),
            Binding::TempRefMut => self.bind_ref_mut(stack_idx),
            Binding::Move => self.bind_move(),
            Binding::Copy => self.bind_copy(),
        }
    }

    unsafe fn bind_eval_ref(&self) -> Self {
        match self {
            __TempValue::EvalRef(value) => __TempValue::EvalRef(*value),
            _ => panic!(),
        }
    }

    unsafe fn bind_temp_ref(&self) -> Self {
        match self {
            __TempValue::Moved => panic!(),
            __TempValue::Copyable(_) => panic!(),
            __TempValue::OwnedEval(value) => {
                let ptr: *const dyn AnyValueDyn = value.any_ptr();
                __TempValue::TempRefEval(&*ptr)
            }
            __TempValue::OwnedTemp(value) => {
                let ptr: *const dyn AnyValueDyn = value.any_ptr();
                __TempValue::TempRefTemp(&*ptr)
            }
            __TempValue::EvalPure(value) => {
                let ptr: *const dyn AnyValueDyn = &**value;
                __TempValue::TempRefEval(&*ptr)
            }
            __TempValue::EvalRef(value) => __TempValue::TempRefEval(value.0),
            __TempValue::TempRefEval(value) => __TempValue::TempRefEval(*value),
            __TempValue::TempRefTemp(value) => __TempValue::TempRefTemp(*value),
            __TempValue::TempRefMutEval { value, owner, gen } => {
                let ptr: *const (dyn AnyValueDyn<'eval> + 'eval) = *value;
                __TempValue::TempRefEval(&*ptr)
            }
            __TempValue::TempRefMutTemp { value, owner, gen } => {
                let ptr: *const (dyn AnyValueDyn<'eval> + 'temp) = *value;
                __TempValue::TempRefTemp(&*ptr)
            }
        }
    }

    fn bind_copy(&self) -> Self {
        match self {
            __TempValue::Copyable(value) => __TempValue::Copyable(*value),
            __TempValue::EvalRef(value) => value.take_copyable_dyn().into(),
            __TempValue::TempRefMutEval { value, owner, gen } => value.take_copyable_dyn().into(),
            _ => panic!(),
        }
    }

    unsafe fn bind_ref_mut(&mut self, stack_idx: VMStackIdx) -> __TempValue<'temp, 'eval> {
        match self {
            __TempValue::Copyable(value) => {
                let ptr: *mut dyn AnyValueDyn<'eval> = value.any_mut();
                __TempValue::TempRefMutEval {
                    value: &mut *ptr,
                    owner: stack_idx,
                    gen: (),
                }
            }
            __TempValue::OwnedEval(value) => {
                let ptr: *mut dyn AnyValueDyn = &mut *value.any_mut_ptr();
                __TempValue::TempRefMutEval {
                    value: &mut *ptr,
                    owner: stack_idx,
                    gen: (),
                }
            }
            __TempValue::OwnedTemp(value) => __TempValue::TempRefMutTemp {
                value: &mut *value.any_mut_ptr(),
                owner: stack_idx,
                gen: (),
            },
            __TempValue::TempRefMutTemp { value, owner, gen } => {
                let ptr: *mut dyn AnyValueDyn = *value;
                __TempValue::TempRefMutTemp {
                    value: &mut *ptr,
                    owner: *owner,
                    gen: *gen,
                }
            }
            _ => panic!(),
        }
    }

    unsafe fn pure(&self, stack_idx: VMStackIdx) -> Self {
        match self {
            __TempValue::Copyable(value) => __TempValue::Copyable(*value),
            __TempValue::OwnedEval(value) => __TempValue::TempRefEval(&*value.any_ptr()),
            __TempValue::EvalPure(value) => __TempValue::EvalPure(value.clone()),
            __TempValue::EvalRef(value) => __TempValue::EvalRef(*value),
            __TempValue::TempRefEval(value) => __TempValue::TempRefEval(*value),
            __TempValue::TempRefMutEval { .. } => todo!(),
            __TempValue::Moved => todo!(),
            __TempValue::OwnedTemp(_) => todo!(),
            __TempValue::TempRefTemp(_) => todo!(),
            __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn bind_move(&mut self) -> Self {
        match self {
            __TempValue::Moved => todo!(),
            __TempValue::Copyable(value) => __TempValue::Copyable(*value),
            __TempValue::OwnedEval(_) => std::mem::replace(self, __TempValue::Moved),
            __TempValue::EvalPure(_) => todo!(),
            __TempValue::EvalRef(_) => todo!(),
            __TempValue::TempRefEval { .. } => todo!(),
            __TempValue::TempRefMutEval { value, owner, gen } => todo!(),
            __TempValue::OwnedTemp(_) => todo!(),
            __TempValue::TempRefTemp(_) => todo!(),
            __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn bind_return(&mut self) -> Self {
        match self {
            __TempValue::Moved => todo!(),
            __TempValue::Copyable(value) => Self::Copyable(*value),
            __TempValue::OwnedEval(_) => std::mem::replace(self, __TempValue::Moved),
            __TempValue::EvalPure(_) => todo!(),
            __TempValue::EvalRef(_) => todo!(),
            __TempValue::TempRefEval { .. } => todo!(),
            __TempValue::TempRefMutEval { value, owner, gen } => todo!(),
            __TempValue::OwnedTemp(_) => todo!(),
            __TempValue::TempRefTemp(_) => todo!(),
            __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    unsafe fn borrow_mut(&mut self, self_stack_idx: VMStackIdx) -> Self {
        Self::TempRefMutEval {
            value: &mut *self.any_mut_ptr(),
            owner: self.owner(self_stack_idx).unwrap(),
            gen: (),
        }
    }

    fn owner(&self, self_stack_idx: VMStackIdx) -> Option<VMStackIdx> {
        match self {
            __TempValue::Copyable(_) | __TempValue::OwnedEval(_) => Some(self_stack_idx),
            __TempValue::EvalRef(_) | __TempValue::EvalPure(_) => None,
            __TempValue::TempRefEval { .. } => todo!(),
            __TempValue::TempRefMutEval { owner, .. } => Some(*owner),
            __TempValue::Moved => todo!(),
            __TempValue::OwnedTemp(_) => todo!(),
            __TempValue::TempRefTemp(_) => todo!(),
            __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub fn any_ref(&self) -> &(dyn AnyValueDyn<'eval> + 'eval) {
        {
            match self {
                __TempValue::Copyable(value) => match value {
                    CopyableValue::I32(value) => value,
                    CopyableValue::F32(value) => value,
                    CopyableValue::B32(value) => value,
                    CopyableValue::B64(value) => value,
                    CopyableValue::Bool(value) => value,
                    CopyableValue::Void(_) => todo!(),
                    CopyableValue::EnumKind(value) => value,
                },
                __TempValue::OwnedEval(value) => value.any_ref(),
                __TempValue::EvalPure(value) => (&**value),
                __TempValue::EvalRef(value) => value.0,
                __TempValue::TempRefEval(value) => *value,
                __TempValue::TempRefMutEval { value, .. } => *value,
                __TempValue::Moved => todo!(),
                __TempValue::OwnedTemp(_) => todo!(),
                __TempValue::TempRefTemp(_) => todo!(),
                __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
            }
        }
    }

    pub fn any_temp_ref(&self) -> &(dyn AnyValueDyn<'eval> + 'temp) {
        {
            match self {
                __TempValue::Copyable(value) => match value {
                    CopyableValue::I32(value) => value,
                    CopyableValue::F32(value) => value,
                    CopyableValue::B32(value) => value,
                    CopyableValue::B64(value) => value,
                    CopyableValue::Bool(value) => value,
                    CopyableValue::Void(_) => todo!(),
                    CopyableValue::EnumKind(value) => value,
                },
                __TempValue::OwnedEval(value) => value.any_ref(),
                __TempValue::EvalPure(value) => (&**value),
                __TempValue::EvalRef(_) => todo!(),
                __TempValue::TempRefEval(value) => *value,
                __TempValue::TempRefMutEval { value, .. } => *value,
                __TempValue::Moved => todo!(),
                __TempValue::OwnedTemp(_) => todo!(),
                __TempValue::TempRefTemp(value) => *value,
                __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
            }
        }
    }

    fn any_mut_ptr(&mut self) -> *mut (dyn AnyValueDyn<'eval> + 'eval) {
        {
            match self {
                __TempValue::Copyable(value) => match value {
                    CopyableValue::I32(value) => value,
                    CopyableValue::F32(value) => value,
                    CopyableValue::B32(value) => value,
                    CopyableValue::B64(value) => value,
                    CopyableValue::Bool(value) => value,
                    CopyableValue::Void(_) => todo!(),
                    CopyableValue::EnumKind(value) => value,
                },
                __TempValue::OwnedEval(value) => value.any_mut_ptr(),
                __TempValue::TempRefMutEval { value, .. } => *value,
                __TempValue::TempRefEval { .. } => {
                    panic!("TempRef cannot be mutated, this is a bug.")
                }
                __TempValue::EvalPure(_) => panic!("GlobalPure cannot be mutated, this is a bug."),
                __TempValue::EvalRef(_) => panic!("EvalRef cannot be mutated, this is a bug."),
                __TempValue::Moved => panic!("Move cannot be mutated, this is a bug."),
                __TempValue::OwnedTemp(_) => todo!(),
                __TempValue::TempRefTemp(_) => todo!(),
                __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
            }
        }
    }

    pub fn downcast_ref<T: AnyValue<'eval>>(&self) -> &T {
        match self {
            __TempValue::Moved => todo!(),
            __TempValue::Copyable(_) => todo!(),
            __TempValue::OwnedEval(_) => todo!(),
            __TempValue::EvalPure(value) => value.downcast_ref(),
            __TempValue::EvalRef(value) => value.downcast_ref(),
            __TempValue::TempRefEval(value) => value.downcast_ref(),
            __TempValue::TempRefMutEval { value, .. } => value.downcast_ref(),
            __TempValue::OwnedTemp(_) => todo!(),
            __TempValue::TempRefTemp(_) => todo!(),
            __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub fn downcast_eval_ref<T: AnyValue<'eval>>(&self) -> &'eval T {
        match self {
            __TempValue::Moved => todo!(),
            __TempValue::Copyable(_) => todo!(),
            __TempValue::OwnedEval(_) => todo!(),
            __TempValue::EvalPure(value) => panic!(),
            __TempValue::EvalRef(value) => value.0.downcast_ref(),
            __TempValue::TempRefEval(value) => panic!(),
            __TempValue::TempRefMutEval { value, .. } => panic!(),
            __TempValue::OwnedTemp(_) => todo!(),
            __TempValue::TempRefTemp(_) => todo!(),
            __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub fn downcast_mut<T: AnyValue<'eval>>(&mut self) -> &mut T {
        match self {
            __TempValue::Moved => todo!(),
            __TempValue::Copyable(_) => todo!(),
            __TempValue::OwnedEval(_)
            | __TempValue::EvalPure(_)
            | __TempValue::EvalRef(_)
            | __TempValue::TempRefEval { .. } => {
                panic!()
            }
            __TempValue::TempRefMutEval { ref mut value, .. } => value.downcast_mut(),
            __TempValue::OwnedTemp(_) => todo!(),
            __TempValue::TempRefTemp(_) => todo!(),
            __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub fn downcast_mut_full<T: AnyValue<'eval>>(&mut self) -> (&'temp mut T, VMStackIdx, ()) {
        match self {
            __TempValue::Moved => todo!(),
            __TempValue::Copyable(_) => todo!(),
            __TempValue::OwnedEval(_)
            | __TempValue::EvalPure(_)
            | __TempValue::EvalRef(_)
            | __TempValue::TempRefEval { .. } => {
                panic!()
            }
            __TempValue::TempRefMutEval { value, owner, gen } => {
                let ptr: *mut T = value.downcast_mut();
                (unsafe { &mut *ptr }, *owner, *gen)
            }
            __TempValue::OwnedTemp(_) => todo!(),
            __TempValue::TempRefTemp(_) => todo!(),
            __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub fn take_copyable(&self) -> CopyableValue {
        match self {
            __TempValue::Copyable(value) => *value,
            __TempValue::TempRefMutEval { value, .. } => value.take_copyable_dyn(),
            _ => {
                p!(self);
                panic!("")
            }
        }
    }

    pub fn clone_into_stack(&self) -> __TempValue<'temp, 'eval> {
        match self {
            __TempValue::Moved => todo!(),
            __TempValue::Copyable(_) => todo!(),
            __TempValue::OwnedEval(_) => todo!(),
            __TempValue::EvalPure(value) => {
                __TempValue::OwnedEval(value.clone_into_box_dyn().into())
            }
            __TempValue::EvalRef(_) => todo!(),
            __TempValue::TempRefEval(value) => {
                __TempValue::OwnedEval(value.clone_into_box_dyn().into())
            }
            __TempValue::TempRefMutEval { value, owner, gen } => todo!(),
            __TempValue::OwnedTemp(_) => todo!(),
            __TempValue::TempRefTemp(_) => todo!(),
            __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn snapshot(&self) -> StackValueSnapshot<'eval> {
        match self {
            __TempValue::Copyable(value) => StackValueSnapshot::Copyable(*value),
            __TempValue::OwnedEval(value) => StackValueSnapshot::Owned(value.clone()),
            __TempValue::EvalPure(value) => StackValueSnapshot::EvalPure(value.clone()),
            __TempValue::EvalRef(value) => StackValueSnapshot::EvalRef(*value),
            __TempValue::TempRefEval(value) => {
                StackValueSnapshot::FullyOwnedRef(value.clone_into_arc_dyn())
            }
            __TempValue::TempRefMutEval { value, owner, gen } => {
                p!(value);
                todo!()
            }
            __TempValue::Moved => todo!(),
            __TempValue::OwnedTemp(_) => todo!(),
            __TempValue::TempRefTemp(_) => todo!(),
            __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub fn static_type_id(&self) -> std::any::TypeId {
        self.any_ref().static_type_id_dyn()
    }

    pub fn field(self, field_idx: usize, field_binding: Binding) -> __TempValue<'temp, 'eval> {
        msg_once!("ad hoc");
        match self {
            __TempValue::OwnedEval(boxed_value) => {
                let mut value: VirtualStruct = boxed_value.take().unwrap();
                value.take_field(field_idx)
            }
            __TempValue::EvalPure(_) => todo!(),
            __TempValue::EvalRef(value) => {
                let value: &VirtualStruct = value.downcast_ref();
                value.access_field(field_idx, field_binding)
            }
            __TempValue::TempRefEval(value) => {
                let value: &VirtualStruct = value.downcast_ref();
                value.access_field(field_idx, field_binding)
            }
            __TempValue::TempRefTemp(value) => {
                let value: &VirtualStruct = value.downcast_ref();
                value.access_field(field_idx, field_binding)
            }
            __TempValue::TempRefMutEval { value, owner, gen } => {
                let virtual_value: &mut VirtualStruct = value.downcast_mut();
                msg_once!("need cleaning");
                virtual_value.field_mut(field_idx, field_binding, owner)
            }
            __TempValue::OwnedTemp(_) => todo!(),
            __TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
            _ => panic!(),
        }
    }
}
