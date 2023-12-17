fn main() {
    let x = 7;
    let add_more: bool = true;

    if x >= 7 {
        println!("Condition is True!")
    } else {
        println!("Condition is False!")
    };

    let y: i32 = if add_more {
        2
    } else {
        0
    };

    let total: i32 = x + y;
    println!("Your final total is {}", total)
}
