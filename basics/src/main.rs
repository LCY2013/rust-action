pub mod basics;
pub mod engineering;
pub mod lifecycle;
pub mod memory;
pub mod ownership;
pub mod types;

use crate::variables::let_tro;
use basics::io::*;
use basics::*;

// cargo run -- https://www.rust-lang.org rust.md, 命令行信息
fn main() {
    println!("Hello, world!");
    let name = "fufeng".to_string();
    std::thread::spawn(|| {
        //println!("hello {}", name)
    });
    // 获取系统环境参数信息
    for arg in std::env::args() {
        println!("{}", arg);
    }

    // 猜数字
    //guess();

    // 随机数
    //rand();

    // 猜数字比较
    //guess_tips();

    // 猜数字，类型转换
    //guess_convert();

    // 猜数字，循环结构
    //guess_loop();

    // 猜数字，健壮性
    //guess_robustness();

    // 变量篇
    let_tro();
}
