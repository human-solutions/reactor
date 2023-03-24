//! This is used in benches and in flamegraph examples
//!

use crate::{
    primitives::{Data, Func},
    runtimes::ClientRuntime,
    Scope, Signal,
};

pub fn create_1000_nested_scopes_each_with_a_signal() -> (
    Scope<ClientRuntime>,
    Signal<Data<usize>, ClientRuntime>,
    Signal<Func<usize>, ClientRuntime>,
) {
    let mut scope = ClientRuntime::bench_root_scope();

    // don't use the signal! macro, because we want to force the signals to
    // be non equals. Otherwise a propagation wouldn't happen
    let start_sig = Signal::new_data(scope, 0usize);
    let mut next_sig = Signal::new_func(scope, move || start_sig.get() + 1);

    (0..1000).for_each(|_| {
        scope = scope.new_child();
        let sig = Signal::new_func(scope, move || next_sig.get() + 1);
        next_sig = sig;
    });

    let end_sig = next_sig;
    (scope, start_sig, end_sig)
}

pub fn create_1000_nested_signals_in_a_scope() -> (
    Scope<ClientRuntime>,
    Signal<Data<usize>, ClientRuntime>,
    Signal<Func<usize>, ClientRuntime>,
) {
    let mut scope = ClientRuntime::bench_root_scope();

    // don't use the signal! macro, because we want to force the signals to
    // be non equals. Otherwise a propagation wouldn't happen
    let start_sig = Signal::new_data(scope, 0usize);
    let mut next_sig = Signal::new_func(scope, move || start_sig.get() + 1);

    (0..1000).for_each(|_| {
        scope = scope.new_child();
        let sig = Signal::new_func(scope, move || next_sig.get() + 1);
        next_sig = sig;
    });

    let end_sig = next_sig;
    (scope, start_sig, end_sig)
}

pub fn create_1000_nested_scopes() -> () {
    let mut scope = ClientRuntime::bench_root_scope();

    (0..1000).for_each(|_| {
        scope = scope.new_child();
    });
}

pub fn create_1000_data_signals() -> () {
    let scope = ClientRuntime::bench_root_scope();

    (0..1000).for_each(|_| {
        Signal::new_data(scope, 0usize);
    });
}

pub fn create_1000_func_signals() -> () {
    let scope = ClientRuntime::bench_root_scope();

    (0..1000).for_each(|_| {
        Signal::new_func(scope, || 1);
    });
}

pub fn create_1000_func_signals_with_one_subscription() -> () {
    let scope = ClientRuntime::bench_root_scope();
    let sig = Signal::new_data(scope, 0usize);
    (0..1000).for_each(|_| {
        Signal::new_func(scope, move || sig.get());
    });
}

pub fn create_1000_siblings() -> (
    Scope<ClientRuntime>,
    Signal<Data<usize>, ClientRuntime>,
    Signal<Func<usize>, ClientRuntime>,
) {
    let scope = ClientRuntime::bench_root_scope();
    // don't use the signal! macro, because we want to force the signals to
    // be non equals. Otherwise a propagation wouldn't happen
    let start_sig = Signal::new_data(scope, 0usize);
    let mut next_sig = Signal::new_func(scope, move || start_sig.get() + 1);

    (0..1000).for_each(|_| {
        let sx = scope.new_child();
        let sig = Signal::new_func(sx, move || next_sig.get() + 1);
        next_sig = sig;
    });

    let end_sig = next_sig;
    (scope, start_sig, end_sig)
}

#[test]
fn test_1000_nested() {
    let (_sx, start, end) = create_1000_nested_scopes_each_with_a_signal();

    start.set(1);
    assert_eq!(end.get(), 1002);
}

#[test]
fn test_1000_siblings() {
    let (_sx, start, end) = create_1000_siblings();

    start.set(1);
    assert_eq!(end.get(), 1002);
}
