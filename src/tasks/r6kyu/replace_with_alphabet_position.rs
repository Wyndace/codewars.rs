fn to_alphabet_index(c: char) -> Option<usize> {
    match c {
        'A'..='Z' => Some(c as usize - 'A' as usize + 1),
        'a'..='z' => Some(c as usize - 'a' as usize + 1),
        _=> None
    }
}

pub fn alphabet_position(text: &str) -> String {
    let mut result = String::new();
    for c in text.chars() {
        let index = to_alphabet_index(c).unwrap_or(0);
        if index == 0 {
            continue
        }
        result.push_str(format!("{} ", index).as_str());
    }
    result.trim().to_owned()
}