// ============================================================================
// 09_modules.rs - Rust 包和模块详解
// ============================================================================

pub fn run() {
    println!("=== Rust 包和模块学习 ===\n");

    // -------------------------------------------------------------------------
    // 1. 模块系统概述
    // -------------------------------------------------------------------------
    println!("【1. 模块系统概述】\n");
    
    // Rust 的模块系统包括：
    // - Package（包）：一个 Cargo 项目，包含 Cargo.toml
    // - Crate（单元包）：一个模块树，产生库或可执行文件
    // - Module（模块）：用 mod 组织代码和路径
    // - Path（路径）：用于命名项，如 struct、function
    
    // -------------------------------------------------------------------------
    // 2. 模块定义
    // -------------------------------------------------------------------------
    println!("【2. 模块定义】\n");
    
    // 模块使用 mod 关键字定义
    mod front_of_house {
        // 模块可以嵌套
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("添加到等待列表");
            }
            
            fn seat_at_table() {
                println!("安排入座");
            }
        }
        
        pub mod serving {
            pub fn take_order() {
                println!("点餐");
            }
            
            pub fn serve_order() {
                println!("上菜");
            }
            
            fn take_payment() {
                println!("结账");
            }
        }
    }
    
    // 调用模块中的函数
    front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::take_order();

    // -------------------------------------------------------------------------
    // 3. 路径
    // -------------------------------------------------------------------------
    println!("\n【3. 路径】\n");
    
    // 使用 use 关键字引入路径
    use front_of_house::hosting;
    hosting::add_to_waitlist();
    
    // 使用 as 关键字起别名
    use front_of_house::serving::serve_order as serve;
    serve();
    
    // 使用 use 引入多个项
    use front_of_house::serving::{take_order, serve_order};
    take_order();
    serve_order();
    
    // 使用 * 引入所有公共项（慎用）
    // use front_of_house::hosting::*;

    // -------------------------------------------------------------------------
    // 4. pub 关键字
    // -------------------------------------------------------------------------
    println!("\n【4. pub 关键字 - 可见性】\n");
    
    // pub 使项变为公共的
    // 默认情况下，所有项都是私有的
    
    mod my_module {
        // 私有函数（默认）
        fn private_function() {
            println!("私有函数");
        }
        
        // 公共函数
        pub fn public_function() {
            println!("公共函数");
            private_function();  // 同模块可访问私有项
        }
        
        // 公共结构体
        pub struct PublicStruct {
            pub public_field: i32,      // 公共字段
            private_field: i32,         // 私有字段
        }
        
        impl PublicStruct {
            pub fn new(value: i32) -> PublicStruct {
                PublicStruct {
                    public_field: value,
                    private_field: value * 2,
                }
            }
            
            pub fn get_private(&self) -> i32 {
                self.private_field
            }
        }
        
        // 公共枚举（枚举的所有变体自动成为公共）
        pub enum PublicEnum {
            Variant1,
            Variant2(i32),
        }
    }
    
    my_module::public_function();
    
    let s = my_module::PublicStruct::new(10);
    println!("公共字段: {}", s.public_field);
    println!("私有字段通过方法访问: {}", s.get_private());
    // s.private_field;  // 错误！私有字段不可访问

    // -------------------------------------------------------------------------
    // 5. super 和 self 关键字
    // -------------------------------------------------------------------------
    println!("\n【5. super 和 self 关键字】\n");
    
    mod parent {
        pub fn parent_function() {
            println!("父模块函数");
        }
        
        pub mod child {
            pub fn child_function() {
                println!("子模块函数");
            }
            
            pub fn call_parent() {
                // 使用 super 访问父模块
                super::parent_function();
                // 等同于
                // crate::parent::parent_function();
            }
            
            pub fn call_self() {
                // 使用 self 访问当前模块
                self::child_function();
            }
        }
    }
    
    parent::child::call_parent();
    parent::child::call_self();

    // -------------------------------------------------------------------------
    // 6. 结构体可见性
    // -------------------------------------------------------------------------
    println!("\n【6. 结构体和字段可见性】\n");
    
    mod shapes {
        pub struct Rectangle {
            pub width: u32,     // 公共字段
            pub height: u32,    // 公共字段
        }
        
        impl Rectangle {
            pub fn new(width: u32, height: u32) -> Rectangle {
                Rectangle { width, height }
            }
            
            pub fn area(&self) -> u32 {
                self.width * self.height
            }
        }
        
        // 私有结构体
        struct InternalShape {
            points: Vec<(i32, i32)>,
        }
        
        pub fn create_internal() -> InternalShape {
            InternalShape {
                points: vec![(0, 0), (1, 1), (2, 2)],
            }
        }
    }
    
    let rect = shapes::Rectangle::new(10, 20);
    println!("矩形面积: {}", rect.area());
    // let internal = shapes::InternalShape { ... };  // 错误！私有结构体

    // -------------------------------------------------------------------------
    // 7. use 使用最佳实践
    // -------------------------------------------------------------------------
    println!("\n【7. use 使用最佳实践】\n");
    
    // 函数：引入父模块而非函数本身
    // use std::collections::HashMap;  // 好
    // use std::collections::hash_map::insert;  // 不好
    
    // 结构体和枚举：引入完整路径
    // use std::collections::HashMap;
    // use std::collections::HashSet;
    
    // 同名项使用 as
    use std::io::Result as IoResult;
    let _result: IoResult<()> = Ok(());

    // -------------------------------------------------------------------------
    // 8. 模块文件分离
    // -------------------------------------------------------------------------
    println!("\n【8. 模块文件分离】\n");
    
    // 模块可以放在单独的文件中：
    // - src/lib.rs 作为库的根
    // - src/main.rs 作为二进制程序的根
    // 
    // 模块声明方式：
    // - mod garden;  // 告诉 Rust 查找 garden.rs 或 garden/mod.rs
    // - mod garden {...}  // 内联模块
    
    // 示例目录结构：
    // backyard/
    // ├── Cargo.lock
    // ├── Cargo.toml
    // └── src
    //     ├── garden
    //     │   ├── vegetables.rs
    //     │   └── mod.rs
    //     ├── garden.rs
    //     └── main.rs

    // -------------------------------------------------------------------------
    // 9. crate 根
    // -------------------------------------------------------------------------
    println!("\n【9. crate 根】\n");
    
    // crate:: 前缀表示从 crate 根开始的绝对路径
    // 例如：crate::front_of_house::hosting::add_to_waitlist();

    // -------------------------------------------------------------------------
    // 10. pub use 重导出
    // -------------------------------------------------------------------------
    println!("\n【10. pub use 重导出】\n");
    
    // pub use 可以重导出项，改变调用路径
    mod outer {
        pub mod inner {
            pub fn deep_function() {
                println!("深层函数");
            }
        }
        
        // 重导出，使调用更简单
        pub use inner::deep_function;
    }
    
    // 两种调用方式都可以
    outer::inner::deep_function();
    outer::deep_function();  // 更简洁

    // -------------------------------------------------------------------------
    // 11. 外部包
    // -------------------------------------------------------------------------
    println!("\n【11. 外部包】\n");
    
    // 在 Cargo.toml 中添加依赖：
    // [dependencies]
    // rand = "0.8"
    
    // 使用外部包
    // use rand::Rng;
    // let n = rand::thread_rng().gen_range(1..=100);
    
    // 嵌套路径引入
    // use std::io::{self, Read, Write};
    // 等同于：
    // use std::io;
    // use std::io::Read;
    // use std::io::Write;

    // -------------------------------------------------------------------------
    // 12. 模块示例 - 工具库
    // -------------------------------------------------------------------------
    println!("\n【12. 模块示例 - 工具库】\n");
    
    mod utils {
        pub mod math {
            pub fn add(a: i32, b: i32) -> i32 {
                a + b
            }
            
            pub fn multiply(a: i32, b: i32) -> i32 {
                a * b
            }
            
            pub fn factorial(n: u64) -> u64 {
                if n <= 1 { 1 } else { n * factorial(n - 1) }
            }
        }
        
        pub mod string_utils {
            pub fn reverse(s: &str) -> String {
                s.chars().rev().collect()
            }
            
            pub fn is_palindrome(s: &str) -> bool {
                let reversed: String = s.chars().rev().collect();
                s.eq_ignore_ascii_case(&reversed)
            }
        }
    }
    
    use utils::math;
    use utils::string_utils;
    
    println!("5 + 3 = {}", math::add(5, 3));
    println!("5! = {}", math::factorial(5));
    println!("reverse: {}", string_utils::reverse("hello"));
    println!("'racecar' 是回文: {}", string_utils::is_palindrome("racecar"));

    // -------------------------------------------------------------------------
    // 13. 属性与模块
    // -------------------------------------------------------------------------
    println!("\n【13. 属性与模块】\n");
    
    // #[cfg(test)] - 测试模块
    // #[cfg(feature = "my_feature")] - 条件编译
    // #[doc = "..."] - 文档注释
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_add() {
            assert_eq!(math::add(2, 3), 5);
        }
    }
    
    println!("测试模块只在 cargo test 时编译");

    // -------------------------------------------------------------------------
    // 14. 模块私有性规则总结
    // -------------------------------------------------------------------------
    println!("\n【14. 模块私有性规则】\n");
    
    // 规则：
    // 1. 如果项是公共的，可以访问
    // 2. 如果项是私有的，只能在当前模块及其子模块中访问
    // 3. 结构体字段默认私有，需要 pub 才能外部访问
    // 4. 枚举变体继承枚举的可见性
    
    println!("模块系统帮助封装和组织代码，提高可维护性");
}
