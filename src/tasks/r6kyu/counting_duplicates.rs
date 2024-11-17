use std::collections::HashMap;

fn get_asymptotic_model(text: &str) -> HashMap<char, i32> {
    let mut count_of_letters = HashMap::new();
    text.to_ascii_lowercase().chars().for_each(|c| {
        *count_of_letters.entry(c).or_insert(0) += 1;
    });
    count_of_letters
}


pub fn count_duplicates(text: &str) -> u32 {
    get_asymptotic_model(text).into_values().filter(
        |v| {
            v > &1
        }
    ).count() as u32
}