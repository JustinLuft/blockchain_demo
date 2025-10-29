mod blocks;
use blocks::block::Block;
use blocks::blockchain::Blockchain;

use std::io::{self, Write};

fn main() {
    let mut blockchain = Blockchain::new(2); // difficulty = 2 zeros

    loop {
        println!("\n--- Rust Blockchain ---");
        println!("1. Add Block");
        println!("2. Print Chain");
        println!("3. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Enter block data: ");
                io::stdout().flush().unwrap();
                let mut data = String::new();
                io::stdin().read_line(&mut data).unwrap();
                let data = data.trim().to_string();
                blockchain.add_block(data);
                println!("Block added!");
            }
            "2" => {
                blockchain.print_chain();
            }
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option. Try again."),
        }
    }
}
