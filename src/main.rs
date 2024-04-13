enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

fn print_color(color: Color) -> () {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::Yellow=> println!("yellow"),
    }
}

fn main() {
    print_color(Color::Yellow);
}
