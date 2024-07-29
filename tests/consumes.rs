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
        #[assert_function_consumes("u8", "ConsumedStruct")]
        fn test_function(_arg1: i32, _arg2: u8, _arg3: ConsumedStruct) {}
        // The only type of argument that proc-macros can accept are &str
    }

    // Should generate a compile time error
    // #[test]
    // fn test_consumes_list() {
    //     #[allow(dead_code)]
    //     #[assert_function_consumes("u8", "ConsumedStruct2")]
    //     fn test_function(_arg1: i32, _arg2: u8, _arg3: ConsumedStruct) {}
    // }
}
