#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {
    #[ink(storage)]
    pub struct Incrementer {
        // Storage Declaration
        value: i32,
        my_value: ink_storage::collections::HashMap<AccountId, i32>
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            // Contract Constructor
            Self{
                value: init_value,
                my_value: ink_storage::collections::HashMap::new()
            }
        }

        pub fn default() -> Self {
            Self {
                value: 0,
                my_value: Default::default()
            }
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            // Contract Message
            self.value
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value += by;
        }

        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            let caller = self.env().caller();
            self.my_value_or_zero(&caller)
        }

        #[ink(message)]
        pub fn inc_mine(&mut self, by: i32) {
            let caller = self.env().caller();
            self.my_value
                .entry(caller)
                .and_modify(|old_value| *old_value += by)
                .or_insert(by);
        }

        fn my_value_or_zero(&self, of: &AccountId) -> i32 {
            let value = self.my_value.get(of).unwrap_or(&0);
            *value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            // Test Your Contract
            let contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
        }        
        
        #[ink::test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
            contract.inc(-50);
            assert_eq!(contract.get(), -3);
        }

        #[ink::test]
        fn my_value_works() {
            let mut contract = Incrementer::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.inc_mine(10);
            assert_eq!(contract.get_mine(), 15);
        }
    }
}
