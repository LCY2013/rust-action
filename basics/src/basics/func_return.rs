/*
Rust 函数参数的类型和返回值的类型都必须显式定义，如果没有返回值可以省略，返回 unit。
函数内部如果提前返回，需要用 return 关键字，否则最后一个表达式就是其返回值。
如果最后一个表达式后添加了; 分号，隐含其返回值为 unit。
*/
fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    3.2425926;
}

fn main() {
    let is_pi = pi();
    let is_uint1 = not_pi();
    let is_uint2 = {
        pi();
    };

    println!(
        "is_pi: {:?}, is_uint1: {:?}, is_uint2: {:?}",
        is_pi, is_uint1, is_uint2
    );
}
