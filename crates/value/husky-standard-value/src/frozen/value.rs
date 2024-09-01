use super::*;
use husky_value_protocol::presentation::EnumUnitValuePresenter;

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
///
/// we use Arc for everything on heap to reduce clone costs
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum FrozenValue {
    /// useful for snapshot caching on stack
    None,
    Uninit,
    Invalid,
    Moved,
    Unit(()),
    Bool(bool),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    R8(u8),
    R16(u16),
    R32(u32),
    R64(u64),
    R128(u128),
    RSize(usize),
    F32(f32),
    F64(f64),
    StringLiteral(StringLiteralId),
    EnumUsize {
        index: usize,
        presenter: EnumUnitValuePresenter,
    },
    Box(Arc<dyn FrozenDyn>),
    Leash(&'static dyn StaticDyn),
    SizedRef(Arc<dyn FrozenDyn>),
    SizedRefMut(Arc<dyn FrozenDyn>),
    OptionBox(Option<Arc<dyn FrozenDyn>>),
    OptionLeash(Option<&'static dyn StaticDyn>),
    OptionSizedRef(Option<Arc<dyn FrozenDyn>>),
    OptionSizedRefMut(Option<Arc<dyn FrozenDyn>>),
    Intrinsic(Arc<dyn FrozenDyn>),
}

impl Value {
    pub unsafe fn freeze(&self) -> FrozenValue {
        match self {
            Value::Uninit => todo!(),
            Value::Moved => FrozenValue::Moved,
            Value::Invalid => FrozenValue::Invalid,
            Value::Unit(_) => FrozenValue::Unit(()),
            Value::Bool(val) => FrozenValue::Bool(*val),
            Value::Char(val) => FrozenValue::Char(*val),
            Value::I8(val) => FrozenValue::I8(*val),
            Value::I16(val) => FrozenValue::I16(*val),
            Value::I32(val) => FrozenValue::I32(*val),
            Value::I64(val) => FrozenValue::I64(*val),
            Value::I128(val) => FrozenValue::I128(*val),
            Value::ISize(val) => FrozenValue::ISize(*val),
            Value::U8(val) => FrozenValue::U8(*val),
            Value::U16(val) => FrozenValue::U16(*val),
            Value::U32(val) => FrozenValue::U32(*val),
            Value::U64(val) => FrozenValue::U64(*val),
            Value::U128(val) => FrozenValue::U128(*val),
            Value::USize(val) => FrozenValue::USize(*val),
            Value::R8(val) => FrozenValue::R8(*val),
            Value::R16(val) => FrozenValue::R16(*val),
            Value::R32(val) => FrozenValue::R32(*val),
            Value::R64(val) => FrozenValue::R64(*val),
            Value::R128(val) => FrozenValue::R128(*val),
            Value::RSize(val) => FrozenValue::RSize(*val),
            Value::F32(val) => FrozenValue::F32(*val),
            Value::F64(val) => FrozenValue::F64(*val),
            Value::StringLiteral(id) => FrozenValue::StringLiteral(*id),
            &Value::EnumUnit { index, presenter } => FrozenValue::EnumUsize { index, presenter },
            Value::Owned(slf) => todo!(),
            Value::Leash(_) => todo!(),
            Value::Ref(_) => todo!(),
            Value::Mut(_) => todo!(),
            Value::OptionBox(_) => todo!(),
            Value::OptionLeash(_) => todo!(),
            Value::OptionSizedRef(_) => todo!(),
            Value::OptionSizedMut(_) => todo!(),
        }
    }
}
