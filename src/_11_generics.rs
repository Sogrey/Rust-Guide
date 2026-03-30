// ============================================================================
// 11_generics.rs - Rust 泛型详解
// ============================================================================

#![allow(dead_code, unused_variables)]

pub fn run() {
    println!("=== Rust 泛型学习 ===\n");

    // -------------------------------------------------------------------------
    // 1. 泛型函数
    // -------------------------------------------------------------------------
    println!("【1. 泛型函数】\n");
    
    // 非泛型函数 - 只能处理特定类型
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    
    // 泛型函数 - 可以处理任何实现了 PartialOrd 和 Copy trait 的类型
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    
    let number_list = vec![34, 50, 25, 100, 65];
    println!("最大整数: {}", largest(&number_list));
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("最大字符: {}", largest(&char_list));

    // -------------------------------------------------------------------------
    // 2. 泛型结构体
    // -------------------------------------------------------------------------
    println!("【2. 泛型结构体】\n");
    
    // 单个泛型参数
    struct Point<T> {
        x: T,
        y: T,
    }
    
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("整数点: ({}, {})", integer_point.x, integer_point.y);
    println!("浮点点: ({}, {})", float_point.x, float_point.y);
    
    // 多个泛型参数
    struct PointMixed<T, U> {
        x: T,
        y: U,
    }
    
    let mixed = PointMixed { x: 5, y: 4.0 };
    println!("混合点: ({}, {})", mixed.x, mixed.y);

    // -------------------------------------------------------------------------
    // 3. 泛型枚举
    // -------------------------------------------------------------------------
    println!("\n【3. 泛型枚举】\n");
    
    // 标准库中的 Option 和 Result 就是泛型枚举
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // 
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    
    // 自定义泛型枚举
    #[derive(Debug)]
    enum Container<T> {
        Single(T),
        Pair(T, T),
        Triple(T, T, T),
    }
    
    let single = Container::Single(42);
    let pair = Container::Pair(1, 2);
    println!("{:?}, {:?}", single, pair);

    // -------------------------------------------------------------------------
    // 4. 泛型方法
    // -------------------------------------------------------------------------
    println!("\n【4. 泛型方法】\n");
    
    struct Point2D<T> {
        x: T,
        y: T,
    }
    
    impl<T> Point2D<T> {
        fn x(&self) -> &T {
            &self.x
        }
        
        fn y(&self) -> &T {
            &self.y
        }
    }
    
    // 为特定类型实现方法
    impl Point2D<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    
    // 多泛型方法
    impl<T, U> PointMixed<T, U> {
        fn mixup<V, W>(self, other: PointMixed<V, W>) -> PointMixed<T, W> {
            PointMixed {
                x: self.x,
                y: other.y,
            }
        }
    }
    
    let p1 = PointMixed { x: 5, y: 10.4 };
    let p2 = PointMixed { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("mixup: ({}, {})", p3.x, p3.y);

    // -------------------------------------------------------------------------
    // 5. Trait Bound (特征约束)
    // -------------------------------------------------------------------------
    println!("\n【5. Trait Bound】\n");
    
    // 使用 where 子句简化复杂约束
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: std::fmt::Display + Clone,
        U: Clone + std::fmt::Debug,
    {
        println!("T: {}, U: {:?}", t, u);
        0
    }
    
    // 条件实现方法
    struct Pair<T> {
        x: T,
        y: T,
    }
    
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    
    impl<T: std::fmt::Display + PartialOrd> Pair<T> {
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
    // 6. 泛型性能
    // -------------------------------------------------------------------------
    println!("\n【6. 泛型性能】\n");
    
    // Rust 的泛型使用单态化 (Monomorphization)
    // 编译时为每个具体类型生成专门的代码
    // 所以泛型代码运行时没有额外开销
    
    // 编译后，以下代码会生成两个版本的函数：
    // - largest_i32
    // - largest_f64
    // 而不是运行时动态分发
    
    println!("泛型在编译时单态化，运行时性能与手写代码相同");

    // -------------------------------------------------------------------------
    // 7. 默认泛型类型参数
    // -------------------------------------------------------------------------
    println!("\n【7. 默认泛型类型参数】\n");
    
    #[derive(Debug, PartialEq)]
    struct Inches(i32);
    
    impl std::ops::Add for Inches {
        type Output = Inches;
        
        fn add(self, rhs: Inches) -> Inches {
            Inches(self.0 + rhs.0)
        }
    }
    
    let height1 = Inches(10);
    let height2 = Inches(5);
    let total = height1 + height2;
    println!("总高度: {:?}", total);

    // -------------------------------------------------------------------------
    // 8. 关联类型
    // -------------------------------------------------------------------------
    println!("\n【8. 关联类型】\n");
    
    // 关联类型是 trait 中的泛型
    pub trait ContainerTrait {
        type Item;  // 关联类型
        
        fn get(&self, index: usize) -> Option<&Self::Item>;
        fn len(&self) -> usize;
    }
    
    struct MyContainer<T> {
        items: Vec<T>,
    }
    
    impl<T> ContainerTrait for MyContainer<T> {
        type Item = T;
        
        fn get(&self, index: usize) -> Option<&Self::Item> {
            self.items.get(index)
        }
        
        fn len(&self) -> usize {
            self.items.len()
        }
    }
    
    let container = MyContainer { items: vec![1, 2, 3] };
    println!("容器长度: {}", container.len());
    println!("第一个元素: {:?}", container.get(0));

    // -------------------------------------------------------------------------
    // 9. 泛型约束示例
    // -------------------------------------------------------------------------
    println!("\n【9. 泛型约束示例】\n");
    
    use std::fmt::Display;
    
    // 多重约束
    fn print_and_compare<T: Display + PartialOrd>(a: T, b: T) {
        println!("比较 {} 和 {}", a, b);
        if a > b {
            println!("{} > {}", a, b);
        } else if a < b {
            println!("{} < {}", a, b);
        } else {
            println!("{} == {}", a, b);
        }
    }
    
    print_and_compare(5, 10);
    print_and_compare("hello", "world");
    
    // where 子句版本
    fn print_and_compare_where<T, U>(a: T, b: T, c: U)
    where
        T: Display + PartialOrd,
        U: Display,
    {
        println!("额外参数: {}", c);
        print_and_compare(a, b);
    }
    
    print_and_compare_where(3, 7, "test");

    // -------------------------------------------------------------------------
    // 10. 泛型与生命周期
    // -------------------------------------------------------------------------
    println!("\n【10. 泛型与生命周期】\n");
    
    // 泛型可以与生命周期一起使用
    fn longest_with_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("公告: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let result = longest_with_announcement("hello", "world!", "比较字符串");
    println!("更长的字符串: {}", result);

    // -------------------------------------------------------------------------
    // 11. Const 泛型
    // -------------------------------------------------------------------------
    println!("\n【11. Const 泛型】\n");
    
    // Const 泛型允许在泛型中使用常量值
    struct Array<T, const N: usize> {
        data: [T; N],
    }
    
    impl<T, const N: usize> Array<T, N> {
        fn len(&self) -> usize {
            N
        }
    }
    
    let arr: Array<i32, 5> = Array { data: [1, 2, 3, 4, 5] };
    println!("数组长度: {}", arr.len());
    
    // 使用 const 泛型参数的函数
    fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("数组: {:?}", arr);
    }
    
    print_array([1, 2, 3]);

    // -------------------------------------------------------------------------
    // 12. 泛型实战示例
    // -------------------------------------------------------------------------
    println!("\n【12. 泛型实战示例】\n");
    
    // 实现一个简单的栈
    struct Stack<T> {
        items: Vec<T>,
    }
    
    impl<T> Stack<T> {
        fn new() -> Self {
            Stack { items: Vec::new() }
        }
        
        fn push(&mut self, item: T) {
            self.items.push(item);
        }
        
        fn pop(&mut self) -> Option<T> {
            self.items.pop()
        }
        
        fn peek(&self) -> Option<&T> {
            self.items.last()
        }
        
        fn is_empty(&self) -> bool {
            self.items.is_empty()
        }
        
        fn len(&self) -> usize {
            self.items.len()
        }
    }
    
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("栈长度: {}", stack.len());
    println!("栈顶: {:?}", stack.peek());
    println!("弹出: {:?}", stack.pop());
    println!("弹出: {:?}", stack.pop());
}
