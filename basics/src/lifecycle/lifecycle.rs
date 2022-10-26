/// compare string
/// this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `s1` or `s2`
/*fn max(s1: &str, s2: &str) -> &str {
    if s1 > s2 { s1 } else { s2 }
}*/

/// compare string
fn max_correct<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 { s1 } else { s2 }
}

/// get max
fn get_max(s1: &str) -> &str {
    //max(s1, "Cynthia")
    max_correct(s1, "Cynthia")
}

/// first word
fn first(s: &str) -> &str {
    let trimmed = s.trim();
    match trimmed.find(' ') {
        None => "",
        Some(pos) => &trimmed[..pos],
    }
}

/// first word : 等价于 fn first(s: &str) -> &str{}
fn first_correct<'a>(s: &'a str) -> &'a str {
    let trimmed = s.trim();
    match trimmed.find(' ') {
        None => "",
        Some(pos) => &trimmed[..pos],
    }
}

/// 生命周期标注
pub fn strtok<'a, 'b>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        // 由于 delimiter 可以是 utf8，所以我们需要获取其 utf8 长度，
        // 直接使用 len 返回的字节长度，会有问题
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        // 如果没有找到，就返回整个字符串，把原字符串指针 s 指向空串
        let prefix = *s;
        *s = prefix;
        prefix
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_str() {
        let s1 = String::from("Lindsey");
        let s2 = String::from("Rosie");
        //let result = max(&s1, &s2);
        let result = max_correct(&s1, &s2);
        println!("bigger one: {}", result);
    }

    #[test]
    fn test_mul_max() {
        let s1 = String::from("Lindsey");
        let s2 = String::from("Rosie");
        //let result = max(&s1, &s2);
        let result = max_correct(&s1, &s2);
        println!("bigger one: {}", result);
        let result = get_max(&s1);
        println!("bigger one: {}", result);
    }

    #[test]
    fn test_first() {
        let s1 = "hello world";
        println!("first word of s1: {}", first(&s1));
    }

    #[test]
    fn test_strtok() {
        let s = "hello world".to_owned();
        let mut s1 = s.as_str();
        let hello = strtok(&mut s1, ' ');
        println!("hello is: {}, s1: {}, s: {}", hello, s1, s);
    }
}