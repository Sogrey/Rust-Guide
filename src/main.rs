// ============================================================================
// Rust 学习示例主程序
// ============================================================================
// 
// 运行方式：
// cargo run --bin rust_learning
//
// 单独运行某个示例：
// 1. 修改下方代码，注释掉其他模块
// 2. 或者在对应文件中添加 #[allow(dead_code)]
// ============================================================================

mod _01_variables;
mod _02_data_types;
mod _03_functions;
mod _04_control_flow;
mod _05_ownership;
mod _06_structs;
mod _07_enums;
mod _08_collections;
mod _09_modules;
mod _10_error_handling;
mod _11_generics;
mod _12_traits;
mod _13_lifetimes;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║           Rust 完整学习示例 - 从入门到精通                   ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    // 依次运行所有示例
    println!("▶ 运行示例 01 - 变量");
    println!("{}", "─".repeat(60));
    _01_variables::run();
    println!("\n");

    println!("▶ 运行示例 02 - 数据类型");
    println!("{}", "─".repeat(60));
    _02_data_types::run();
    println!("\n");

    println!("▶ 运行示例 03 - 函数");
    println!("{}", "─".repeat(60));
    _03_functions::run();
    println!("\n");

    println!("▶ 运行示例 04 - 流程控制");
    println!("{}", "─".repeat(60));
    _04_control_flow::run();
    println!("\n");

    println!("▶ 运行示例 05 - 所有权");
    println!("{}", "─".repeat(60));
    _05_ownership::run();
    println!("\n");

    println!("▶ 运行示例 06 - 结构体");
    println!("{}", "─".repeat(60));
    _06_structs::run();
    println!("\n");

    println!("▶ 运行示例 07 - 枚举和模式匹配");
    println!("{}", "─".repeat(60));
    _07_enums::run();
    println!("\n");

    println!("▶ 运行示例 08 - 集合");
    println!("{}", "─".repeat(60));
    _08_collections::run();
    println!("\n");

    println!("▶ 运行示例 09 - 包和模块");
    println!("{}", "─".repeat(60));
    _09_modules::run();
    println!("\n");

    println!("▶ 运行示例 10 - 错误处理");
    println!("{}", "─".repeat(60));
    _10_error_handling::run();
    println!("\n");

    println!("▶ 运行示例 11 - 泛型");
    println!("{}", "─".repeat(60));
    _11_generics::run();
    println!("\n");

    println!("▶ 运行示例 12 - Trait");
    println!("{}", "─".repeat(60));
    _12_traits::run();
    println!("\n");

    println!("▶ 运行示例 13 - 生命周期");
    println!("{}", "─".repeat(60));
    _13_lifetimes::run();
    println!("\n");

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                 所有示例运行完成！                          ║");
    println!("╚════════════════════════════════════════════════════════════╝");
}
