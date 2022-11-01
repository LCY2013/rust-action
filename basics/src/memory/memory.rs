use std::collections::HashMap;
use std::mem::{align_of, size_of};

struct S1 {
    a: u8,
    b: u16,
    c: u8,
}

struct S2 {
    a: u8,
    c: u8,
    b: u16,
}

#[repr()] // 强制 Rust 编译器不做关于内存对齐相关优化
struct S3 {
    a: u8,
    b: u16,
    c: u8,
}

enum E {
    A(f64),
    B(HashMap<String, String>),
    C(Result<Vec<u8>, String>),
}

/// 这里一个声明宏，它会打印各种数据结构本身的大小，在 Option 中的大小，以及在 Result 中的大小
macro_rules! show_size {
    (header) => {
        println!(
            "{:<24} {:>4}   {}  {}",
            "Type", "T", "Option<T>", "Result<T, io::Error>"
        );
        println!("{}", "-".repeat(64));
    };
    ($t:ty) => {
        println!(
            "{:<24} {:4}   {:8}  {:12}",
            stringify!($t),
            size_of::<$t>(),
            size_of::<Option<$t>>(),
            size_of::<Result<$t, std::io::Error>>(),
        )
    }
}

#[derive(Default)]
struct Align1 {
    a: u8,
    b: usize,
    c: u32,
}

#[derive(Default)]
struct Align2 {
    a: u8,
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use super::*;

    #[test]
    fn test_mem_padding() {
        println!("sizeof S1: {}, S2: {}", size_of::<S1>(), size_of::<S2>());
        println!("alignof S1: {}, S2: {}", align_of::<S1>(), align_of::<S2>());
        println!("sizeof S1: {}, S2: {}, S3: {}", size_of::<S1>(), size_of::<S2>(), size_of::<S3>());
    }

    #[test]
    fn test_show_size() {
        show_size!(header);
        show_size!(u8);
        show_size!(f64);
        show_size!(&u8);
        show_size!(Box<u8>);
        show_size!(&[u8]);
        show_size!(String);
        show_size!(Vec<u8>);
        show_size!(HashMap<String, String>);
        show_size!(E);
        show_size!(Result<String,()>);
    }

    #[test]
    fn test_io() -> std::io::Result<()> {
        let mut file = File::create("foo.txt")?;
        file.write_all(b"hello world")?;
        Ok(())
    }

    #[test]
    fn test_mem() {
        let s1 = "a";
        let s2 = "aaaa";
        let s3 = "hello";
        let a = Align1::default();
        let b = Align2::default();

        println!("{:p}", s1);
        println!("{:p}", s2);
        println!("{:p}", s3);

        println!("Align1.a: {:p}", &a.a);
        println!("Align1.b: {:p}", &a.b);
        println!("Align1.c: {:p}", &a.c);
        println!("Align2.a: {:p}", &b.a);
    }

}

