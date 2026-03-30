#![allow(dead_code, unused_variables, unused_imports)]

// Rust 文件与 IO 示例

use std::fs;
use std::io::{self, Write, BufRead};

pub fn run() {
    println!("=== 标准输入输出 ===");
    std_io();
    
    println!("\n=== 文件操作 ===");
    file_operations();
    
    println!("\n=== 路径操作 ===");
    path_operations();
}

fn std_io() {
    // 标准输出
    println!("Hello, World!");
    print!("不换行");
    print!(" - 继续\n");
    
    // 标准错误
    eprintln!("错误信息");
    
    // 格式化输出
    let name = "Alice";
    let age = 30;
    println!("姓名: {}, 年龄: {}", name, age);
}

fn file_operations() {
    // 写入文件
    let content = "Hello, Rust!\n这是第二行";
    match fs::write("example.txt", content) {
        Ok(_) => println!("文件写入成功"),
        Err(e) => println!("写入错误: {}", e),
    }
    
    // 读取文件
    match fs::read_to_string("example.txt") {
        Ok(content) => {
            println!("文件内容:");
            println!("{}", content);
        }
        Err(e) => println!("读取错误: {}", e),
    }
    
    // 文件元数据
    match fs::metadata("example.txt") {
        Ok(metadata) => {
            println!("文件大小: {} 字节", metadata.len());
            println!("是文件: {}", metadata.is_file());
        }
        Err(e) => println!("元数据错误: {}", e),
    }
    
    // 清理
    let _ = fs::remove_file("example.txt");
}

use std::path::{Path, PathBuf};

fn path_operations() {
    let path = Path::new("src/main.rs");
    
    // 路径组件
    println!("路径: {:?}", path);
    println!("父目录: {:?}", path.parent());
    println!("文件名: {:?}", path.file_name());
    println!("扩展名: {:?}", path.extension());
    
    // 构建路径
    let mut path_buf = PathBuf::new();
    path_buf.push("src");
    path_buf.push("lib.rs");
    println!("构建路径: {:?}", path_buf);
    
    // 连接路径
    let base = Path::new("/usr/local");
    let full = base.join("bin").join("rust");
    println!("连接路径: {:?}", full);
}
