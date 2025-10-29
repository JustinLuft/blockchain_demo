// Declare the blocks module so Rust knows where to find block and blockchain modules
mod blocks;

// Import the Block struct and Blockchain struct from the blocks module
use blocks::block::Block;
use blocks::blockchain::Blockchain;

// Import standard I/O traits for reading from stdin and writing to stdout
use std::io::{self, Write};

// Entry point of the Rust program
fn main() {
    // Create a new blockchain with difficulty = 2 (hash must start with 2 zeros)
    let mut blockchain = Blockchain::new(2);

    // Start an infinite loop for the interactive terminal menu
    loop {
        // Print menu options
        println!("\n--- Rust Blockchain ---");
        println!("1. Add Block");
        println!("2. Print Chain");
        println!("3. Exit");

        // Prompt the user for input
        print!("Choose an option: ");
        io::stdout().flush().unwrap(); // Flush stdout to ensure the prompt appears immediately

        // Read user input
        let mut choice = String::new(); // Create a mutable string to store input
        io::stdin().read_line(&mut choice).unwrap(); // Read a line from stdin
        let choice = choice.trim(); // Remove whitespace/newline from input

        // Match user input to corresponding actions
        match choice {
            "1" => {
                // Option 1: Add a new block
                print!("Enter block data: ");
                io::stdout().flush().unwrap(); // Flush prompt
                let mut data = String::new(); // Create mutable string for block data
                io::stdin().read_line(&mut data).unwrap(); // Read user input
                let data = data.trim().to_string(); // Trim and convert to owned String
                blockchain.add_block(data); // Add the new block to the blockchain
                println!("Block added!"); // Confirm to user
            }
            "2" => {
                // Option 2: Print the entire blockchain
                blockchain.print_chain(); // Call the print_chain method
            }
            "3" => {
                // Option 3: Exit the program
                println!("Exiting...");
                break; // Exit the loop and end the program
            }
            _ => println!("Invalid option. Try again."), // Handle invalid input
        }
    }
}
