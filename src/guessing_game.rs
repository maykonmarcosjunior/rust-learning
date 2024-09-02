/*
the program will generate a random integer between 1 and 100.
It will then prompt the player to enter a guess.
After a guess is entered, the program will indicate whether the guess is too low or too high.
If the guess is correct, the game will print a congratulatory message and exit.
*/

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Insira o número chutado:");
    
        let resposta = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut chute = String::new();
        io::stdin()
            .read_line(&mut chute)
            .expect("Problemas para ler a entrada");

        let chute: u32 = match chute.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // let chute: u32 = chute.trim().parse().expect("Insira o valor de seu chute");
        
        println!("Seu chute foi: {chute}");
        match chute.cmp(&resposta) {
            Ordering::Greater => println!("Grande demais"),
            Ordering::Less => println!("Baixo demais"),
            Ordering::Equal => {println!("Acertou!!!"); break;},
        }        
    }
    println!("a resposta é {resposta}");
}

