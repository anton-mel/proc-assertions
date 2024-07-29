#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate proc_static_assertions;

#[derive(Default)]
pub struct MyStruct {
    field: i32,
}

// Unfortunately, proc-macros cannot be applied to the Struct fields, so we
// work around this by explicitly specifying the data to be checked.
#[mutatedby("allowed_mutate", "allowed_mutate_multiple")]
impl MyStruct {
    #[assert_mutates]
    pub fn allowed_mutate(&mut self) {
        self.field += 1;
    }
    
    #[assert_mutates]
    pub fn allowed_mutate_multiple(&mut self) {
        self.field -= 1;
    }

    #[assert_mutates]
    pub fn unauthorized_mutate(&mut self) {
        self.field = 0;
    }
}

pub fn outside_caller(instance: &mut MyStruct) {
    instance.field += 2;
}

#[cfg(test)]
mod simple_tests {
    use super::*;

    #[test]
    fn test_allowed_mutate() {
        let mut instance = MyStruct::default();
        instance.allowed_mutate();
        assert_eq!(instance.field, 1);
    }

    #[test]
    fn test_allowed_mutate_multiple() {
        let mut instance = MyStruct::default();
        instance.allowed_mutate_multiple();
        assert_eq!(instance.field, -1);
    }

    #[test]
    fn test_outside_caller() {
        let mut instance = MyStruct::default();
        outside_caller(&mut instance);
        assert_eq!(instance.field, 2);
    }

    #[test]
    #[should_panic(expected = "Unauthorized function trying to mutate fields in MyStruct: unauthorized_mutate")]
    fn test_unauthorized_mutate() {
        let mut instance = MyStruct::default();
        instance.unauthorized_mutate();
    }
}
