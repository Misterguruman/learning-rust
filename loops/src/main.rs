use std::{thread, time};

fn main() {
    let delay = time::Duration::from_secs(1);
    let mut plz_no_infinite_loops: i32 = 5;

    loop {
        if plz_no_infinite_loops < 0 {
            break;
        }

        println!("Counting down to liftoff: {}", plz_no_infinite_loops);
        plz_no_infinite_loops = plz_no_infinite_loops - 1;
        thread::sleep(delay);
    };

    println!("You have reached liftoff!");
    plz_no_infinite_loops = 0;

    while plz_no_infinite_loops <= 5 {
        let altitude_offset = plz_no_infinite_loops * 10_000;

        println!("Your current altitude is {}", altitude_offset);
        plz_no_infinite_loops = plz_no_infinite_loops + 1;
        thread::sleep(delay);
    };

    println!("You made it to space!")
}
