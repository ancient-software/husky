use std::sync::Arc;

use check_utils::should;

#[test]
fn test_eq() {
    let a = Arc::new(1);
    let b = Arc::new(1);
    should!(a == b);
}
