// ============================================================================
// 08_collections.rs - Rust 常见集合详解
// ============================================================================

pub fn run() {
    println!("=== Rust 常见集合学习 ===\n");

    // -------------------------------------------------------------------------
    // 1. Vector (Vec<T>)
    // -------------------------------------------------------------------------
    println!("【1. Vector - 动态数组】\n");
    
    // 创建 Vector
    let v: Vec<i32> = Vec::new();  // 空向量
    let v = vec![1, 2, 3, 4, 5];   // 使用宏创建
    
    // 添加元素
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vector: {:?}", v);
    
    // 读取元素
    let third = &v[2];  // 索引访问（可能 panic）
    println!("第三个元素: {}", third);
    
    let third = v.get(2);  // get 方法（返回 Option）
    match third {
        Some(n) => println!("第三个元素: {}", n),
        None => println!("没有第三个元素"),
    }
    
    // 遍历
    print!("遍历: ");
    for i in &v {
        print!("{} ", i);
    }
    println!();
    
    // 可变遍历
    for i in &mut v {
        *i *= 2;  // 修改元素
    }
    println!("修改后: {:?}", v);
    
    // 存储多种类型（使用枚举）
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    println!("多类型 Vector: {:?}", row);
    
    // 常用方法
    let mut v = vec![1, 2, 3, 4, 5];
    v.pop();  // 移除最后一个元素
    v.remove(0);  // 移除指定索引
    v.insert(0, 0);  // 在指定位置插入
    println!("操作后: {:?}", v);
    
    println!("长度: {}, 容量: {}", v.len(), v.capacity());

    // -------------------------------------------------------------------------
    // 2. String
    // -------------------------------------------------------------------------
    println!("\n【2. String - 字符串】\n");
    
    // 创建字符串
    let s1 = String::new();  // 空字符串
    let s2 = "initial contents".to_string();  // 从 &str 创建
    let s3 = String::from("initial contents");  // 从字面量创建
    
    // 更新字符串
    let mut s = String::from("foo");
    s.push_str("bar");  // 追加字符串切片
    s.push('!');  // 追加字符
    println!("更新后: {}", s);
    
    // 拼接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 被移动，s2 被借用
    println!("拼接: {}", s3);
    
    // 使用 format! 宏
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("format!: {}", s);
    
    // 字符串切片（注意 UTF-8 边界）
    let hello = "你好";
    let s = &hello[0..3];  // 一个中文字符
    println!("切片: {}", s);
    
    // 遍历字符
    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    println!();
    
    // 遍历字节
    for b in "hello".bytes() {
        print!("{} ", b);
    }
    println!();
    
    // 常用方法
    let mut s = String::from("hello");
    s.insert(5, ' ');  // 插入字符
    s.insert_str(6, "world");  // 插入字符串
    println!("插入后: {}", s);
    
    println!("包含 'world': {}", s.contains("world"));
    println!("以 'hello' 开头: {}", s.starts_with("hello"));
    println!("以 'world' 结尾: {}", s.ends_with("world"));
    
    // 替换
    let s = "I like dogs";
    println!("替换: {}", s.replace("dogs", "cats"));
    
    // 分割
    let s = "hello,world,rust";
    let parts: Vec<&str> = s.split(',').collect();
    println!("分割: {:?}", parts);

    // -------------------------------------------------------------------------
    // 3. HashMap
    // -------------------------------------------------------------------------
    println!("\n【3. HashMap - 哈希映射】\n");
    
    use std::collections::HashMap;
    
    // 创建 HashMap
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    println!("HashMap: {:?}", scores);
    
    // 从 Vector 创建
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("从 Vector 创建: {:?}", scores);
    
    // 访问值
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(s) => println!("Blue 分数: {}", s),
        None => println!("Blue 不存在"),
    }
    
    // 遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // 更新 HashMap
    scores.insert(String::from("Blue"), 25);  // 覆盖
    println!("覆盖后: {:?}", scores);
    
    // 只在键不存在时插入
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);
    println!("entry 后: {:?}", scores);
    
    // 基于旧值更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("单词计数: {:?}", map);

    // -------------------------------------------------------------------------
    // 4. HashSet
    // -------------------------------------------------------------------------
    println!("\n【4. HashSet - 哈希集合】\n");
    
    use std::collections::HashSet;
    
    // 创建 HashSet
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(2);  // 重复值会被忽略
    
    println!("HashSet: {:?}", set);
    
    // 检查包含
    println!("包含 2: {}", set.contains(&2));
    
    // 移除
    set.remove(&2);
    println!("移除后: {:?}", set);
    
    // 集合运算
    let a: HashSet<_> = [1, 2, 3, 4].iter().collect();
    let b: HashSet<_> = [3, 4, 5, 6].iter().collect();
    
    let union: HashSet<_> = a.union(&b).collect();  // 并集
    let intersection: HashSet<_> = a.intersection(&b).collect();  // 交集
    let difference: HashSet<_> = a.difference(&b).collect();  // 差集
    let symmetric_difference: HashSet<_> = a.symmetric_difference(&b).collect();  // 对称差集
    
    println!("并集: {:?}", union);
    println!("交集: {:?}", intersection);
    println!("差集 (a - b): {:?}", difference);
    println!("对称差集: {:?}", symmetric_difference);

    // -------------------------------------------------------------------------
    // 5. VecDeque (双端队列)
    // -------------------------------------------------------------------------
    println!("\n【5. VecDeque - 双端队列】\n");
    
    use std::collections::VecDeque;
    
    let mut deque = VecDeque::new();
    
    // 从前端添加
    deque.push_front(1);
    deque.push_front(2);
    
    // 从后端添加
    deque.push_back(3);
    deque.push_back(4);
    
    println!("VecDeque: {:?}", deque);
    
    // 从两端移除
    println!("前端弹出: {:?}", deque.pop_front());
    println!("后端弹出: {:?}", deque.pop_back());
    println!("剩余: {:?}", deque);

    // -------------------------------------------------------------------------
    // 6. LinkedList (链表)
    // -------------------------------------------------------------------------
    println!("\n【6. LinkedList - 链表】\n");
    
    use std::collections::LinkedList;
    
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(0);
    
    println!("LinkedList: {:?}", list);
    
    // 遍历
    for elem in &list {
        print!("{} ", elem);
    }
    println!();

    // -------------------------------------------------------------------------
    // 7. BTreeMap 和 BTreeSet
    // -------------------------------------------------------------------------
    println!("\n【7. BTreeMap 和 BTreeSet - 有序集合】\n");
    
    use std::collections::{BTreeMap, BTreeSet};
    
    // BTreeMap - 自动按键排序
    let mut btree_map = BTreeMap::new();
    btree_map.insert(3, "c");
    btree_map.insert(1, "a");
    btree_map.insert(2, "b");
    
    print!("BTreeMap (有序): ");
    for (k, v) in &btree_map {
        print!("{}:{} ", k, v);
    }
    println!();
    
    // BTreeSet - 自动排序
    let mut btree_set = BTreeSet::new();
    btree_set.insert(3);
    btree_set.insert(1);
    btree_set.insert(2);
    
    println!("BTreeSet (有序): {:?}", btree_set);

    // -------------------------------------------------------------------------
    // 8. BinaryHeap (二叉堆)
    // -------------------------------------------------------------------------
    println!("\n【8. BinaryHeap - 二叉堆（优先队列）】\n");
    
    use std::collections::BinaryHeap;
    
    let mut heap = BinaryHeap::new();
    
    heap.push(1);
    heap.push(5);
    heap.push(2);
    
    println!("堆顶: {:?}", heap.peek());
    
    // 弹出最大值（大顶堆）
    while let Some(n) = heap.pop() {
        print!("{} ", n);
    }
    println!();
    
    // 小顶堆
    use std::cmp::Reverse;
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(5));
    min_heap.push(Reverse(2));
    
    print!("小顶堆: ");
    while let Some(Reverse(n)) = min_heap.pop() {
        print!("{} ", n);
    }
    println!();

    // -------------------------------------------------------------------------
    // 9. 迭代器方法
    // -------------------------------------------------------------------------
    println!("\n【9. 迭代器常用方法】\n");
    
    let v = vec![1, 2, 3, 4, 5];
    
    // map - 转换
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("map: {:?}", doubled);
    
    // filter - 过滤
    let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("filter: {:?}", evens);
    
    // fold - 折叠/累积
    let sum = v.iter().fold(0, |acc, x| acc + x);
    println!("fold (求和): {}", sum);
    
    // find - 查找
    let first_even = v.iter().find(|x| *x % 2 == 0);
    println!("find: {:?}", first_even);
    
    // take - 取前 n 个
    let first_three: Vec<&i32> = v.iter().take(3).collect();
    println!("take(3): {:?}", first_three);
    
    // skip - 跳过前 n 个
    let skip_two: Vec<&i32> = v.iter().skip(2).collect();
    println!("skip(2): {:?}", skip_two);
    
    // chain - 连接迭代器
    let v1 = vec![1, 2];
    let v2 = vec![3, 4];
    let chained: Vec<&i32> = v1.iter().chain(v2.iter()).collect();
    println!("chain: {:?}", chained);
    
    // zip - 配对
    let names = vec!["Alice", "Bob", "Carol"];
    let ages = vec![25, 30, 35];
    let pairs: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("zip: {:?}", pairs);
    
    // enumerate - 带索引
    for (i, v) in v.iter().enumerate() {
        println!("索引 {}: 值 {}", i, v);
    }
}
