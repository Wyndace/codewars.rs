pub fn solution(s: &str) -> Vec<String> {
    let mut result = vec![];
    let mut s = s.to_owned();
    let mut result_str = String::new();
    if s.len() % 2 != 0 {
        s.push('_');
    }
    for c in s.chars() {
        result_str.push(c);
        if result_str.len() == 2 {
            result.push(result_str.clone());
            result_str = String::new()
        }
    }
    result
}