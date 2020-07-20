#![no_std]
#![allow(non_snake_case)]
#![feature(proc_macro_hygiene)]
#![feature(alloc)]

extern crate pwasm_std;
extern crate pwasm_ethereum;
extern crate pwasm_abi;
extern crate pwasm_abi_derive;
extern crate serde;
extern crate serde_json;
extern crate alloc;

pub mod token {
    use pwasm_ethereum;
    use pwasm_abi::types::*;

    // eth_abi is a procedural macros https://doc.rust-lang.org/book/first-edition/procedural-macros.html
    use pwasm_abi_derive::eth_abi;
    use serde::{Serialize, Deserialize};
    use serde_json::from_str;
    use alloc::string::String;

    #[eth_abi(TokenEndpoint, TokenClient)]
    pub trait TokenInterface {
        /// The constructor
        fn constructor(&mut self);
        /// Total amount of tokens
        #[constant]
        fn checkFieldIntegrity(&mut self, data: String) -> String;
    }

    #[derive(Serialize, Deserialize)]
    struct Person {
        name: String,
        age: u8,
        phones: Vec<String>,
        weight: Option<u8>,
    }
    pub struct TokenContract;

    impl TokenInterface for TokenContract {
        fn constructor(&mut self) {

        }

        fn checkFieldIntegrity(&mut self, data: String) -> String {
            let data = r#"
                {
                    "name": "John Doe",
                    "age": 43,
                    "phones": [
                        "+44 1234567",
                        "+44 2345678"
                    ]
                }"#;

            // 解析字符串到Person对象。
            let p: Person = serde_json::from_str(data).unwrap();
            p.name
        }

    }


}
// Declares the dispatch and dispatch_ctor methods
use pwasm_abi::eth::EndpointInterface;

#[no_mangle]
pub fn call() {
    let mut endpoint = token::TokenEndpoint::new(token::TokenContract{});
    // Read http://solidity.readthedocs.io/en/develop/abi-spec.html#formal-specification-of-the-encoding for details
    pwasm_ethereum::ret(&endpoint.dispatch(&pwasm_ethereum::input()));
}

#[no_mangle]
pub fn deploy() {
    let mut endpoint = token::TokenEndpoint::new(token::TokenContract{});
    endpoint.dispatch_ctor(&pwasm_ethereum::input());
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    extern crate pwasm_test;
    extern crate std;
    use super::*;
    use core::str::FromStr;
    use pwasm_abi::types::*;
    use self::pwasm_test::{ext_reset, ext_get};
    use token::TokenInterface;
    
    #[test]
    fn parse_person_name() {
        let mut contract = token::TokenContract{};
        contract.constructor();
        assert_eq!(contract.totalSupply(String::from("data")), String::from("John Doe"))
    }


}
