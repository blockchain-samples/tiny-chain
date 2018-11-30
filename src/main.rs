#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;


fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("input  a miner addres: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);
    print!("difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("we need an integer");
    println!("generating genesis block!");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);
    


    
}
