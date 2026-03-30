#![allow(dead_code, unused_variables, unused_imports)]

// Rust 并发编程示例

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, mpsc};

pub fn run() {
    println!("=== 线程示例 ===");
    threads();
    
    println!("\n=== 消息传递 ===");
    channels();
    
    println!("\n=== 共享状态 ===");
    shared_state();
}

fn threads() {
    // 创建线程
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("子线程: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for i in 1..=3 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(100));
    }
    
    handle.join().unwrap();
    
    // 使用 move 闭包
    let message = String::from("来自线程的消息");
    let handle = thread::spawn(move || {
        println!("{}", message);
    });
    handle.join().unwrap();
}

fn channels() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let messages = vec![
            String::from("你好"),
            String::from("来自"),
            String::from("线程"),
        ];
        
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for received in rx {
        println!("收到: {}", received);
    }
}

fn shared_state() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
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
