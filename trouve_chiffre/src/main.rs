use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Trouve le bon chiffre!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    //println!("La chiffre inconue est {}", secret_number);
    loop{
        println!("Choisi un chiffre");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("J'ai pas compris");
    
        let guess : u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_)  => continue,

            };

        println!("Tu as choisi: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less    => println!("Trop petit"),
            Ordering::Greater => println!("Trop grand"),
            Ordering::Equal   => { 
                println!("Voila! Tres bien!");
                break;
            }
        }
    }
}
