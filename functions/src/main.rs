fn main() {
    println!("Hello, world!");

    let x: i32 = defined_after_main();

    println!("X is equal to: {}", x)
}

fn defined_after_main() -> i32 {
    5
}
