pub fn solution(s: &str) -> Vec<String> {
    let mut s = s.to_owned();
    if s.len() % 2 != 0 {
        s.push('_');
    }
    s
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
}