#![allow(dead_code, unused_variables, unused_imports)]

// Rust 智能指针示例

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};

pub fn run() {
    println!("=== Box<T> ===");
    box_example();
    
    println!("\n=== Rc<T> ===");
    rc_example();
    
    println!("\n=== RefCell<T> ===");
    refcell_example();
    
    println!("\n=== Arc<T> 和 Mutex<T> ===");
    arc_mutex_example();
}

fn box_example() {
    // 堆分配
    let b = Box::new(5);
    println!("Box 值: {}", b);
    
    // 递归类型
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("列表: {:?}", list);
}

fn rc_example() {
    // 引用计数
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);
    
    println!("引用计数: {}", Rc::strong_count(&a));
    println!("值: a={}, b={}, c={}", a, b, c);
}

fn refcell_example() {
    // 内部可变性
    let data = RefCell::new(5);
    
    println!("初始值: {}", *data.borrow());
    
    *data.borrow_mut() += 1;
    println!("修改后: {}", *data.borrow());
}

fn arc_mutex_example() {
    // 线程安全的共享可变状态
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("计数器结果: {}", *counter.lock().unwrap());
}
