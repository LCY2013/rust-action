// 变量与不可变量
pub fn let_tro() {
    //let x = 5;
    let mut x = 5;
    println!("This value x is: {}", x);
    x = 6;
    println!("This value x is: {}", x);
}

// 隐藏（Shadowing） 变量隐藏
pub fn let_var_tro() {
    // 这里允许第一个 spaces 变量是字符串类型，而第二个 spaces 变量，它是一个恰巧与第一 个变量同名的崭新变量，是数字类型。
    // 隐藏使我们不必使用不同的名字，如 spaces_str 和 spaces_num ；
    // 相反，可以复用 spaces 这个更简单的名字。
    let spaces = " ";
    let spaces = spaces.len();

    // 然而，如果尝试使用 mut ，将会得到一个编译时错误。
    //let mut spaces = " ";
    //spaces = spaces.len();
}
