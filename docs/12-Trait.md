# Rust 教程 - Trait 详解

## 本章概述

Trait 是 Rust 的共享行为机制。本章详细介绍 trait 的定义、实现、trait bound 和动态分发。

## 学习目标

- 掌握 trait 的定义和实现
- 理解默认实现
- 掌握 trait bound
- 理解关联类型
- 掌握动态分发

## 1. 定义 Trait

```rust
pub trait Summary {
    fn summarize(&self) -> String;
    
    fn read_more(&self) -> String {
        String::from("阅读更多...")
    }
}
```

## 2. 实现 Trait

```rust
struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} - 作者: {}", self.title, self.author)
    }
}
```

## 3. Trait 作为参数

```rust
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}
```

## 4. Trait Bound

```rust
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

## 5. 关联类型

```rust
trait Container {
    type Item;
    fn get(&self, index: usize) -> Option<&Self::Item>;
}
```

## 6. 动态分发

```rust
let items: Vec<Box<dyn Summary>> = vec![
    Box::new(article),
    Box::new(tweet),
];
```

## 下一步

继续学习：[13-生命周期](./13-生命周期.md)
