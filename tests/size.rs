#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate proc_static_assertions;

// See more abiout how rust compiler allignes memory:
// https://doc.rust-lang.org/reference/type-layout.html.
mod simple_tests {
    #[test]
    fn test_assert_align_size() {
        // Basic struct with a single i32 field
        #[allow(dead_code)]
        #[assert_align_size(size: 4, align: 4)]
        struct Bar {
            value: i32,
        }

        // Struct with larger size and alignment
        #[allow(dead_code)]
        #[assert_align_size(size: 16, align: 8)]
        struct LargeStruct {
            a: i64,
            b: i32,
            c: u16,
        }

        // Struct with packed fields (alignment 1)
        #[allow(dead_code)]
        #[assert_align_size(size: 8, align: 4)]
        struct PackedStruct {
            a: u8,
            b: u16,
            c: u32,
        }

        // Struct with minimum alignment (default alignment)
        #[allow(dead_code)]
        #[assert_align_size(size: 2, align: 1)]
        struct DefaultAlign {
            a: u8,
            b: u8,
        }

        // Struct with custom alignment attribute
        #[allow(dead_code)]
        #[repr(align(16))]
        #[assert_align_size(size: 16, align: 16)]
        struct CustomAlignStruct {
            a: u64,
            b: u64,
        }

        // Empty struct with default alignment
        #[allow(dead_code)]
        #[assert_align_size(size: 0, align: 1)]
        struct EmptyStruct;

        // Struct with no fields (size 0) and default alignment
        #[allow(dead_code)]
        #[assert_align_size(size: 0, align: 1)]
        struct NoFieldsStruct {}
    }
}
