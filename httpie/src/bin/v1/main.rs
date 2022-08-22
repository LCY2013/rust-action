use clap::{AppSettings, Parser};

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

// get 子命令

/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP method to request URL
    url: String,
}

// post 子命令。需要输入一个 URL，和若干个可选的 key=value，用于提供 json body

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Parser, Debug)]
struct Post {
    //// HTTP method to request URL
    url: String,
    /// HTTP body to post request
    body: Vec<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts)
}

// 代码中用到了 clap 提供的宏来让 CLI 的定义变得简单，这个宏能够生成一些额外的代码帮处理 CLI 的解析。
// 通过 clap ，只需要先用一个数据结构 T 描述 CLI 都会捕获什么数据，之后通过 T::parse() 就可以解析出各种命令行参数了。parse() 函数我们并没有定义，它是 #[derive(Clap)] 自动生成的。
// 目前定义了两个子命令，在 Rust 中子命令可以通过 enum 定义，每个子命令的参数又由各自的数据结构 Get 和 Post 来定义。

// run:
// cargo build --quiet && ../target/debug/httpie post httpbin.org/post a=1 b=2
// Opts { subcmd: Post(Post { url: "httpbin.org/post", body: ["a=1", "b=2"] }) }
// ../target/debug/httpie --help

// 默认情况下，cargo build 编译出来的二进制，在项目根目录的 target/debug 下。可以看到，命令行解析成功。

// 《加入校验信息》
// 一是验证 URL，另一个是验证 body。
