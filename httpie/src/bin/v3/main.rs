use anyhow::{anyhow, Result};
use clap::{AppSettings, Parser};
use colored::Colorize;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};

// https://docs.rs/clap/latest/clap/
// 定义 httpie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助

/// A naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "fufeng <luochunyun1995@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    #[clap(subcommand)]
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
#[derive(Debug, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

/// 当实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // 使用 = 进行  ，这会得到一个迭代器
        let mut split = s.split("=");
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

// 注意看把 main 函数变成了 async fn，它代表异步函数。
// 对于 async main，需要使用 #[tokio::main] 宏来自动添加处理异步的运行时。
// 然后在 main 函数内部，根据子命令的类型，分别调用 get 和 post 函数做具体处理，这两个函数实现如下：

/// get sub command
async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

/// post sub command
async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

// 其中，解析出来的 KvPair 列表，需要装入一个 HashMap，然后传给 HTTP client 的 JSON 方法。这样的 HTTPie 的基本功能就完成了。
// 不过现在打印出来的数据对用户非常不友好，需要进一步用不同的颜色打印 HTTP header 和 HTTP body，就像 Python 版本的 HTTPie 那样。

// 打印服务器版本号 + 状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

// 打印服务器返回的HTTP header
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    print!("\n");
}

// 打印服务器返回的http body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        // 对于 applicaiton/json , 使用 pretty print
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        // 其他类型的 mime type, 直接输出
        _ => println!("{}", body),
    }
}

// 打印整个响应
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_context_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

// 将服务器返回的 content-type 解析成 Mime 类型
fn get_context_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

// HTTP Do , 程序入口函数，因为在 HTTP 请求时使用了异步处理，所以这里引入tokio
#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let mut headers = header::HeaderMap::new();
    // 为 HTTP 客户端添加一些缺省的 HTTP 头
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);

    // 生成一个HTTP客户端
    let client = Client::builder().default_headers(headers).build()?;
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
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

// 仅仅在 cargo test 时运行
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_err());
        assert!(parse_url("http://httpbin.org/post").is_err())
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());

        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                k: "a".into(),
                v: "1".into()
            }
        );

        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair {
                k: "b".into(),
                v: "".into()
            }
        );
    }
}

// 可以用 cargo test 运行。Rust 支持条件编译，这里 #[cfg(test)] 表明整个 mod tests 都只在 cargo test 时才编译。

// 使用代码行数统计工具 tokei 可以看到，总共使用了 138 行代码，就实现了这个功能，其中还包含了约 30 行的单元测试代码：

// https://github.com/chinanf-boy/tokei-zh#%E6%94%AF%E6%8C%81%E7%9A%84%E8%AF%AD%E8%A8%80

// $ cargo install tokei

// $ tokei main.rs

/*
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 Rust                    1          230          138           54           38
 |- Markdown             1           12            0           12            0
 (Total)                            242          138           66           38
===============================================================================
 Total                   1          230          138           54           38
===============================================================================
 */

// 可以使用 cargo build —release，编译出 release 版本，并将其拷贝到某个在 $PATH下的目录，然后体验一下。

/*
只用了 100 行代码出头，就实现了 HTTPie 的核心功能，远低于预期的 200 行。不知道你能否从中隐约感受到 Rust 解决实际问题的能力，以今天实现的 HTTPie 为例，

1、要把命令行解析成数据结构，我们只需要在数据结构上，添加一些简单的标注就能搞定。
2、数据的验证，又可以由单独的、和主流程没有任何耦合关系的函数完成。
3、作为 CLI 解析库，clap 的整体体验和 Python 的 click 非常类似，但比 Golang 的 cobra 要更简单。

这就是 Rust 语言的能力体现，明明是面向系统级开发，却能够做出类似 Python 的抽象和体验，所以一旦你适应了 Rust ，用起来就会感觉非常美妙。

syntect 是 Rust 的一个语法高亮库，非常强大。
*/
