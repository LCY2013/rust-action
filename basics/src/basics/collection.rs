fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }

    None
}

fn sum(data: Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
       Rust所有权规则：
        1、一个值只能被一个变量所拥有，这个变量被称为所有者（Each value in Rust has a variable that’s called its owner）。
        2、一个值同一时刻只能有一个所有者（There can only be one owner at a time），也就是说不能有两个变量拥有相同的值。所以对应刚才说的变量赋值、参数传递、函数返回等行为，旧的所有者会把值的所有权转移给新的所有者，以便保证单一所有者的约束。
        3、当所有者离开作用域，其拥有的值被丢弃（When the owner goes out of scope, the value will be dropped），内存得到释放。
    */
    #[test]
    fn it_works() {
        let data = vec![1, 2, 3, 4, 5];
        let v = 3;
        if let Some(pos) = find_pos(data, v) {
            println!("Find {} at {}", v, pos);
        }
        //println!("{:?}", data);
    }

    /*
       在这段代码里，先创建了一个不可变数据 data，然后将 data 赋值给 data1。
       按照所有权的规则，赋值之后，data 指向的值被移动给了 data1，它自己便不可访问了。
       而随后，data1 作为参数被传给函数 sum()，在 main() 函数下，data1 也不可访问了。

       但是后续的代码依旧试图访问 data1 和 data，所以，这段代码应该会有两处错误。
       cargo test --color=always --bin basics basics::collection::tests::it_works_sum
    */
    #[test]
    fn it_works_sum() {
        let data = vec![1, 2, 3, 4, 5];
        let data1 = data;
        println!("sum of data1: {}", sum(data1));
        //println!("data1: {:?}", data1);
        //println!("sum of data: {}", sum(data));
    }

    /*
       如果要在把 data1 传给 sum()，同时，还想让 main() 能够访问 data，该怎么办？
       可以调用  data.clone() 把 data 复制一份出来给 data1，这样，在堆上就有  vec![1,2,3,4] 两个互不影响且可以独立释放的副本。
    */
    #[test]
    fn it_works_sum_copy() {
        let data = vec![1, 2, 3, 4];
        let data1 = data.clone();
        println!("sum of data1:{}", sum(data1));
        println!("sum of data:{}", sum(data))
    }

    /*
       所有权规则，解决了谁真正拥有数据的生杀大权问题，让堆上数据的多重引用不复存在，这是它最大的优势。
       但是，这也会让代码变复杂，尤其是一些只存储在栈上的简单数据，如果要避免所有权转移之后不能访问的情况，就需要手动复制，会非常麻烦，效率也不高。


       1、如果你不希望值的所有权被转移，在 Move 语义外，Rust 提供了 Copy 语义。如果一个数据结构实现了 Copy trait，那么它就会使用 Copy 语义。这样，在你赋值或者传参时，值会自动按位拷贝（浅拷贝）。

       2、如果你不希望值的所有权被转移，又无法使用 Copy 语义，那可以“借用”数据。

       符合 Copy 语义的类型，在你赋值或者传参时，值会自动按位拷贝。

       当你要移动一个值，如果值的类型实现了 Copy trait，就会自动使用 Copy 语义进行拷贝，否则使用 Move 语义进行移动。

       根据上图中的错误代码 E0382 使用 rustc --explain E0382  探索更详细的信息。
    */

    /*
       在 Rust 中，什么数据结构实现了 Copy trait 呢？ 你可以通过下面的代码快速验证一个数据结构是否实现了 Copy trait
    */
    fn is_copy<T: Copy>() {}

    fn types_impl_copy_trait() {
        is_copy::<bool>();
        is_copy::<char>();
        // all iXX and uXX, usize/isize, fXX implement Copy trait
        is_copy::<i8>();
        is_copy::<u64>();
        is_copy::<i64>();
        is_copy::<usize>();
        // function (actually a pointer) is Copy
        is_copy::<fn()>();
        // raw pointer is Copy
        is_copy::<*const String>();
        is_copy::<*mut String>();
        // immutable reference is Copy
        is_copy::<&[Vec<u8>]>();
        is_copy::<&String>();
        // array/tuple with values which is Copy is Copy
        is_copy::<[u8; 4]>();
        is_copy::<(&str, &str)>();
    }

    fn types_not_impl_copy_trait() {
        // unsized or dynamic sized type is not Copy
        // is_copy::<str>();
        // is_copy::<[u8]>();
        // is_copy::<Vec<u8>>();
        // is_copy::<String>();
        // mutable reference is not Copy
        //is_copy::<&mut String>();
        // array / tuple with values that not Copy is not Copy
        //is_copy::<[Vec<u8>; 4]>();
        // is_copy::<(String, u32)>();
    }

    /*
       原生类型，包括函数、不可变引用和裸指针实现了 Copy；
       数组和元组，如果其内部的数据结构实现了 Copy，那么它们也实现了 Copy；
       可变引用没有实现 Copy；
       非固定大小的数据结构，没有实现 Copy。

       官方文档介绍 Copy trait 的页面包含了 Rust 标准库中实现 Copy trait 的所有数据结构。
       也可以在访问某个数据结构的时候，查看其文档的 Trait implementation 部分，看看它是否实现了 Copy trait。
    */
    #[test]
    fn it_works_copy_trait() {
        types_impl_copy_trait();
        types_not_impl_copy_trait();
    }

    /*
       所有权：一个值只能被一个变量所拥有，且同一时刻只能有一个所有者，当所有者离开作用域，其拥有的值被丢弃，内存得到释放。
       Move 语义：赋值或者传参会导致值 Move，所有权被转移，一旦所有权转移，之前的变量就不能访问。
       Copy 语义：如果值实现了 Copy trait，那么赋值或传参会使用 Copy 语义，相应的值会被按位拷贝（浅拷贝），产生新的值。
    */
}
