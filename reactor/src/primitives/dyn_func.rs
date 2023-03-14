use std::{any::Any, cell::RefCell};

use super::AnyData;

pub struct DynFunc {
    pub(crate) func: Box<dyn Fn(&Box<RefCell<dyn Any>>) -> bool>,
    pub(crate) value: AnyData,
}

#[cfg(feature = "extra-traits")]
impl std::fmt::Debug for DynFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RefFunc")
    }
}

impl DynFunc {
    pub fn new<F, T>(func: F) -> Self
    where
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let val = AnyData::new(func());
        let func = Box::new(move |val: &Box<RefCell<dyn Any>>| {
            let new = func();
            let mut old_any = val.borrow_mut();
            let old: &mut T = old_any.downcast_mut::<T>().unwrap();
            *old = new;
            true
        });
        Self { func, value: val }
    }

    #[allow(dead_code)]
    pub fn new_eq<F, T>(func: F) -> Self
    where
        F: Fn() -> T + 'static,
        T: PartialEq + 'static,
    {
        let val = AnyData::new(func());
        let func = Box::new(move |val: &Box<RefCell<dyn Any>>| {
            let new = func();
            let mut old_any = val.borrow_mut();
            let old: &mut T = old_any.downcast_mut::<T>().unwrap();
            let changed = new != *old;
            *old = new;
            changed
        });
        Self { func, value: val }
    }
    pub fn run(&self) -> bool {
        (self.func)(&self.value.0)
    }
}

#[test]
fn test_usize() {
    let num_fn = DynFunc::new_eq(|| 42usize);
    assert_eq!(num_fn.run(), false);
    assert_eq!(num_fn.run(), false);
    assert_eq!(num_fn.value.get::<usize>(), 42);
}

#[test]
fn test_string() {
    let string_fn = DynFunc::new_eq(|| "hello".to_string());
    assert_eq!(string_fn.value.cloned::<String>(), "hello".to_string());

    use std::cell::RefCell;
    use std::rc::Rc;

    let input = Rc::new(RefCell::new(1));

    let input_cp = input.clone();
    let dyn_fn = DynFunc::new_eq(move || {
        let val = input_cp.borrow();
        format!("Val: {}", val)
    });

    assert_eq!(dyn_fn.run(), false);

    assert_eq!(dyn_fn.value.cloned::<String>(), "Val: 1".to_string());
    {
        let mut val = input.borrow_mut();
        *val = 2;
    }
    assert_eq!(dyn_fn.run(), true);
    assert_eq!(dyn_fn.run(), false);

    assert_eq!(dyn_fn.value.cloned::<String>(), "Val: 2".to_string());
}
