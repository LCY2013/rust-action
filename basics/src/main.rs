fn main() {
    println!("Hello, world!");
    let name = "fufeng".to_string();
    std::thread::spawn(|| {
        //println!("hello {}", name)
    });
}
