fn recursive_counter(num: u64, count: &u64) -> u64 {
    if num.to_string().len() == 1 {
        *count
    } else {
        let count = count + 1;
        let mut mult = 1u64;
        for c in num.to_string().chars() {
            let digit = c.to_digit(10).unwrap() as u64;
            mult *= digit;
        }
        recursive_counter(mult, &count)
    }
}

pub fn persistence(num: u64) -> u64 {
    let count = 0;
    recursive_counter(num, &count)
}