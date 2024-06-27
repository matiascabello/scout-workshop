#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct MyStruct;

#[contractimpl]
impl MyStruct {
    pub fn split_profit(percentage: u64, total_profit: u64) -> u64 {
        (percentage / 100) * total_profit
    }
}

#[cfg(test)]
mod tests {
    use crate::MyStruct;

    #[test]
    fn split_profit_works() {
        let result = MyStruct::split_profit(33, 100);
        assert_eq!(result, 0);
    }
}
