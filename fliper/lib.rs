#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod fliper {
    /// 定义智能合约的存储类型结构体
    #[ink(storage)]
    pub struct Fliper {
        /// 这里定义了简单的布尔类型
        value: bool,
    }
        /// 创建智能合约实例化方法的关联函数
    impl Fliper {
        /// 创建将布尔型转换成init型的实例化方法
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// 创建默认值的实例化方法
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// 创建用户可调用的公共方法
        #[ink(message)]
        /// flip方法用于布尔类型值true和false的相互转换
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// 简单地返回布尔类型值
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// 创建智能合约的测试方法
    #[cfg(test)]
    mod tests {
        /// 引入所有的外部定义
        use super::*;

        /// 引入`ink_lang`定义，以使用ink测试的属性宏`#[ink::test]`.
        use ink_lang as ink;

        /// ink的测试用例是通过测试合约的实例化方法进行的
        #[ink::test]
        /// 关于默认值的测试用例
        fn default_works() {
            let fliper = Fliper::default();
            assert_eq!(fliper.get(), false);
        }

        
        #[ink::test]
        /// 关于fliper方法的测试用例
        fn it_works() {
            let mut fliper = Fliper::new(false);
            assert_eq!(fliper.get(), false);
            fliper.flip();
            assert_eq!(fliper.get(), true);
        }
    }
}
