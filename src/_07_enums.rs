// ============================================================================
// 07_enums.rs - Rust 枚举和模式匹配详解
// ============================================================================

#![allow(dead_code, unused_variables)]

pub fn run() {
    println!("=== Rust 枚举和模式匹配学习 ===\n");

    // -------------------------------------------------------------------------
    // 1. 枚举定义
    // -------------------------------------------------------------------------
    println!("【1. 枚举定义】");
    
    // 简单枚举
    #[derive(Debug)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    
    let up = Direction::Up;
    println!("方向: {:?}", up);

    // -------------------------------------------------------------------------
    // 2. 枚举值关联数据
    // -------------------------------------------------------------------------
    println!("\n【2. 枚举值关联数据】");
    
    #[derive(Debug)]
    enum Message {
        Quit,                       // 没有关联数据
        Move { x: i32, y: i32 },    // 关联匿名结构体
        Write(String),              // 关联 String
        ChangeColor(i32, i32, i32), // 关联元组
    }
    
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 0, 0);
    
    println!("消息: {:?}, {:?}, {:?}, {:?}", m1, m2, m3, m4);

    // -------------------------------------------------------------------------
    // 3. Option 枚举
    // -------------------------------------------------------------------------
    println!("\n【3. Option 枚举】");
    
    // Option 是标准库定义的枚举，用于表示可能存在的值
    // pub enum Option<T> {
    //     Some(T),
    //     None,
    // }
    
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("hello");
    let absent_number: Option<i32> = None;
    
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);
    
    // Option vs null
    // Option 强制处理空值情况，避免空指针异常

    // -------------------------------------------------------------------------
    // 4. Result 枚举
    // -------------------------------------------------------------------------
    println!("\n【4. Result 枚举】");
    
    // Result 用于错误处理
    // pub enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("除数不能为零"))
        } else {
            Ok(a / b)
        }
    }
    
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("错误: {}", e),
    }
    
    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("错误: {}", e),
    }

    // -------------------------------------------------------------------------
    // 5. match 表达式
    // -------------------------------------------------------------------------
    println!("\n【5. match 表达式】");
    
    let number = 7;
    
    match number {
        1 => println!("一"),
        2 | 3 | 5 | 7 => println!("一位数质数"),
        4 | 6 | 8 | 9 => println!("一位数合数"),
        _ => println!("其他"),
    }
    
    // match 必须穷尽所有可能
    // _ 是通配符，匹配任何值

    // -------------------------------------------------------------------------
    // 6. 匹配 Option<T>
    // -------------------------------------------------------------------------
    println!("\n【6. 匹配 Option<T>】");
    
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("five + 1 = {:?}", six);
    println!("None + 1 = {:?}", none);

    // -------------------------------------------------------------------------
    // 7. if let 简化匹配
    // -------------------------------------------------------------------------
    println!("\n【7. if let 简化匹配】");
    
    let some_value = Some(7);
    
    // 使用 match
    match some_value {
        Some(7) => println!("match: 匹配到 7"),
        _ => (),
    }
    
    // 使用 if let 更简洁
    if let Some(7) = some_value {
        println!("if let: 匹配到 7");
    }
    
    // if let + else
    if let Some(n) = some_value {
        println!("if let: 值是 {}", n);
    } else {
        println!("if let: 没有值");
    }

    // -------------------------------------------------------------------------
    // 8. while let 循环
    // -------------------------------------------------------------------------
    println!("\n【8. while let 循环】");
    
    let mut optional = Some(0);
    
    while let Some(i) = optional {
        if i > 5 {
            println!("Done!");
            optional = None;
        } else {
            println!("i = {}", i);
            optional = Some(i + 1);
        }
    }

    // -------------------------------------------------------------------------
    // 9. 匹配绑定
    // -------------------------------------------------------------------------
    println!("\n【9. 匹配绑定】");
    
    let x = Some(5);
    
    match x {
        Some(n) if n < 5 => println!("小于 5: {}", n),  // 守卫
        Some(n) => println!("大于等于 5: {}", n),
        None => println!("没有值"),
    }

    // -------------------------------------------------------------------------
    // 10. 枚举方法
    // -------------------------------------------------------------------------
    println!("\n【10. 枚举方法】");
    
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // ... 其他州
    }
    
    impl Coin {
        fn value(&self) -> u8 {
            match self {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("州 25 分硬币: {:?}", state);
                    25
                }
            }
        }
    }
    
    let coin = Coin::Quarter(UsState::Alaska);
    println!("硬币价值: {} 分", coin.value());

    // -------------------------------------------------------------------------
    // 11. 匹配解构
    // -------------------------------------------------------------------------
    println!("\n【11. 匹配解构】");
    
    // 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }
    
    let point = Point { x: 10, y: 20 };
    
    match point {
        Point { x: 0, y } => println!("在 y 轴上，y = {}", y),
        Point { x, y: 0 } => println!("在 x 轴上，x = {}", x),
        Point { x, y } => println!("坐标: ({}, {})", x, y),
    }
    
    // 解构枚举
    match m2 {
        Message::Quit => println!("退出"),
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) => println!("写入: {}", text),
        Message::ChangeColor(r, g, b) => println!("颜色: ({}, {}, {})", r, g, b),
    }

    // -------------------------------------------------------------------------
    // 12. 匹配守卫
    // -------------------------------------------------------------------------
    println!("\n【12. 匹配守卫】");
    
    let number = 4;
    
    match number {
        n if n % 2 == 0 => println!("偶数: {}", n),
        n => println!("奇数: {}", n),
    }
    
    // 多条件守卫
    let x = 10;
    match x {
        n if n > 0 && n < 10 => println!("个位数正整数"),
        n if n >= 10 && n < 100 => println!("两位数正整数"),
        _ => println!("其他"),
    }

    // -------------------------------------------------------------------------
    // 13. @ 绑定
    // -------------------------------------------------------------------------
    println!("\n【13. @ 绑定】");
    
    // @ 允许在测试值的同时创建变量
    
    match 5 {
        n @ 1..=5 => println!("1 到 5 之间: {}", n),
        n @ 6..=10 => println!("6 到 10 之间: {}", n),
        n => println!("其他: {}", n),
    }
    
    // 解构时使用 @
    struct Person {
        age: u32,
        name: String,
    }
    
    let person = Person {
        age: 25,
        name: String::from("张三"),
    };
    
    match person {
        Person { age: a @ 0..=18, name } => {
            println!("未成年人: {}, 年龄: {}", name, a);
        }
        Person { age: a @ 19..=60, name } => {
            println!("成年人: {}, 年龄: {}", name, a);
        }
        Person { age, name } => {
            println!("老年人: {}, 年龄: {}", name, age);
        }
    }

    // -------------------------------------------------------------------------
    // 14. matches! 宏
    // -------------------------------------------------------------------------
    println!("\n【14. matches! 宏】");
    
    let x = Some(5);
    
    // matches! 返回布尔值
    println!("是否为 Some: {}", matches!(x, Some(_)));
    println!("是否为 Some(5): {}", matches!(x, Some(5)));
    println!("是否为偶数: {}", matches!(x, Some(n) if n % 2 == 0));

    // -------------------------------------------------------------------------
    // 15. 枚举与泛型
    // -------------------------------------------------------------------------
    println!("\n【15. 枚举与泛型】");
    
    // 标准库中的 Option 和 Result 都是泛型枚举
    // 可以定义自己的泛型枚举
    
    #[derive(Debug)]
    enum MyOption<T> {
        Some(T),
        None,
    }
    
    let int_option = MyOption::Some(42);
    let str_option = MyOption::Some("hello");
    let none_option: MyOption<i32> = MyOption::None;
    
    println!("int_option: {:?}", int_option);
    println!("str_option: {:?}", str_option);
    println!("none_option: {:?}", none_option);
}
