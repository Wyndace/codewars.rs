pub fn remove_exclamation_marks(input: &str) -> String {
    input.replace(|c: char| c == '!', "")
}