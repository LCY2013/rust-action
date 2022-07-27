// 《RUST项目组织》
// 当 Rust 代码规模越来越大时，我们就无法用单一文件承载代码了，需要多个文件甚至多个目录协同工作，这时我们可以用 mod 来组织代码。
// 具体做法是：在项目的入口文件 lib.rs / main.rs 里，用 mod 来声明要加载的其它代码文件。如果模块内容比较多，可以放在一个目录下，在该目录下放一个 mod.rs 引入该模块的其它文件。这个文件，和 Python 的 __init__.py 有异曲同工之妙。这样处理之后，就可以用 mod + 目录名引入这个模块了，如下图所示：

// 在 Rust 里，一个项目也被称为一个 crate。crate 可以是可执行项目，也可以是一个库，我们可以用  cargo new <name> -- lib 来创建一个库。当 crate 里的代码改变时，这个 crate 需要被重新编译。
// 在一个 crate 下，除了项目的源代码，单元测试和集成测试的代码也会放在 crate 里。
// Rust 的单元测试一般放在和被测代码相同的文件中，使用条件编译  #[cfg(test)] 来确保测试代码只在测试环境下编译。以下是一个单元测试的例子：
/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/

#[cfg(test)]
mod tests {
   #[test]
   fn it_works() {
       assert_eq!(2+ 2, 4)
   }
}

// 集成测试一般放在 tests 目录下，和 src 平行。和单元测试不同，集成测试只能测试 crate 下的公开接口，编译时编译成单独的可执行文件。
// 在 crate 下，如果要运行测试用例，可以使用  cargo test。
// 当代码规模继续增长，把所有代码放在一个 crate 里就不是一个好主意了，因为任何代码的修改都会导致这个 crate 重新编译，这样效率不高。我们可以使用 workspace。
// 一个 workspace 可以包含一到多个 crates，当代码发生改变时，只有涉及的 crates 才需要重新编译。当我们要构建一个 workspace  时，需要先在某个目录下生成一个如图所示的 Cargo.toml，包含 workspace 里所有的 crates，然后可以  cargo new 生成对应的 crates：
