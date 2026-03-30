#![allow(dead_code, unused_variables, unused_imports)]

// Rust 迭代器示例

pub fn run() {
    println!("=== 迭代器基础 ===");
    iterator_basics();
    
    println!("\n=== 消费者方法 ===");
    consumer_methods();
    
    println!("\n=== 适配器方法 ===");
    adapter_methods();
    
    println!("\n=== 自定义迭代器 ===");
    custom_iterator();
}

fn iterator_basics() {
    // 从数组创建迭代器
    let arr = [1, 2, 3, 4, 5];
    let iter = arr.iter();
    
    // 手动迭代
    let mut manual_iter = arr.iter();
    println!("手动迭代: {:?}", manual_iter.next());
    
    // for 循环
    for value in &arr {
        println!("值: {}", value);
    }
}

fn consumer_methods() {
    let v = vec![1, 2, 3, 4, 5];
    
    // collect
    let collected: Vec<i32> = v.iter().cloned().collect();
    println!("收集: {:?}", collected);
    
    // fold
    let sum = v.iter().fold(0, |acc, &x| acc + x);
    println!("求和: {}", sum);
    
    // count, sum, product
    println!("计数: {}", v.iter().count());
    println!("总和: {}", v.iter().sum::<i32>());
    println!("乘积: {}", v.iter().product::<i32>());
    
    // min, max
    println!("最小: {:?}", v.iter().min());
    println!("最大: {:?}", v.iter().max());
}

fn adapter_methods() {
    let v = vec![1, 2, 3, 4, 5];
    
    // map
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("加倍: {:?}", doubled);
    
    // filter
    let evens: Vec<i32> = v.iter().cloned().filter(|x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);
    
    // enumerate
    for (i, v) in v.iter().enumerate() {
        println!("索引 {}: {}", i, v);
    }
    
    // zip
    let names = vec!["Alice", "Bob"];
    let ages = vec![30, 25];
    let pairs: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("配对: {:?}", pairs);
}

// 自定义迭代器
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

fn custom_iterator() {
    let counter = Counter::new(5);
    let values: Vec<usize> = counter.collect();
    println!("自定义迭代器: {:?}", values);
}
