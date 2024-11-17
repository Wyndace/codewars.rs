mod tasks;

fn main() {
    println!("{}", tasks::counting_duplicates::count_duplicates("abcd"));
    println!("{}", tasks::counting_duplicates::count_duplicates("abcdea"));
    println!("{}", tasks::counting_duplicates::count_duplicates("indivisibility"));
    println!("{}", tasks::counting_duplicates::count_duplicates("Hello, World!"));
    println!("{}", tasks::counting_duplicates::count_duplicates("ABBA"));
    println!("{}", tasks::counting_duplicates::count_duplicates("Indivisibilities"));
    println!("{}", tasks::counting_duplicates::count_duplicates("aA11"));
}
