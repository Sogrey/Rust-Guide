// ============================================================================
// 13_lifetimes.rs - Rust 生命周期详解
// ============================================================================

pub fn run() {
    println!("=== Rust 生命周期学习 ===\n");

    // -------------------------------------------------------------------------
    // 1. 生命周期概念
    // -------------------------------------------------------------------------
    println!("【1. 生命周期概念】\n");
    
    // 生命周期是引用有效的范围
    // Rust 使用生命周期确保引用始终有效
    
    {
        let r;  // ---------+-- 'a
                //          |
        {       //          |
            let x = 5;     // ---+-- 'b
            r = &x;        //    |
        }       // ---------+
                //          |
        // println!("r: {}", r);  // 错误！x 已经离开作用域
    }
    
    // 正确的写法
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+

    // -------------------------------------------------------------------------
    // 2. 函数中的生命周期
    // -------------------------------------------------------------------------
    println!("【2. 函数中的生命周期】\n");
    
    // 生命周期注解语法：'a, 'b, 'static 等
    
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("最长的字符串: {}", result);
    }
    // 这里 result 仍然有效，因为 string1 足够长

    // -------------------------------------------------------------------------
    // 3. 生命周期注解语法
    // -------------------------------------------------------------------------
    println!("\n【3. 生命周期注解语法】\n");
    
    // 生命周期注解不改变引用存活的时间
    // 它们描述多个引用之间的关系
    
    // &i32        - 普通引用
    // &'a i32     - 带有显式生命周期的引用
    // &'a mut i32 - 带有显式生命周期的可变引用
    
    fn first_word<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    let text = "hello world";
    let word = first_word(text);
    println!("第一个单词: {}", word);

    // -------------------------------------------------------------------------
    // 4. 结构体中的生命周期
    // -------------------------------------------------------------------------
    println!("\n【4. 结构体中的生命周期】\n");
    
    // 结构体存储引用需要指定生命周期
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("重要摘录: {}", excerpt.part);
    
    // 结构体方法的生命周期
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
        
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("公告: {}", announcement);
            self.part
        }
    }
    
    println!("级别: {}", excerpt.level());
    println!("部分: {}", excerpt.announce_and_return_part("注意"));

    // -------------------------------------------------------------------------
    // 5. 生命周期省略规则
    // -------------------------------------------------------------------------
    println!("\n【5. 生命周期省略规则】\n");
    
    // 编译器使用三条规则推断生命周期：
    // 
    // 规则 1：每个引用参数都获得一个生命周期
    //         fn foo(x: &str) -> fn foo<'a>(x: &'a str)
    // 
    // 规则 2：如果只有一个输入生命周期参数，它被赋予所有输出生命周期参数
    //         fn foo(x: &str) -> &str 等于 fn foo<'a>(x: &'a str) -> &'a str
    // 
    // 规则 3：如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self，
    //         self 的生命周期被赋予所有输出生命周期参数
    
    // 这些规则使得很多情况下不需要显式标注生命周期
    fn first_word_auto(s: &str) -> &str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }

    // -------------------------------------------------------------------------
    // 6. 'static 生命周期
    // -------------------------------------------------------------------------
    println!("\n【6】'static 生命周期】\n");
    
    // 'static 表示整个程序运行期间都有效
    // 所有字符串字面量都有 'static 生命周期
    
    let s: &'static str = "我有 static 生命周期";
    println!("{}", s);
    
    // 常量也有 'static 生命周期
    const MAX_POINTS: u32 = 100_000;

    // -------------------------------------------------------------------------
    // 7. 多个生命周期参数
    // -------------------------------------------------------------------------
    println!("\n【7. 多个生命周期参数】\n");
    
    fn longest_with_different_lifetimes<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
    where
        'b: 'a,  // 'b 比 'a 活得更久
    {
        println!("比较字符串");
        if x.len() > y.len() {
            x
        } else {
            x  // 返回 x 因为它的生命周期是 'a
        }
    }
    
    // 更实际的例子
    fn longest_with_announcement<'a, 'b>(
        x: &'a str,
        y: &'b str,
        ann: &str,
    ) -> &'a str
    where
        'a: 'b,  // 'a 比 'b 活得更久
    {
        println!("公告: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            x
        }
    }

    // -------------------------------------------------------------------------
    // 8. 生命周期与泛型
    // -------------------------------------------------------------------------
    println!("\n【8. 生命周期与泛型】\n");
    
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: std::fmt::Display,
    {
        println!("公告: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let result = longest_with_an_announcement("hello", "world", "比较中");
    println!("结果: {}", result);

    // -------------------------------------------------------------------------
    // 9. 生命周期子类型化
    // -------------------------------------------------------------------------
    println!("\n【9. 生命周期子类型化】\n");
    
    // 'static: 'a: 'b
    // 'static 是所有生命周期的子类型
    // 可以将 'static 引用赋值给任何生命周期引用
    
    fn assign_static_to_lifetime<'a>() -> &'a str {
        "static 字符串"  // &'static str 可以转换为 &'a str
    }
    
    let s = assign_static_to_lifetime();
    println!("静态字符串: {}", s);

    // -------------------------------------------------------------------------
    // 10. 协变和逆变
    // -------------------------------------------------------------------------
    println!("\n【10. 协变和逆变】\n");
    
    // 协变：如果 'a: 'b，则 &'a T: &'b T
    // 可以将长生命周期的引用赋值给短生命周期引用的变量
    
    fn covariant_example<'a, 'b>(x: &'a str)
    where
        'a: 'b,
    {
        let _y: &'b str = x;  // 合法！'a 比 'b 长
    }

    // -------------------------------------------------------------------------
    // 11. 生命周期实战示例
    // -------------------------------------------------------------------------
    println!("\n【11. 实战示例】\n");
    
    // 解析器示例
    struct Parser<'a> {
        input: &'a str,
    }
    
    impl<'a> Parser<'a> {
        fn new(input: &'a str) -> Self {
            Parser { input }
        }
        
        fn parse_word(&mut self) -> Option<&'a str> {
            let word_end = self.input.find(|c: char| c.is_whitespace())
                .unwrap_or(self.input.len());
            
            if word_end == 0 {
                return None;
            }
            
            let word = &self.input[..word_end];
            self.input = &self.input[word_end..].trim_start();
            Some(word)
        }
    }
    
    let mut parser = Parser::new("hello world from rust");
    while let Some(word) = parser.parse_word() {
        println!("解析单词: {}", word);
    }

    // -------------------------------------------------------------------------
    // 12. 生命周期约束
    // -------------------------------------------------------------------------
    println!("\n【12. 生命周期约束】\n");
    
    // T: 'a 表示 T 中的所有引用都必须比 'a 活得长
    fn print_ref<'a, T>(x: &'a T)
    where
        T: std::fmt::Debug + 'a,
    {
        println!("{:?}", x);
    }
    
    print_ref(&42);
    
    // 结构体约束
    struct RefWrapper<'a, T: 'a> {
        reference: &'a T,
    }
    
    let value = 42;
    let wrapper = RefWrapper { reference: &value };
    println!("包装器中的值: {}", wrapper.reference);

    // -------------------------------------------------------------------------
    // 13. 高阶生命周期 (HRTB)
    // -------------------------------------------------------------------------
    println!("\n【13. 高阶生命周期】\n");
    
    // 高阶 Trait Bound (HRTB) - for<'a>
    // 表示"对于所有生命周期 'a"
    
    fn call_with_ref<F>(f: F)
    where
        F: for<'a> Fn(&'a i32),
    {
        let value = 42;
        f(&value);
    }
    
    call_with_ref(|x| println!("值: {}", x));

    // -------------------------------------------------------------------------
    // 14. 常见生命周期模式
    // -------------------------------------------------------------------------
    println!("\n【14. 常见生命周期模式】\n");
    
    // 模式 1：输入生命周期 = 输出生命周期
    // fn foo<'a>(x: &'a str) -> &'a str
    
    // 模式 2：多个输入，返回其中一个
    // fn foo<'a>(x: &'a str, y: &'a str) -> &'a str
    
    // 模式 3：返回新分配的数据（不涉及生命周期）
    // fn foo(x: &str) -> String
    
    // 模式 4：回调函数
    // fn foo<F>(f: F) where F: for<'a> Fn(&'a str)
    
    println!("生命周期确保引用始终有效，防止悬垂引用");
    println!("大多数情况下，编译器可以自动推断生命周期");
}
