use std::io;
use io::stdin;
use std::cmp::Ordering;
use rand::Rng;

pub fn guess_convert() {
    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("Guess the number！");
    let mut guess = String::new();
    stdin().read_line(&mut guess)
        .expect("Error reading line");

    // 类型转换
    let guess: u32 = guess.trim().parse()
        .expect("Please enter a number");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You wind!"),
    }
}

pub fn guess_tips() {
    let secret_number = rand::thread_rng().gen_range(1..101).to_string();

    println!("Guess the number!");
    println!("Please input the number");
    let mut guess = String::new();

    stdin().read_line(&mut guess)
        .expect("Error reading");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You wind!"),
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
    }
}

pub fn guess() {
    println!("Guess the number!");
    println!("Please input the number.");
    let mut guess = String::new();

    stdin().read_line(&mut guess)
        .expect("Error reading");

    println!("You guess: {}", guess)
}

/// 使用 crate 来增加更多功能 记住，crate 是一个 Rust 代码包。
/// 正在构建的项目是一个 二进制 crate，它生成一个可 执行文件。 rand crate 是一个 库 crate，库 crate 可以包含任意能被其他程序使用的代码。
/// Cargo 对外部 crate 的运用是其真正闪光的地方。
/// 在我们使用 rand 编写代码之前，需要修 改 Cargo.toml 文件，引入一个 rand 依赖。现在打开这个文件并在底部的 [dependencies] 片段标题之下添加：
/// rand = "0.8.5"
pub fn rand() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    stdin().read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {}", guess)
}

/// cargo test --color=always --lib basics::io::tests::it_works
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guess_test() {
        guess();
    }

    #[test]
    fn rand_test() {
        rand();
    }
}
