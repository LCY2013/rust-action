use clap::{AppSettings,Parser};
use reqwest::Url;
use std::str::FromStr;
use anyhow::{anyhow,Result};

// https://docs.rs/clap/latest/clap/
// 定义 httpie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助

/// A naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "fufeng <luochunyun1995@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(author, version, about, long_about = None)]
struct  Opts {
    #[clap(subcommand) ]
    subcmd: SubCommand,
}

// 子命令分别对应不同的 HTTP 方法，目前只支持 get / post
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
    // TODO: 支持其他的http method
}

// 《加入校验信息》
// 一是验证 URL，另一个是验证 body。
// 验证url的正确性
fn parse_url(url: &str) -> Result<String> {
    // 这里检查url的合法性
    let _url: Url = url.parse()?;
    Ok(url.into())
}

// get 子命令

/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    // clap 允许为每个解析出来的值添加自定义的解析函数，这里定义了个 parse_url 检查一下。
    /// HTTP method to request URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

// post 子命令。需要输入一个 URL，和若干个可选的 key=value，用于提供 json body。
// 然后，要确保 body 里每一项都是 key=value 的格式。
// 可以定义一个数据结构 KvPair 来存储这个信息，并且也自定义一个解析函数把解析的结果放入 KvPair：

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

/// 当实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // 使用 = 进行  ，这会得到一个迭代器
        let mut  split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None
            // 将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为 value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

/// 因为为 KvPair 实现了 FromStr，这里可以直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

// 这里实现了一个 FromStr trait，可以把满足条件的字符串转换成 KvPair。
// FromStr 是 Rust 标准库定义的 trait，实现它之后，就可以调用字符串的 parse() 泛型函数，很方便地处理字符串到某个类型的转换了。

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Parser, Debug)]
struct Post {
    //// HTTP method to request URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    /// HTTP body to post request
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts)
}

/*
❯ cargo build --quiet
❯ ../../../../target/debug/v2 post https://httpbin.org/post a=1 b
error: Invalid value for '<BODY>...': Failed to parse b
For more information try --help
❯ ../../../../target/debug/v2 post abc a=1
error: Invalid value for '<URL>': relative URL without a base
For more information try --help
target/debug/httpie post https://httpbin.org/post a=1 b=2
Opts { subcmd: Post(Post { url: "https://httpbin.org/post", body: [KvPair { k: "a", v: "1" }, KvPair { k: "b", v: "2" }] }) }
 */

// 没有把各种验证代码一股脑塞在主流程中，而是通过实现额外的验证函数和 trait 来完成的，这些新添加的代码，高度可复用且彼此独立，并不用修改主流程。

// 这非常符合软件开发的开闭原则（Open-Closed Principle）：Rust 可以通过宏、trait、泛型函数、trait object 等工具，帮助更容易写出结构良好、容易维护的代码。
