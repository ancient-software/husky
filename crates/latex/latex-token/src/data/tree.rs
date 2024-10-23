#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxCodeTokenData {
    LeftDelimiter,
    Pos,
    Word,
    RightDelimiter,
}
