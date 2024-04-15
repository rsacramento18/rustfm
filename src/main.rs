use std::fs::read_to_string;

fn main() {
    let file_name = std::env::args().nth(1).expect("file name to be passed in");

    let file = read_to_string(file_name).expect("file to exist");

    file.lines().for_each(|line| println!("{}", line));
}
