
#[cfg(test)]
mod tests {
    use std::borrow::{Borrow, BorrowMut};
    use std::cell::RefCell;
    use std::sync::{Arc, Mutex, RwLock};
    use std::thread::{JoinHandle, sleep};
    use std::time::Duration;

    #[test]
    pub fn test_thread() {
        let arr = vec![1];

        std::thread::spawn(move || {
            println!("arr: {:?}", arr);
        });
    }

    #[test]
    pub fn test_thread_() {
        let arr = Arc::new(RwLock::new(vec![1]));

        // to force the closure to take ownership of `arr` (and any other referenced variables), use the `move` keyword
        /*std::thread::spawn(|| {
            println!("arr: {:?}", arr);
        });*/
    }

    #[test]
    pub fn test_thread_shared_string() {
        let shared_str = Arc::new(Mutex::new(Vec::<String>::new()));

        let shared_str_buf = shared_str.clone();

        std::thread::spawn(move || {
            println!("shared_str: {:?}", shared_str.lock().unwrap().push(String::from("foo")));
        });
        //std::time::sleep(Duration::from_secs(5));
        sleep(Duration::from_secs(1));
        println!("shared_str: {:?}", shared_str_buf.lock().unwrap().pop().unwrap());
    }

}
