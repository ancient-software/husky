#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Precedence {
    ScopeResolution = 240,
    Application = 230,
    Prefix = 220,
    As = 210,
    Power = 200,
    Multiplicative = 160,
    Additive = 150,
    Shift = 120,
    OrdComparison = 111,
    EqComparison = 110,
    BitAnd = 102,
    BitXor = 101,
    BitOr = 100,
    And = 90,
    Or = 80,
    Is = 60,
    Be = 40,
    /// means `->`
    Curry = 30,
    KeyedArgument = 22,
    ListItem = 21,
    Closure = 20,
    Method = 12,
    Assign = 11,
    List = 1,
    None = 0,
}

#[cfg(test)]
#[test]
fn test_precedence_order() {
    use husky_check_utils::should;

    should!(Precedence::Power < Precedence::Prefix);
}

pub trait HasPrecedence: Copy {
    fn precedence(self) -> Precedence;
}
