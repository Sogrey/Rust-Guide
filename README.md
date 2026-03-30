# Rust 学习指南

Rust 语言完整学习示例，从入门到精通。

## 项目简介

本项目包含 13 个详细的 Rust 学习示例文件，涵盖了 Rust 语言的核心概念和特性。每个文件都包含丰富的代码示例和详细的中文注释，适合初学者系统学习 Rust。

## 目录结构

```
rust-guide/
├── Cargo.toml                 # Cargo 配置文件
├── README.md                  # 项目说明文档
├── LICENSE                    # 许可证
└── src/
    ├── main.rs                # 主程序入口
    ├── _01_variables.rs       # 变量详解
    ├── _02_data_types.rs      # 数据类型详解
    ├── _03_functions.rs       # 函数详解
    ├── _04_control_flow.rs    # 流程控制详解
    ├── _05_ownership.rs       # 所有权详解
    ├── _06_structs.rs         # 结构体详解
    ├── _07_enums.rs           # 枚举和模式匹配详解
    ├── _08_collections.rs     # 集合详解
    ├── _09_modules.rs         # 包和模块详解
    ├── _10_error_handling.rs  # 错误处理详解
    ├── _11_generics.rs        # 泛型详解
    ├── _12_traits.rs          # Trait 详解
    └── _13_lifetimes.rs       # 生命周期详解
```

## 学习内容

### 01. 变量 (Variables)

- 变量声明与可变性 (`let`, `mut`)
- 变量遮蔽 (Variable Shadowing)
- 常量与不可变变量的区别
- 类型标注与类型推断
- 变量作用域
- 解构赋值

### 02. 数据类型 (Data Types)

- **标量类型**：整数、浮点数、布尔值、字符
- **复合类型**：元组、数组
- 整数字面量表示法（十进制、十六进制、八进制、二进制）
- 类型别名与类型转换
- 单元类型与空类型

### 03. 函数 (Functions)

- 函数定义与参数
- 返回值与表达式
- 语句 vs 表达式
- 函数指针
- 闭包 (Closure)
- 递归函数
- 高阶函数
- 方法和关联函数
- 发散函数

### 04. 流程控制 (Control Flow)

- `if` 表达式
- `loop` 循环（包括循环标签）
- `while` 循环
- `for` 循环与迭代器
- `match` 表达式
- `if let` 和 `while let`
- 模式匹配进阶（解构、守卫、@ 绑定）

### 05. 所有权 (Ownership)

- 所有权三大规则
- 变量作用域
- String 类型与堆分配
- 移动语义 (Move Semantics)
- 克隆 (Clone)
- Copy Trait
- 引用与借用
- 可变引用
- 悬垂引用与 Rust 的防护
- 切片 (Slice)

### 06. 结构体 (Structs)

- 结构体定义与实例化
- 可变结构体
- 结构体更新语法
- 元组结构体
- 单元结构体
- 方法定义 (`impl` 块)
- 关联函数
- 结构体打印与调试 (`#[derive(Debug)]`)
- 常用派生 Trait

### 07. 枚举和模式匹配 (Enums & Pattern Matching)

- 枚举定义
- 枚举值关联数据
- `Option<T>` 枚举
- `Result<T, E>` 枚举
- `match` 表达式详解
- `if let` 简化匹配
- `while let` 循环
- 匹配绑定与守卫
- `matches!` 宏

### 08. 集合 (Collections)

- **Vector**：动态数组操作
- **String**：字符串处理
- **HashMap**：键值对存储
- **HashSet**：唯一值集合
- **VecDeque**：双端队列
- **LinkedList**：链表
- **BTreeMap/BTreeSet**：有序集合
- **BinaryHeap**：二叉堆
- 迭代器方法（`map`, `filter`, `fold`, `find` 等）

### 09. 包和模块 (Packages & Modules)

- 模块系统概述
- 模块定义 (`mod`)
- 路径与 `use` 关键字
- `pub` 可见性控制
- `super` 和 `self` 关键字
- 结构体和字段可见性
- 模块文件分离
- 外部包使用

### 10. 错误处理 (Error Handling)

- 不可恢复错误：`panic!`
- 可恢复错误：`Result<T, E>`
- `unwrap` 和 `expect`
- 错误传播：`?` 操作符
- 自定义错误类型
- `Option` 和 `Result` 转换
- 错误处理最佳实践

### 11. 泛型 (Generics)

- 泛型函数
- 泛型结构体
- 泛型枚举
- 泛型方法
- Trait Bound (特征约束)
- `where` 子句
- 关联类型
- 默认泛型类型参数
- Const 泛型
- 泛型性能：单态化

### 12. Trait

- Trait 定义与实现
- 默认实现
- Trait 作为参数
- Trait Bound
- 返回 Trait
- 完全限定语法
- Supertrait (父 Trait)
- Newtype 模式
- 动态分发 (`dyn Trait`)
- 常用标准库 Trait
- Trait 继承

### 13. 生命周期 (Lifetimes)

- 生命周期概念
- 生命周期注解语法
- 函数中的生命周期
- 结构体中的生命周期
- 生命周期省略规则
- `'static` 生命周期
- 多个生命周期参数
- 生命周期与泛型
- 生命周期子类型化
- 协变和逆变
- 高阶生命周期 (HRTB)
- 常见生命周期模式

## 快速开始

### 环境要求

- Rust 1.70 或更高版本
- Cargo（Rust 包管理器）

### 运行项目

```bash
# 克隆仓库
git clone git@github.com:Sogrey/Rust-Guide.git
cd Rust-Guide

# 运行所有示例
cargo run

# 编译检查
cargo check

# 构建项目
cargo build
```

### 运行特定示例

如果你想单独学习某个主题，可以在 `main.rs` 中注释掉其他模块，只保留你想学习的部分。

## 学习建议

1. **按顺序学习**：每个主题都是后续内容的基础，建议按顺序学习
2. **动手实践**：每个示例都可以直接运行，建议修改代码并观察结果
3. **阅读注释**：代码中包含详细的中文注释，帮助理解概念
4. **查阅文档**：遇到不理解的概念，可以查阅 [Rust 官方文档](https://doc.rust-lang.org/)

## 参考资源

- [Rust 官方网站](https://www.rust-lang.org/)
- [Rust 程序设计语言（The Book）](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust 标准库文档](https://doc.rust-lang.org/std/)

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 贡献

欢迎提交 Issue 和 Pull Request！

## 作者

Sogrey

## 致谢

感谢 Rust 社区的所有贡献者，让 Rust 成为如此优秀的编程语言！
