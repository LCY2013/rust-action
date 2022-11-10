#[cfg(test)]
mod tests {
    use std::rc::Rc;
    //use supper::*;

    #[test]
    fn rc() {
        let a = Rc::new(1);
        println!("a: {}", a);
    }

    #[test]
    fn rc_clone() {
        let a = Rc::new(1);
        let b = a.clone();
        let c = a.clone();
        println!("a: {},b: {}, c: {}", a, b, c);
    }
}
