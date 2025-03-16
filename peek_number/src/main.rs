use std::io;
use std::cmp::Ordering;
use rand::Rng; //Trait
use std::io::Error;

fn main() {
    let _ = game();
}

fn game() -> Result<(), io::Error>{
    let mut next_game = String::new();
    loop {
        let secret: i32 = rand::rng().random_range(1..=100);
        let mut pick = String::new();
        loop {
            println!("pick a number from 1..100 {secret}");
            pick.clear();
            io::stdin().read_line(&mut pick)?;
            let pick: i32 = pick.trim().parse().expect("not an integer {pick}");
            match pick.cmp(&secret) {
                Ordering::Less => println!("Number is smaller than the secret!"),
                Ordering::Greater => println!("Number is greater than the secret!"),
                Ordering::Equal => {
                    println!("Success!");
                    break;
                }
            }
        }
        println!("Do you want to play again yes/no:");
        next_game.clear();
        io::stdin().read_line(&mut next_game)?;
        if !next_game.trim().eq("yes") {
            break;
        }
    }
    Ok(())
}