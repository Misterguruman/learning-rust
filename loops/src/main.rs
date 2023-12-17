use std::{thread, time::{self, Duration}};

fn main() {
    const DELAY: Duration = time::Duration::from_secs(0);
    const CREDITS: [&str; 3] = ["Joseph", "Paul", "Langford"];
    let countdown_range = 1..6;

    // loop { replacing manual exit of "loop" with "for x in collection" method
    for i in countdown_range.rev() {
        if i < 0 {
            break;
        }

        println!("Counting down to liftoff: {}", i);
        thread::sleep(DELAY);
    };

    println!("You have reached liftoff!");

    // while plz_no_infinite_loops <= 5 {
    // creating example of using unnamed range to iterate
    for i in 1..6 {
        let altitude_offset = i * 10_000;

        println!("Your current altitude is {}", altitude_offset);

        thread::sleep(DELAY);
    };

    println!("You made it to space!");

    for name in CREDITS {
        println!("Made possible by: {}", name);
        thread::sleep(DELAY);
    }
}
