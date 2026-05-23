fn main() {
    println!("Hello, world!");

    another_function(5, 'h');
}

fn another_function(x: u8, unit_label: char) {
    println!("{x}{unit_label}");
}
