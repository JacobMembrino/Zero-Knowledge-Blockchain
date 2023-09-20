mod chain;
mod wallet;
use chain::{Blockchain};

fn main() {
    // Create a new blockchain
    let mut blockchain = Blockchain::new();

    // Create wallets
    let wallet1 = blockchain
        .create_wallet()
        .expect("Failed to create wallet1");
    let wallet2 = blockchain
        .create_wallet()
        .expect("Failed to create wallet2");

    // Print the addresses of the wallets
    println!("Address of Wallet 1: {}", wallet1.address);
    println!("Address of Wallet 2: {}", wallet2.address);

    // Add a block with a transaction
    blockchain
        .add_block(&wallet1.address, &wallet2.address, 10, None)
        .expect("Failed to add block");

    // Print the blockchain
    println!("{:#?}", blockchain.blocks);
}
