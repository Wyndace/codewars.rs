mod tasks;

fn main() {
    println!("{}", tasks::persistent_burger::persistence(39));
    println!("{}", tasks::persistent_burger::persistence(999));
    println!("{}", tasks::persistent_burger::persistence(4));
    println!("{}", tasks::persistent_burger::persistence(25));
}
