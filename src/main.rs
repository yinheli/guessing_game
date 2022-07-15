use std::{cmp::Ordering, process};

use dialoguer;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..=100) as u16;

    // println!("number is: {}", number);

    loop {
        let guess;

        let mut input = dialoguer::Input::<u16>::new();
        let input = input.with_prompt("please input number, range [1,100]");
        if let Ok(x) = input.interact_text() {
            guess = x;
        } else {
            println!("invalid input");
            process::exit(1);
        }

        match guess.cmp(&number) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("You Win!!!");
                break;
            }
            Ordering::Greater => println!("too larger"),
        }
    }
}
