// ============================================================================
// 06_structs.rs - Rust 结构体详解
// ============================================================================

#![allow(dead_code, unused_variables)]

pub fn run() {
    println!("=== Rust 结构体学习 ===\n");

    // -------------------------------------------------------------------------
    // 1. 定义和实例化结构体
    // -------------------------------------------------------------------------
    println!("【1. 定义和实例化结构体】");
    
    // 定义结构体
    struct User {
        username: String,
        email: String,
        active: bool,
        sign_in_count: u64,
    }
    
    // 创建实例
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("user123"),
        active: true,
        sign_in_count: 1,
    };
    
    // 访问字段
    println!("用户名: {}", user1.username);
    println!("邮箱: {}", user1.email);

    // -------------------------------------------------------------------------
    // 2. 可变结构体
    // -------------------------------------------------------------------------
    println!("\n【2. 可变结构体】");
    
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("another"),
        active: true,
        sign_in_count: 1,
    };
    
    user2.email = String::from("new_email@example.com");
    println!("新邮箱: {}", user2.email);

    // -------------------------------------------------------------------------
    // 3. 结构体更新语法
    // -------------------------------------------------------------------------
    println!("\n【3. 结构体更新语法】");
    
    let user3 = User {
        email: String::from("new@example.com"),
        ..user1  // 其余字段来自 user1
    };
    
    println!("user3 邮箱: {}, 用户名: {}", user3.email, user3.username);
    // 注意：username 已移动到 user3，user1 部分失效
    // println!("{}", user1.username);  // 错误！

    // -------------------------------------------------------------------------
    // 4. 元组结构体
    // -------------------------------------------------------------------------
    println!("\n【4. 元组结构体】");
    
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("黑色: ({}, {}, {})", black.0, black.1, black.2);
    println!("原点: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // Color 和 Point 是不同类型，即使字段相同

    // -------------------------------------------------------------------------
    // 5. 单元结构体
    // -------------------------------------------------------------------------
    println!("\n【5. 单元结构体】");
    
    struct AlwaysEqual;
    
    let _subject = AlwaysEqual;
    println!("单元结构体没有字段，常用于实现 trait");

    // -------------------------------------------------------------------------
    // 6. 结构体方法
    // -------------------------------------------------------------------------
    println!("\n【6. 结构体方法】");
    
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        // 方法 - 获取面积
        // &self 是 self: &Self 的缩写
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        // 方法 - 是否为正方形
        fn is_square(&self) -> bool {
            self.width == self.height
        }
        
        // 方法 - 调整大小（可变借用）
        fn resize(&mut self, width: u32, height: u32) {
            self.width = width;
            self.height = height;
        }
        
        // 关联函数 - 创建正方形
        // 没有 self 参数
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
        
        // 关联函数 - 创建默认矩形
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
    }
    
    let rect = Rectangle::new(10, 20);
    println!("面积: {}", rect.area());
    println!("是否为正方形: {}", rect.is_square());
    
    let square = Rectangle::square(5);
    println!("正方形面积: {}", square.area());

    // -------------------------------------------------------------------------
    // 7. 多个 impl 块
    // -------------------------------------------------------------------------
    println!("\n【7. 多个 impl 块】");
    
    impl Rectangle {
        fn width(&self) -> u32 {
            self.width
        }
        
        fn height(&self) -> u32 {
            self.height
        }
    }
    
    // 可以有多个 impl 块（通常用于组织代码或泛型实现）
    println!("宽度: {}, 高度: {}", rect.width(), rect.height());

    // -------------------------------------------------------------------------
    // 8. 结构体打印和调试
    // -------------------------------------------------------------------------
    println!("\n【8. 结构体打印】");
    
    #[derive(Debug)]  // 自动实现 Debug trait
    struct Person {
        name: String,
        age: u32,
    }
    
    let person = Person {
        name: String::from("张三"),
        age: 25,
    };
    
    // Debug 格式打印
    println!("Debug 格式: {:?}", person);
    println!("美化 Debug: {:#?}", person);

    // -------------------------------------------------------------------------
    // 9. 结构体与所有权
    // -------------------------------------------------------------------------
    println!("\n【9. 结构体与所有权】");
    
    struct Book {
        title: String,  // 拥有 String 的所有权
        author: String,
        pages: u32,
    }
    
    let book = Book {
        title: String::from("Rust 程序设计语言"),
        author: String::from("Rust 社区"),
        pages: 300,
    };
    
    // 存储引用需要在结构体中定义生命周期
    // struct BookRef<'a> {
    //     title: &'a str,
    // }

    // -------------------------------------------------------------------------
    // 10. 结构体示例 - 构建系统
    // -------------------------------------------------------------------------
    println!("\n【10. 综合示例】");
    
    #[derive(Debug)]
    struct Building {
        name: String,
        floors: u32,
        elevators: u32,
        address: Address,
    }
    
    #[derive(Debug)]
    struct Address {
        city: String,
        street: String,
        number: u32,
    }
    
    impl Address {
        fn new(city: &str, street: &str, number: u32) -> Address {
            Address {
                city: String::from(city),
                street: String::from(street),
                number,
            }
        }
        
        fn full_address(&self) -> String {
            format!("{}市 {}街 {}号", self.city, self.street, self.number)
        }
    }
    
    impl Building {
        fn new(name: &str, floors: u32, elevators: u32, address: Address) -> Building {
            Building {
                name: String::from(name),
                floors,
                elevators,
                address,
            }
        }
        
        fn info(&self) {
            println!("建筑: {}", self.name);
            println!("楼层数: {}", self.floors);
            println!("电梯数: {}", self.elevators);
            println!("地址: {}", self.address.full_address());
        }
        
        fn is_skyscraper(&self) -> bool {
            self.floors >= 40
        }
    }
    
    let address = Address::new("北京", "长安", 1);
    let building = Building::new("国贸大厦", 81, 20, address);
    
    building.info();
    println!("是否为摩天大楼: {}", building.is_skyscraper());

    // -------------------------------------------------------------------------
    // 11. 结构体更新与解构
    // -------------------------------------------------------------------------
    println!("\n【11. 结构体解构】");
    
    struct Point2D {
        x: i32,
        y: i32,
    }
    
    let point = Point2D { x: 10, y: 20 };
    
    // 解构
    let Point2D { x, y } = point;
    println!("x = {}, y = {}", x, y);
    
    // 在 match 中解构
    match point {
        Point2D { x: 0, y } => println!("在 y 轴上"),
        Point2D { x, y: 0 } => println!("在 x 轴上"),
        Point2D { x, y } => println!("坐标: ({}, {})", x, y),
    }

    // -------------------------------------------------------------------------
    // 12. 常用派生 Trait
    // -------------------------------------------------------------------------
    println!("\n【12. 常用派生 Trait】");
    
    #[derive(Debug, Clone, PartialEq)]
    struct Vector2D {
        x: f64,
        y: f64,
    }
    
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = v1.clone();
    let v3 = Vector2D { x: 1.0, y: 2.0 };
    
    println!("v1 == v2: {}", v1 == v2);  // PartialEq
    println!("v1 == v3: {}", v1 == v3);
    println!("Debug: {:?}", v1);
}
