#![allow(dead_code, unused_variables, unused_imports)]

// Rust 面向对象示例

pub fn run() {
    println!("=== 封装示例 ===");
    encapsulation();
    
    println!("\n=== 多态示例 ===");
    polymorphism();
    
    println!("\n=== 设计模式示例 ===");
    design_patterns();
}

// 封装示例
pub struct User {
    username: String,
    email: String,
    active: bool,
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        User {
            username,
            email,
            active: true,
        }
    }
    
    pub fn username(&self) -> &str {
        &self.username
    }
    
    pub fn email(&self) -> &str {
        &self.email
    }
    
    pub fn is_active(&self) -> bool {
        self.active
    }
}

fn encapsulation() {
    let user = User::new("alice".to_string(), "alice@example.com".to_string());
    println!("用户名: {}", user.username());
    println!("邮箱: {}", user.email());
    println!("活跃: {}", user.is_active());
}

// 多态示例
trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn polymorphism() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle { width: 5.0, height: 3.0 }),
        Box::new(Circle { radius: 2.0 }),
    ];
    
    for shape in &shapes {
        println!("面积: {}", shape.area());
    }
}

// 设计模式示例 - 构建者模式
struct HttpRequest {
    url: String,
    method: String,
    body: Option<String>,
}

impl HttpRequest {
    fn builder(url: &str) -> HttpRequestBuilder {
        HttpRequestBuilder::new(url)
    }
    
    fn send(&self) {
        println!("发送 {} 请求到 {}", self.method, self.url);
    }
}

struct HttpRequestBuilder {
    url: String,
    method: String,
    body: Option<String>,
}

impl HttpRequestBuilder {
    fn new(url: &str) -> Self {
        HttpRequestBuilder {
            url: url.to_string(),
            method: "GET".to_string(),
            body: None,
        }
    }
    
    fn method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self
    }
    
    fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }
    
    fn build(self) -> HttpRequest {
        HttpRequest {
            url: self.url,
            method: self.method,
            body: self.body,
        }
    }
}

fn design_patterns() {
    let request = HttpRequest::builder("https://api.example.com")
        .method("POST")
        .body(r#"{"name": "test"}"#)
        .build();
    
    request.send();
}
