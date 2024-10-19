#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate proc_assertions;

#[derive(Default)]
pub struct MyStruct {
    pub field1: i32,
    pub field2: i32,
    pub field3: i32,
}

impl MyStruct {
    // Although passed as mutable reference,
    // function does not mutate the field.
    #[nomutates(MyStruct: ("field1"))]
    pub fn allowed_mutate(&mut self) {
        self.field1;
        // ``` should fail
        // self.field1 = 1;
        // self.field1 /= 1;
        // self.field1 *= 1;
        // self.field1 += 1;
        // self.field1 -= 1;
        // ```
    }
    
    // Hanldes multiple fields, if field is not listed
    // nomutates will not raise the compile error.
    #[nomutates(MyStruct: ("field1", "field2"))]
    pub fn unauthorized_mutate(&mut self) {
        self.field1;
        self.field2;
        self.field3 = 0; // not listed field
        // should is not checked by nomutates.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allowed_mutate_inner() {
        let mut instance = MyStruct::default();
        instance.allowed_mutate();
    }

    #[test]
    fn test_allowed_mutate_args() {
        let mut instance = MyStruct::default();
        test_allowed_mutate_args_impl(&mut instance);
    }

    fn test_allowed_mutate_args_impl(instance: &mut MyStruct) {
        instance.allowed_mutate();
    }
}

// Test case in nested scenarios
mod nested_tests {
    use super::*;

    #[test]
    #[nomutates(MyStruct: ("field1"))]
    fn test_nested_mutate() {
        #[allow(unused_mut)]
        let mut instance = MyStruct::default();

        // Check for nested macros
        #[allow(unused_mut)]
        let mut name = || {
            while false {
                for _ in 0..5 {
                    // Mutation here should be detected if it violates the rules
                    // Should produce a compile-time error if uncommented
                    
                    instance.field1; // OK
                    // instance.field1 /= 1; // fails
                }
            }

            // Any mutation here should also produce a compile-time error if uncommented
            let _i = { instance.field2 -= 1; }; // OK, not whitelisted
            while false { instance.field2 -= 1; };  // OK, not whitelisted
            if false { instance.field2 -= 1; }      // OK, not whitelisted
        };

        name();
    }
}
