mod tasks;

fn main() {
    println!("{}", tasks::traffic_light::update_light("yellow"));
    println!("{}", tasks::traffic_light::update_light("red"));
    println!("{}", tasks::traffic_light::update_light("green"));
}
