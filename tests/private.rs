#![no_std]
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

        // ISSUE! After the proc macro application, the struct cannot be found
        // let a = TestStruct{ field1: 0, field2: 0};
    }
}

// Uncommenting the following code should 
// trigger a compile-time error
// #[private_fields("field1")]
// struct InvalidStruct {
//     pub field1: i32,
//     field2: String,
// }
