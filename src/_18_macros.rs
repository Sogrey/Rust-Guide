#![allow(dead_code, unused_variables, unused_imports)]

// Rust 宏示例

pub fn run() {
    println!("=== 声明宏 ===");
    declarative_macros();
    
    println!("\n=== 内置宏 ===");
    builtin_macros();
}

// 声明宏示例
macro_rules! say_hello {
    () => {
        println!("你好！");
    };
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("函数 {:?} 被调用", stringify!($func_name));
        }
    };
}

macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn declarative_macros() {
    // 使用自定义宏
    say_hello!();
    
    create_function!(foo);
    create_function!(bar);
    foo();
    bar();
    
    // 向量创建宏
    let v = my_vec![1, 2, 3];
    println!("向量: {:?}", v);
}

fn builtin_macros() {
    // println! 和 format!
    let s = format!("值: {}", 42);
    println!("格式化字符串: {}", s);
    
    // vec!
    let v = vec![1, 2, 3, 4, 5];
    println!("向量: {:?}", v);
    
    // assert! 系列
    assert!(true);
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);
    
    // dbg!
    let x = 42;
    dbg!(x);
    
    // concat! 和 stringify!
    let s = concat!("Hello", " ", "World");
    println!("连接: {}", s);
    
    let s = stringify!(1 + 2);
    println!("字符串化: {}", s);
    
    // cfg!
    if cfg!(debug_assertions) {
        println!("调试模式");
    } else {
        println!("发布模式");
    }
}
