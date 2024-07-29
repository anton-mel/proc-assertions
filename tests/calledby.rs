#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate proc_static_assertions;

pub struct MyStruct;

impl MyStruct {
    /// Try removing one of the function here to see
    #[calledby("allowed_caller", "allowed_caller_multiple", "outside_caller")]
    pub fn target_function(&self) -> &str {
        return "Hello World";
    }

    #[assert_callsite]
    pub fn allowed_caller(&self) {
        self.target_function();
    }

    #[assert_callsite]
    pub fn allowed_caller_multiple(&self) {
        self.target_function();
    }

    #[assert_callsite]
    pub fn unauthorized_caller(&self) {
        self.target_function();
    }
}

pub fn outside_caller(my_struct: &MyStruct) {
    MyStruct::__callsite("outside_caller");
    my_struct.target_function();
}

#[cfg(test)]
mod simple_tests {
    use super::*;

    #[test] 
    fn test_allowed_caller() {
        let my_struct = MyStruct;
        my_struct.allowed_caller();
    }

    #[test]
    fn test_allowed_caller_multiple() {
        let my_struct = MyStruct;
        my_struct.allowed_caller_multiple();
    }

    #[test]
    fn test_outside_caller() {
        let my_struct = MyStruct;
        outside_caller(&my_struct);
    }

    #[test]
    #[should_panic(expected = "Unauthorized function trying to call target_function: unauthorized_caller")]
    fn test_unauthorized_caller() {
        let my_struct = MyStruct;
        my_struct.unauthorized_caller();
    }
}
