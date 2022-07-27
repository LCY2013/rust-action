// 在 Rust 下，函数是一等公民，可以作为参数或者返回值。我们来看一个函数作为参数的例子
fn apply(value: i32,f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

/*
这里  fn(i32) -> i32 是 apply 函数第二个参数的类型，它表明接受一个函数作为参数，这个传入的函数必须是：参数只有一个，且类型为 i32，返回值类型也是 i32。
Rust 函数参数的类型和返回值的类型都必须显式定义，如果没有返回值可以省略，返回 unit。
函数内部如果提前返回，需要用 return 关键字，否则最后一个表达式就是其返回值。
如果最后一个表达式后添加了; 分号，隐含其返回值为 unit。
 */
fn main() {
    println!("apply square: {}", apply(2,square));
    println!("apply cube: {}", apply(2,cube));
}
