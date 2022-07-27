// 《错误处理》
// Rust 没有沿用 C++/Java 等诸多前辈使用的异常处理方式，而是借鉴 Haskell，把错误封装在  Result<T, E> 类型中，同时提供了 ? 操作符来传播错误，方便开发。Result<T, E> 类型是一个泛型数据结构，T 代表成功执行返回的结果类型，E 代表错误类型。
// reqwest 项目中其实里面很多调用已经使用了  Result<T, E> 类型，这里我再展示一下代码，不过我们使用了  unwrap() 方法，只关心成功返回的结果，如果出错，整个程序会终止。

use std::fs;
fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);
}

// 如果想让错误传播，可以把所有的  unwrap() 换成 ? 操作符，并让 main() 函数返回一个 Result<T, E>，如下所示：
// main 函数现在返回一个 Result
fn main_() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);
    Ok(())
}
