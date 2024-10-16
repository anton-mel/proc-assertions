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
    }

    #[test]
    fn test_consumes_list_reference() {
        #[allow(dead_code)]
        #[consumes("u8", "& ConsumedStruct")]
        fn test_function(_arg1: i32, _arg2: u8, _arg3: &ConsumedStruct) -> i32 { 5 }

        let a = ConsumedStruct;
        assert_eq!(test_function(0, 0, &a), 5);
    }

    #[test]
    fn test_consumes_list_mutable() {
        #[allow(dead_code)]
        #[consumes("u8", "& mut ConsumedStruct")]
        fn test_function(_arg1: i32, _arg2: u8, _arg3: &mut ConsumedStruct) {}
    }
}

// Define a separate struct for testing self-consuming methods
pub struct StructWithSelf;

impl StructWithSelf {
    #[allow(dead_code)]
    #[consumes("self")]
    pub fn into_allocated_frames(self) -> i32 {
        10
    }
}

#[cfg(test)]
mod self_tests {
    use super::StructWithSelf;

    #[test]
    fn test_self_consumption() {
        let s = StructWithSelf;
        assert_eq!(s.into_allocated_frames(), 10);
    }
}
