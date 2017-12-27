extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("=== KUIS TEBAK ANGKA ===");	
    let angka_rahasia = rand::thread_rng().gen_range(1,4);

    loop {
        println!("Masukan tebakan anda:");
        let mut tebakan = String::new();
        io::stdin().read_line(&mut tebakan)
            .expect("Ulangi tebakan anda!");
        
        let tebakan: u32 = match tebakan.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            
        match tebakan.cmp(&angka_rahasia){
            Ordering::Less => println!("Masih kekecilan"),
            Ordering::Greater => println!("Masih kebesaran"),
            Ordering::Equal => { 
                println!("Tepat sekali!");
                break
            }
        }

    println!("Angkanya {}, tebakan anda {}",angka_rahasia, tebakan );

    }
   }
