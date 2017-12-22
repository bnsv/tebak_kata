extern crate rand;

//use std::io;
use rand::Rng;

fn main() {
    println!("==== Guess The Number ====");
	let secret_number = rand::thread_rng().gen_range(1,4);
    println!("{}",secret_number);
}
