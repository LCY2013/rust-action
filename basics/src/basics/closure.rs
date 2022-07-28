// 闭包引用的上下文中的自由变量，会被捕获到闭包的结构中，成为闭包类型的一部分。
// 一般来说，如果一门编程语言，其函数是一等公民，那么它必然会支持闭包（closure），因为函数作为返回值往往需要返回一个闭包。
fn closure() {
    let a = "hello";
    let b = "Tyr";

    let c = |msg: &str| {
        println!("{}{}:{}",a,b,msg);
    };

    c("How are you?")
}
// 接口与虚表