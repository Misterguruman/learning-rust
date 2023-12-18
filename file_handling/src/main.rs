use std::fs;

fn main() {
    let file_path: &str = "./input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("unable to read file");

    let contents: Vec<&str> = contents.split('\n').collect();

    println!("{}", contents.join(", "));
}
