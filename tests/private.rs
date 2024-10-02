#![no_std]
#![allow(dead_code)]
#![deny(unsafe_code)]

#[macro_use]
extern crate proc_static_assertions;

mod simple_tests {
    #[test]
    fn test_assert_private_fields() {
        #[private_fields("field1")]
        struct TestStruct {
            field1: i32,
            pub field2: u32,
        }

        let _a = TestStruct{ field1: 0, field2: 0};
    }
}

// Uncommenting the following code should 
// trigger a compile-time error
// #[private_fields("field1")]
// struct InvalidStruct {
//     pub field1: i32,
//     field2: String,
// }
