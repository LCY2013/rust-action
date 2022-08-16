use std::io;
use io::stdin;
use std::cmp::Ordering;
use rand::Rng;

pub fn guess_convert() {
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("This secret number is: {:?}", secret_number);

    println!("Guess the number：");
    let mut guess = String::new();
    stdin().read_line(&mut guess)
        .expect("Error reading line");

    // 类型转换
    // guess.trim().parse() 表达式上。表达式中的 guess 是包含输入的原 始 String 类型。
    // String 实例的 trim 方法会去除字符串开头和结尾的空白字符。
    // u32 只 能由数字字符转换，不过用户必须输入 enter 键才能让 read_line 返回，然而用户按下 enter 键时，会在字符串中增加一个换行（newline）符。
    // 例如，用户输入 5 并按下 enter， guess 看起来像这样： 5\n 。
    // \n 代表 “换行”，回车键。 trim 方法消除 \n ，只留下 5 。
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
