// ============================================================================
// 04_control_flow.rs - Rust 流程控制详解
// ============================================================================

pub fn run() {
    println!("=== Rust 流程控制学习 ===\n");

    // -------------------------------------------------------------------------
    // 1. if 表达式
    // -------------------------------------------------------------------------
    println!("【1. if 表达式】");
    
    // if 是表达式，不是语句！
    let number = 7;
    
    if number < 5 {
        println!("条件为真");
    } else {
        println!("条件为假");
    }
    
    // if 可以作为表达式返回值
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("if 表达式结果: {}", result);
    
    // 注意：if 和 else 分支必须返回相同类型
    // let wrong = if condition { 5 } else { "six" };  // 错误！

    // -------------------------------------------------------------------------
    // 2. else if
    // -------------------------------------------------------------------------
    println!("\n【2. else if】");
    
    let number = 6;
    
    if number % 4 == 0 {
        println!("number 可以被 4 整除");
    } else if number % 3 == 0 {
        println!("number 可以被 3 整除");
    } else if number % 2 == 0 {
        println!("number 可以被 2 整除");
    } else {
        println!("number 不能被 4、3、2 整除");
    }

    // -------------------------------------------------------------------------
    // 3. 循环 - loop
    // -------------------------------------------------------------------------
    println!("\n【3. loop 循环】");
    
    // 无限循环
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;  // break 可以返回值
        }
    };
    
    println!("loop 循环结果: {}", result);
    
    // 循环标签 - 处理嵌套循环
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        
        loop {
            println!("  remaining = {}", remaining);
            if remaining == 9 {
                break;  // 跳出内层循环
            }
            if count == 2 {
                break 'counting_up;  // 跳出外层循环
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    println!("循环结束");

    // -------------------------------------------------------------------------
    // 4. 循环 - while
    // -------------------------------------------------------------------------
    println!("\n【4. while 循环】");
    
    let mut number = 3;
    
    while number != 0 {
        println!("倒计时: {}!", number);
        number -= 1;
    }
    println!("发射！");

    // -------------------------------------------------------------------------
    // 5. 循环 - for
    // -------------------------------------------------------------------------
    println!("\n【5. for 循环】");
    
    // 遍历数组
    let arr = [10, 20, 30, 40, 50];
    
    for element in arr {
        println!("数组元素: {}", element);
    }
    
    // 使用 Range
    for number in 1..=5 {
        println!("Range 1..=5: {}", number);
    }
    
    // 反向迭代
    for number in (1..4).rev() {
        println!("反向 Range: {}", number);
    }

    // -------------------------------------------------------------------------
    // 6. match 表达式
    // -------------------------------------------------------------------------
    println!("\n【6. match 表达式】");
    
    let number = 13;
    
    match number {
        1 => println!("一"),
        2 | 3 | 5 | 7 | 11 => println!("质数"),
        13..=19 => println!("青少年数字"),
        _ => println!("其他数字"),  // _ 是通配符，必须穷尽所有可能
    }
    
    // match 作为表达式
    let description = match number {
        1 => "一",
        2 | 3 | 5 | 7 | 11 => "质数",
        _ => "其他",
    };
    println!("数字 {} 是 {}", number, description);

    // -------------------------------------------------------------------------
    // 7. if let
    // -------------------------------------------------------------------------
    println!("\n【7. if let】");
    
    // 用于处理只关心一种模式的匹配
    let some_value = Some(7);
    
    // 使用 match 的方式
    match some_value {
        Some(7) => println!("匹配到 7"),
        _ => (),
    }
    
    // 使用 if let 更简洁
    if let Some(7) = some_value {
        println!("if let 匹配到 7");
    }
    
    // if let + else
    let some_value = Some(8);
    if let Some(7) = some_value {
        println!("匹配到 7");
    } else {
        println!("没匹配到 7");
    }

    // -------------------------------------------------------------------------
    // 8. while let
    // -------------------------------------------------------------------------
    println!("\n【8. while let】");
    
    let mut optional = Some(0);
    
    while let Some(i) = optional {
        if i > 5 {
            println!("大于 5，退出");
            optional = None;
        } else {
            println!("i = {}", i);
            optional = Some(i + 1);
        }
    }

    // -------------------------------------------------------------------------
    // 9. let else
    // -------------------------------------------------------------------------
    println!("\n【9. let else】");
    
    let some_value: Option<i32> = Some(42);
    
    let Some(value) = some_value else {
        println!("值不存在");
        return;  // 或者 panic!, break 等
    };
    
    println!("值存在: {}", value);

    // -------------------------------------------------------------------------
    // 10. 循环控制
    // -------------------------------------------------------------------------
    println!("\n【10. 循环控制】\n");
    
    // continue - 跳过当前迭代
    for i in 1..=5 {
        if i == 3 {
            continue;
        }
        println!("continue 示例: {}", i);
    }
    
    // break - 退出循环
    for i in 1..=5 {
        if i == 3 {
            break;
        }
        println!("break 示例: {}", i);
    }
    
    // 带值的 break
    let sum = loop {
        let mut s = 0;
        for i in 1..=10 {
            s += i;
        }
        break s;
    };
    println!("带值的 break: {}", sum);

    // -------------------------------------------------------------------------
    // 11. 模式匹配进阶
    // -------------------------------------------------------------------------
    println!("\n【11. 模式匹配进阶】");
    
    // 解构元组
    let point = (3, 5);
    match point {
        (0, y) => println!("在 y 轴上，y = {}", y),
        (x, 0) => println!("在 x 轴上，x = {}", x),
        (x, y) => println!("坐标: ({}, {})", x, y),
    }
    
    // 解构数组
    let arr = [1, 2, 3, 4, 5];
    match arr {
        [1, 2, .., 5] => println!("开头是 1, 2，结尾是 5"),
        [1, ..] => println!("以 1 开头"),
        _ => println!("其他情况"),
    }
    
    // 守卫 (Guard)
    let number = 4;
    match number {
        n if n % 2 == 0 => println!("偶数: {}", n),
        n => println!("奇数: {}", n),
    }
    
    // 绑定值
    match number {
        e @ 1..=5 => println!("1 到 5 之间: {}", e),
        _ => println!("其他"),
    }

    // -------------------------------------------------------------------------
    // 12. 模式匹配 - 结构体
    // -------------------------------------------------------------------------
    println!("\n【12. 结构体模式匹配】");
    
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
    
    // 简写（字段名和变量名相同）
    let Point { x, y } = point;
    println!("解构: x = {}, y = {}", x, y);

    // -------------------------------------------------------------------------
    // 13. matches! 宏
    // -------------------------------------------------------------------------
    println!("\n【13. matches! 宏】");
    
    let some_value = Some(5);
    
    // matches! 返回布尔值
    let is_match = matches!(some_value, Some(5));
    println!("是否匹配 Some(5): {}", is_match);
    
    let is_even = matches!(some_value, Some(n) if n % 2 == 0);
    println!("是否为偶数: {}", is_even);
}
