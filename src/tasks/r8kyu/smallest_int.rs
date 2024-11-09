use std::cmp::min;

pub fn find_smallest_int(arr: &[i32]) -> i32 {
    let mut s: i32 = arr[0];
    for i in arr {
        s = min(s, *i)
    }
    s
}