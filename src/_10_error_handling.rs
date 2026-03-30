// ============================================================================
// 10_error_handling.rs - Rust 错误处理详解
// ============================================================================

pub fn run() {
    println!("=== Rust 错误处理学习 ===\n");

    // -------------------------------------------------------------------------
    // 1. 不可恢复错误 - panic!
    // -------------------------------------------------------------------------
    println!("【1. panic! - 不可恢复错误】\n");
    
    // panic! 会导致程序立即终止
    // panic!("程序崩溃！");
    
    // 常见的 panic 场景
    let v = vec![1, 2, 3];
    // v[10];  // 索引越界会 panic
    
    // 使用 RUST_BACKTRACE=1 环境变量获取调用栈

    // -------------------------------------------------------------------------
    // 2. Result<T, E> 枚举
    // -------------------------------------------------------------------------
    println!("【2. Result<T, E> 枚举】\n");
    
    // Result 用于可恢复的错误
    // pub enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    
    use std::fs::File;
    use std::io::ErrorKind;
    
    // 打开文件可能失败
    let f = File::open("hello.txt");
    
    let f = match f {
        Ok(file) => {
            println!("文件打开成功");
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("文件不存在，创建新文件");
                File::create("hello.txt").unwrap()
            }
            other_error => {
                panic!("打开文件失败: {:?}", other_error);
            }
        }
    };

    // -------------------------------------------------------------------------
    // 3. unwrap 和 expect
    // -------------------------------------------------------------------------
    println!("\n【3. unwrap 和 expect】\n");
    
    // unwrap：成功返回值，失败 panic
    // let f = File::open("hello.txt").unwrap();
    
    // expect：可以自定义错误信息
    // let f = File::open("hello.txt").expect("无法打开 hello.txt");
    
    // 推荐使用 expect 而不是 unwrap

    // -------------------------------------------------------------------------
    // 4. 错误传播 - ?
    // -------------------------------------------------------------------------
    println!("\n【4. 错误传播 - ? 操作符】\n");
    
    use std::io::{self, Read};
    
    // 传统方式
    fn read_username_from_file_old() -> Result<String, io::Error> {
        let f = File::open("username.txt");
        
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        
        let mut s = String::new();
        
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    
    // 使用 ? 操作符简化
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("username.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    
    // 更简洁的写法（链式调用）
    fn read_username_from_file_short() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("username.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
    
    // ? 操作符的工作原理：
    // 如果 Result 是 Ok，返回内部的值
    // 如果 Result 是 Err，将错误返回给调用者

    // -------------------------------------------------------------------------
    // 5. 自定义错误类型
    // -------------------------------------------------------------------------
    println!("\n【5. 自定义错误类型】\n");
    
    use std::fmt;
    use std::error::Error;
    
    #[derive(Debug)]
    struct MyError {
        message: String,
    }
    
    impl MyError {
        fn new(msg: &str) -> MyError {
            MyError {
                message: msg.to_string(),
            }
        }
    }
    
    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "MyError: {}", self.message)
        }
    }
    
    impl Error for MyError {}
    
    fn do_something(flag: bool) -> Result<(), MyError> {
        if flag {
            Ok(())
        } else {
            Err(MyError::new("操作失败"))
        }
    }
    
    match do_something(false) {
        Ok(_) => println!("成功"),
        Err(e) => println!("错误: {}", e),
    }

    // -------------------------------------------------------------------------
    // 6. 错误类型转换
    // -------------------------------------------------------------------------
    println!("\n【6. 错误类型转换】\n");
    
    // 使用 Box<dyn Error> 作为通用错误类型
    fn generic_error() -> Result<(), Box<dyn Error>> {
        let _f = File::open("nonexistent.txt")?;  // io::Error 自动转换
        Ok(())
    }
    
    // 使用 anyhow crate（推荐）
    // 使用 thiserror crate 创建自定义错误（推荐）

    // -------------------------------------------------------------------------
    // 7. Option 和 Result 转换
    // -------------------------------------------------------------------------
    println!("\n【7. Option 和 Result 转换】\n");
    
    // Option -> Result
    let some: Option<i32> = Some(5);
    let result: Result<i32, &str> = some.ok_or("没有值");
    println!("Option -> Result: {:?}", result);
    
    let none: Option<i32> = None;
    let result: Result<i32, &str> = none.ok_or("没有值");
    println!("None -> Result: {:?}", result);
    
    // Result -> Option
    let ok: Result<i32, &str> = Ok(5);
    let option = ok.ok();
    println!("Result -> Option: {:?}", option);
    
    // 转换错误类型
    let err: Result<i32, &str> = Err("错误");
    let err_mapped: Result<i32, i32> = err.map_err(|e| e.len() as i32);
    println!("错误映射: {:?}", err_mapped);

    // -------------------------------------------------------------------------
    // 8. 错误处理最佳实践
    // -------------------------------------------------------------------------
    println!("\n【8. 错误处理最佳实践】\n");
    
    // 1. 库代码：返回 Result，让调用者决定如何处理
    fn library_function(x: i32) -> Result<i32, String> {
        if x < 0 {
            Err("负数不允许".to_string())
        } else {
            Ok(x * 2)
        }
    }
    
    // 2. 应用代码：遇到错误可以 panic 或优雅处理
    match library_function(5) {
        Ok(result) => println!("结果: {}", result),
        Err(e) => eprintln!("错误: {}", e),
    }
    
    // 3. 使用 ? 传播错误
    fn app_function() -> Result<(), String> {
        let result = library_function(-1)?;
        println!("结果: {}", result);
        Ok(())
    }
    
    // 4. 提供有用的错误信息
    println!("错误信息应包含：发生了什么、为什么发生、如何修复");

    // -------------------------------------------------------------------------
    // 9. 常用方法
    // -------------------------------------------------------------------------
    println!("\n【9. Result 常用方法】\n");
    
    let ok: Result<i32, &str> = Ok(5);
    let err: Result<i32, &str> = Err("错误");
    
    // is_ok / is_err
    println!("ok.is_ok(): {}", ok.is_ok());
    println!("err.is_err(): {}", err.is_err());
    
    // ok / err
    println!("ok.ok(): {:?}", ok.ok());
    println!("err.err(): {:?}", err.err());
    
    // unwrap_or / unwrap_or_else / unwrap_or_default
    println!("err.unwrap_or(0): {}", err.unwrap_or(0));
    println!("err.unwrap_or_else(|_| 10): {}", err.unwrap_or_else(|_| 10));
    
    // map / map_err
    let doubled = ok.map(|x| x * 2);
    println!("ok.map(|x| x * 2): {:?}", doubled);
    
    // and_then
    let chained = ok.and_then(|x| {
        if x > 3 {
            Ok(x * 2)
        } else {
            Err("值太小")
        }
    });
    println!("and_then: {:?}", chained);

    // -------------------------------------------------------------------------
    // 10. panic vs Result
    // -------------------------------------------------------------------------
    println!("\n【10. panic vs Result 使用场景】\n");
    
    // 使用 panic! 的场景：
    // - 示例代码、原型代码
    // - 测试代码
    // - 程序状态无法恢复
    // - 调用者违反契约（如传入无效参数）
    
    // 使用 Result 的场景：
    // - 预期可能失败的错误
    // - I/O 操作
    // - 解析用户输入
    // - 网络请求
    
    println!("原则：让不可能发生的事 panic，让可能发生的事返回 Result");

    // -------------------------------------------------------------------------
    // 11. 错误处理示例
    // -------------------------------------------------------------------------
    println!("\n【11. 完整示例】\n");
    
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeSqrt,
    }
    
    impl fmt::Display for MathError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                MathError::DivisionByZero => write!(f, "除数不能为零"),
                MathError::NegativeSqrt => write!(f, "不能对负数开平方"),
            }
        }
    }
    
    impl Error for MathError {}
    
    fn divide(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
    
    fn sqrt(n: f64) -> Result<f64, MathError> {
        if n < 0.0 {
            Err(MathError::NegativeSqrt)
        } else {
            Ok(n.sqrt())
        }
    }
    
    fn complex_calculation(a: f64, b: f64) -> Result<f64, MathError> {
        let quotient = divide(a, b)?;
        let root = sqrt(quotient)?;
        Ok(root)
    }
    
    match complex_calculation(16.0, 4.0) {
        Ok(result) => println!("计算结果: {}", result),
        Err(e) => eprintln!("计算错误: {}", e),
    }
    
    match complex_calculation(16.0, 0.0) {
        Ok(result) => println!("计算结果: {}", result),
        Err(e) => eprintln!("计算错误: {}", e),
    }
}
