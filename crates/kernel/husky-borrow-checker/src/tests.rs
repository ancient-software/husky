use crate::*;

#[test]
fn new_resource_works() {
    let mut symbol_registry = SymbolRegistry::default();
    let immutable0 = symbol_registry.new_variable();
    let lifetime1 = symbol_registry.new_lifetime();
    let mut borrow_table = DependencyTable::default();
    let mut time_machine = BorrowChecker::new(&borrow_table);
}

#[test]
fn new_borrow_works() {
    let mut symbol_registry = SymbolRegistry::default();
    let x = symbol_registry.new_variable();
    let a = symbol_registry.new_lifetime();
    let borrows = DependencyTable::default();
    let mut borrow_checker = BorrowChecker::new(&borrows);
    borrow_checker.exec(&BorrowInstruction::push(x));
    borrow_checker.exec(&BorrowInstruction::push(a));
    assert_eq!(borrow_checker.variable_state(x), &VariableState::Intact);
    assert_eq!(
        borrow_checker.lifetime_state(a),
        LifetimeState::Uninitialized
    );
    assert_eq!(borrow_checker.sim_borrow(x, a), Ok(()));
    assert_eq!(borrow_checker.lifetime_state(a), LifetimeState::Intact);
}

#[test]
fn new_borrow_err() {
    let mut symbol_registry = SymbolRegistry::default();
    let x = symbol_registry.new_variable();
    let a = symbol_registry.new_lifetime();
    let mut dependencies = DependencyTable::default();
    dependencies.add_borrow(x, a);
    let mut borrow_checker = BorrowChecker::new(&dependencies);
    borrow_checker.exec(&BorrowInstruction::push(x));
    borrow_checker.exec(&BorrowInstruction::push(a));
    assert_eq!(borrow_checker.variable_state(x), &VariableState::Intact);
    assert_eq!(
        borrow_checker.lifetime_state(a),
        LifetimeState::Uninitialized
    );
    assert_eq!(borrow_checker.sim_borrow(x, a), Ok(()));
    assert_eq!(
        borrow_checker.sim_borrow_mut(x, a),
        Err(BorrowError::InvalidLifetime)
    );
}
