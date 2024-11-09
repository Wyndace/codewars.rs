mod tasks;

fn main() {
    println!("{:?}", tasks::split_strings::solution("abcdef"));
    println!("{:?}", tasks::split_strings::solution(("abcdefg")));
}
