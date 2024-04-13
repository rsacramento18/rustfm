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

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        } return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => return false,
            Color::Green => return false,
            Color::Blue => return true,
            Color::Yellow => return true,
        }
    }

}

fn main() {
   let foo = Color::Green; 

   foo.is_green();

}
