#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate proc_static_assertions;

pub fn allowed_function() {}
pub fn disallowed_function() {}
pub fn allowed_function_not_checked() {}

pub struct MyStruct;

impl MyStruct {
    pub fn target_function(&self) {}
    pub fn target_function2(&self) {}

    #[calls("target_function")]
    pub fn allowed_caller(&self) {
        self.target_function();
    }

    #[calls("allowed_function", "target_function2")]
    pub fn allowed_caller_multiple(&self) {
        allowed_function();
        self.target_function2();
    }

    // ``` fails
    // #[calls("allowed_function", "target_function2")]
    // pub fn unauthorized_caller(&self) {
    //     self.target_function();
    // }
}

#[cfg(test)]
mod simple_tests {
    use super::*;

    #[test]
    fn test_allowed_function_calls() {
        #[allow(dead_code)]
        #[calls("allowed_function")]
        pub fn my_function() {
            allowed_function();
            // although not listed, error is not raised
            allowed_function_not_checked();
        }
    }

    // ``` fails
    // fn test_disallowed_function_calls() {
    //     #[allow(dead_code)]
    //     #[calls("allowed_function")]
    //     pub fn my_function() {
    //         disallowed_function();
    //     }
    // }
}

mod nested_tests {
    use super::*;
    
    #[test]
    fn test_nested_calls() {
        // A calls macro would require whitelisting all the function called in the function.
        // Therefore I have also implemented #[nocalls("name")] that swaps mode and allows
        // restrict the usage to certain functions.
        
        #[allow(dead_code)]
        #[calls("allowed_function", "name")]
        pub fn my_function() {
            let name = || {
                while false {
                    for _ in 0..5 {
                        // ``` fails
                        // disallowed_function();
                        allowed_function();
                    }
                }

                // ``` fails
                // let _i = { disallowed_function(); };
            };

            name();
            allowed_function();
        }
    }
}
