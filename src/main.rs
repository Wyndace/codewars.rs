mod tasks;

fn main() {
    println!("{}", tasks::smallest_int::find_smallest_int(&[34, 15, 88, 2]));
    println!("{}", tasks::smallest_int::find_smallest_int(&[34, -345, -1, 100]));
}
