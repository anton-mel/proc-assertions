#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate proc_static_assertions;

mod simple_tests {
    #[allow(dead_code)]
    pub struct ConsumedStruct;

    #[test]
    fn test_consumes_list() {
        #[allow(dead_code)]
        #[consumes("u8", "ConsumedStruct")]
        fn test_function(_arg1: i32, _arg2: u8, _arg3: ConsumedStruct) {}
        // The only type of argument that proc-macros can accept are &str
    }

    // Should generate a compile time error
    // #[test]
    // fn test_consumes_list_fails() {
    //     #[allow(dead_code)]
    //     #[consumes("u8", "ConsumedStruct2")]
    //     fn test_function(_arg1: i32, _arg2: u8, _arg3: ConsumedStruct) {}
    // }

    // This should generate a compile time error because the main purpose of this macro is to check
    // that a type is "consumed", meaning ownership is passed to the called function, not just that it's
    // in the argument list
    #[test]
    fn test_consumes_list_should_fail() {
        #[allow(dead_code)]
        #[consumes("u8", "ConsumedStruct")]
        fn test_function(_arg1: i32, _arg2: u8, _arg3: &mut ConsumedStruct) {}
    }
}
