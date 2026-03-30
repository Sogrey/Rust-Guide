// ============================================================================
// 12_traits.rs - Rust Trait 详解
// ============================================================================

#![allow(dead_code, unused_variables)]

pub fn run() {
    println!("=== Rust Trait 学习 ===\n");

    // -------------------------------------------------------------------------
    // 1. 定义 Trait
    // -------------------------------------------------------------------------
    println!("【1. 定义 Trait】\n");
    
    // Trait 定义共享行为
    pub trait Summary {
        fn summarize(&self) -> String;
        
        // 默认实现
        fn read_more(&self) -> String {
            String::from("阅读更多...")
        }
    }

    // -------------------------------------------------------------------------
    // 2. 为类型实现 Trait
    // -------------------------------------------------------------------------
    println!("【2. 为类型实现 Trait】\n");
    
    struct Article {
        title: String,
        author: String,
        content: String,
    }
    
    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("{} - 作者: {}", self.title, self.author)
        }
    }
    
    struct Tweet {
        username: String,
        content: String,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("@{}: {}", self.username, self.content)
        }
    }
    
    let article = Article {
        title: String::from("Rust 教程"),
        author: String::from("张三"),
        content: String::from("Rust 是一门系统编程语言..."),
    };
    
    let tweet = Tweet {
        username: String::from("rust_fan"),
        content: String::from("Rust 太棒了！"),
    };
    
    println!("文章摘要: {}", article.summarize());
    println!("推文摘要: {}", tweet.summarize());
    println!("默认实现: {}", article.read_more());

    // -------------------------------------------------------------------------
    // 3. Trait 作为参数
    // -------------------------------------------------------------------------
    println!("\n【3. Trait 作为参数】\n");
    
    // impl Trait 语法
    fn notify(item: &impl Summary) {
        println!("通知: {}", item.summarize());
    }
    
    notify(&article);
    notify(&tweet);
    
    // Trait Bound 语法
    fn notify2<T: Summary>(item: &T) {
        println!("通知2: {}", item.summarize());
    }
    
    notify2(&article);

    // -------------------------------------------------------------------------
    // 4. 多个 Trait 约束
    // -------------------------------------------------------------------------
    println!("\n【4. 多个 Trait 约束】\n");
    
    use std::fmt::Display;
    
    // + 语法
    fn notify_display<T: Summary + Display>(item: &T) {
        println!("显示: {}", item);
    }
    
    // where 子句（更清晰）
    fn notify_where<T>(item: &T)
    where
        T: Summary + Display,
    {
        println!("where 子句: {}", item.summarize());
    }

    // -------------------------------------------------------------------------
    // 5. 返回 Trait
    // -------------------------------------------------------------------------
    println!("\n【5. 返回 Trait】\n");
    
    // 返回 impl Trait - 只能返回一种具体类型
    fn returns_summarizable_article() -> impl Summary {
        Article {
            title: String::from("标题"),
            author: String::from("作者"),
            content: String::from("内容"),
        }
    }
    
    // 使用 Box<dyn Trait> 可以返回不同类型
    fn returns_summarizable_box(switch: bool) -> Box<dyn Summary> {
        if switch {
            Box::new(Article {
                title: String::from("标题"),
                author: String::from("作者"),
                content: String::from("内容"),
            })
        } else {
            Box::new(Tweet {
                username: String::from("用户"),
                content: String::from("内容"),
            })
        }
    }
    
    // 注意：不能返回不同类型（需要使用 Box<dyn Trait>）
    // 错误示例：
    // fn returns_summarizable(switch: bool) -> impl Summary {
    //     if switch {
    //         Article { ... }
    //     } else {
    //         Tweet { ... }  // 错误！不同类型
    //     }
    // }

    // -------------------------------------------------------------------------
    // 6. Trait Bound 条件实现
    // -------------------------------------------------------------------------
    println!("\n【6. Trait Bound 条件实现】\n");
    
    struct Pair<T> {
        x: T,
        y: T,
    }
    
    // 为所有 Pair<T> 实现 new 方法
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    
    // 只为实现了 Display + PartialOrd 的类型实现 cmp_display
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("x >= y: {} >= {}", self.x, self.y);
            } else {
                println!("x < y: {} < {}", self.x, self.y);
            }
        }
    }
    
    let pair = Pair::new(5, 10);
    pair.cmp_display();

    // -------------------------------------------------------------------------
    // 7. 关联类型
    // -------------------------------------------------------------------------
    println!("\n【7. 关联类型】\n");
    
    // Iterator trait 的关联类型
    pub trait Iterator {
        type Item;  // 关联类型
        
        fn next(&mut self) -> Option<Self::Item>;
    }
    
    // 实现关联类型
    struct Counter {
        count: usize,
        max: usize,
    }
    
    impl Counter {
        fn new(max: usize) -> Counter {
            Counter { count: 0, max }
        }
    }
    
    impl Iterator for Counter {
        type Item = usize;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
    
    let mut counter = Counter::new(5);
    while let Some(n) = counter.next() {
        print!("{} ", n);
    }
    println!();

    // -------------------------------------------------------------------------
    // 8. 默认泛型类型参数
    // -------------------------------------------------------------------------
    println!("\n【8. 默认泛型类型参数】\n");
    
    // 标准库中的 Add trait
    // trait Add<Rhs=Self> {
    //     type Output;
    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }
    
    #[derive(Debug)]
    struct Millimeters(u32);
    struct Meters(u32);
    
    impl std::ops::Add for Millimeters {
        type Output = Millimeters;
        
        fn add(self, other: Millimeters) -> Millimeters {
            Millimeters(self.0 + other.0)
        }
    }
    
    // 混合类型相加
    impl std::ops::Add<Meters> for Millimeters {
        type Output = Millimeters;
        
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
    
    let mm1 = Millimeters(500);
    let mm2 = Millimeters(300);
    println!("500mm + 300mm = {:?}", mm1 + mm2);
    
    let mm3 = Millimeters(500);
    let m = Meters(1);
    println!("500mm + 1m = {:?}", mm3 + m);

    // -------------------------------------------------------------------------
    // 9. 完全限定语法
    // -------------------------------------------------------------------------
    println!("\n【9. 完全限定语法】\n");
    
    trait Pilot {
        fn fly(&self);
    }
    
    trait Wizard {
        fn fly(&self);
    }
    
    struct Human;
    
    impl Pilot for Human {
        fn fly(&self) {
            println!("机长正在驾驶飞机");
        }
    }
    
    impl Wizard for Human {
        fn fly(&self) {
            println!("巫师正在飞行");
        }
    }
    
    impl Human {
        fn fly(&self) {
            println!("人类正在挥动双臂");
        }
    }
    
    let person = Human;
    
    // 直接调用会调用类型自身的方法
    person.fly();
    
    // 使用完全限定语法调用特定 trait 的方法
    Pilot::fly(&person);
    Wizard::fly(&person);
    <Human as Pilot>::fly(&person);  // 最明确的写法

    // -------------------------------------------------------------------------
    // 10. Supertrait（父 Trait）
    // -------------------------------------------------------------------------
    println!("\n【10. Supertrait】\n");
    
    // 要求实现 trait 的类型也实现另一个 trait
    trait OutlinePrint: std::fmt::Debug {
        fn outline_print(&self) {
            println!("* {:?} *", self);
            println!("*************");
        }
    }
    
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl OutlinePrint for Point {}
    
    let point = Point { x: 10, y: 20 };
    point.outline_print();

    // -------------------------------------------------------------------------
    // 11. Newtype 模式
    // -------------------------------------------------------------------------
    println!("\n【11. Newtype 模式】\n");
    
    // 为外部类型实现外部 trait
    // Rust 不允许为外部类型实现外部 trait（孤儿规则）
    // 解决方案：使用 Newtype 模式
    
    struct Wrapper(Vec<String>);
    
    impl std::fmt::Display for Wrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("Wrapper: {}", w);

    // -------------------------------------------------------------------------
    // 12. 动态分发 (dyn Trait)
    // -------------------------------------------------------------------------
    println!("\n【12. 动态分发】\n");
    
    // 静态分发（编译时确定）
    fn static_dispatch<T: Summary>(item: &T) {
        println!("{}", item.summarize());
    }
    
    // 动态分发（运行时确定）
    fn dynamic_dispatch(item: &dyn Summary) {
        println!("{}", item.summarize());
    }
    
    // 使用 trait 对象
    let articles: Vec<Box<dyn Summary>> = vec![
        Box::new(Article {
            title: String::from("标题1"),
            author: String::from("作者1"),
            content: String::from("内容1"),
        }),
        Box::new(Tweet {
            username: String::from("用户1"),
            content: String::from("内容1"),
        }),
    ];
    
    for article in &articles {
        println!("动态分发: {}", article.summarize());
    }

    // -------------------------------------------------------------------------
    // 13. 常用标准库 Trait
    // -------------------------------------------------------------------------
    println!("\n【13. 常用标准库 Trait】\n");
    
    // Debug - 调试格式
    // Display - 用户显示格式
    // Clone - 深拷贝
    // Copy - 栈上拷贝
    // PartialEq, Eq - 比较
    // PartialOrd, Ord - 排序
    // Default - 默认值
    // From, Into - 类型转换
    // AsRef, AsMut - 引用转换
    // Deref, DerefMut - 智能指针
    
    #[derive(Debug, Clone, PartialEq, Default)]
    struct Student {
        name: String,
        age: u32,
    }
    
    let s1 = Student::default();
    let s2 = s1.clone();
    println!("s1 == s2: {}", s1 == s2);
    println!("默认学生: {:?}", s1);

    // -------------------------------------------------------------------------
    // 14. Trait 继承
    // -------------------------------------------------------------------------
    println!("\n【14. Trait 继承】\n");
    
    trait Animal {
        fn name(&self) -> String;
    }
    
    trait Dog: Animal {
        fn breed(&self) -> String;
    }
    
    struct Labrador {
        name: String,
    }
    
    impl Animal for Labrador {
        fn name(&self) -> String {
            self.name.clone()
        }
    }
    
    impl Dog for Labrador {
        fn breed(&self) -> String {
            String::from("拉布拉多")
        }
    }
    
    let dog = Labrador { name: String::from("旺财") };
    println!("狗的名字: {}, 品种: {}", dog.name(), dog.breed());
}
