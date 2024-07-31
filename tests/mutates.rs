#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate proc_static_assertions;


#[derive(Default)]
pub struct MyStruct {
    pub field: i32,
}

impl MyStruct {
    #[mutates(MyStruct: ("field"))]
    pub fn allowed_mutate(&mut self) {
        self.field += 1;
    }
    
    // works even if used several times in a mod
    #[mutates(MyStruct: ("field"))]
    pub fn allowed_mutate_multiple(&mut self) {
        self.field -= 1;
    }

    pub fn unauthorized_mutate(&mut self) {
        self.field = 0;
    }
}

// Your test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[mutates(MyStruct: ("field"))]
    fn test_allowed_mutate_inner() {
        let mut instance = MyStruct::default();
        instance.field = 1;
        assert_eq!(instance.field, 1);
    }

    #[test]
    fn test_allowed_mutate_args() {
        let mut instance = MyStruct::default();
        test_allowed_mutate_args_impl(&mut instance);
    }

    #[mutates(MyStruct: ("field"))]
    fn test_allowed_mutate_args_impl(instance: &mut MyStruct) {
        instance.field += 1;
        assert_eq!(instance.field, 1);
    }
}

mod nested_tests {
    use super::*;
    
    #[test]
    #[mutates(MyStruct: ("field"))]
    fn test_nested_mutate() {
        #[allow(unused_mut)]
        let mut instance = MyStruct::default();

        #[allow(unused_mut)]
        let mut name = || {
            while false {
                for _ in 0..5 {
                    // ```all with = sign fails
                    // instance.field = 1;
                    // instance.field /= 1;
                    // instance.field *= 1;
                    // instance.field += 1;
                    // instance.field -= 1;
                    instance.field; // ok
                }
            }

            // ```fails even if nested
            // let _i = { instance.field -= 1; };
            // while false { instance.field -= 1; };
            // if false { instance.field -= 1; }
        };

        // Call the inner function
        name();
    }
}
