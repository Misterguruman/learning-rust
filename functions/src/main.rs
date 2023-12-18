fn main() { 
    let mut value: String = String::from("Hello");

    defined_after_main(&mut value);

    println!("{}", value);
}

fn defined_after_main(statement:&mut String) -> () {
    statement.push_str(", World.");
}
