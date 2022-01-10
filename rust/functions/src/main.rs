fn main() {
    println!("Hello, world!");

    another_function(32);
    print_label_mesurement('A', 32)
}

fn another_function(num: i32) {
    println!("the number is {}", num);
}

fn print_label_mesurement(label: char, mesurement: i32) {
    println!("{}: {}", label, mesurement);
}