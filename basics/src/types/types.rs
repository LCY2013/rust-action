#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::net::SocketAddr;

    // map自动类型推导
    #[test]
    pub fn test_type_derivation() {
        let mut map = BTreeMap::new();
        // 如果注释掉下面这行：consider giving `map` an explicit type, where the type for type parameter `K` is specified
        map.insert("hello", "world");
        println!("map: {:?}", map);
        println!("map: {:?}", map);
    }

    // 下面无法类型推导
    #[test]
    pub fn test_not_type_derivation() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        /*let even_numbers = numbers
            .into_iter()
            .filter(|n| n % 2 == 0)
            .collect();
        println!("{:?}", even_numbers);*/
    }

    #[test]
    pub fn test_allow_type_derivation() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let even_numbers: Vec<_> = numbers
            .into_iter()
            .filter(|n| n % 2 == 0)
            .collect();
        println!("{:?}", even_numbers);
    }

    #[test]
    pub fn test_allow_type_derivation_var() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let even_numbers = numbers
            .into_iter()
            .filter(|n| n % 2 == 0)
            .collect::<Vec<_>>();
        println!("{:?}", even_numbers);
    }

    #[test]
    pub fn test_generic_turbofish() {
        let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
        println!("addr: {:?}, port: {:?}", addr.ip(), addr.port())
    }
}
