mod wallet; // Import the wallet.rs file
use wallet::Wallet; // Import the Wallet struct from wallet.rs
use crate::blockchain::{Blockchain, Block, Transaction}; // Import the blockchain

fn main() {
    let mut blockchain = Blockchain::new();
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();

    blockchain.wallets.push(wallet1.clone());
    blockchain.wallets.push(wallet2.clone());

    blockchain.add_block(&wallet1.address, &wallet2.address, 10);

    // Add sample transactions and mine a new block
    let transactions = vec![
        Transaction::new(String::from("Alice"), String::from("Bob"), 100),
        Transaction::new(String::from("Bob"), String::from("Alice"), 50),
    ];
    blockchain.add_block(transactions);

    // Add more transactions and mine another block
    let transactions = vec![
        Transaction::new(String::from("Alice"), String::from("Charlie"), 200),
        Transaction::new(String::from("Charlie"), String::from("Bob"), 75),
    ];
    blockchain.add_block(transactions);

    // Display the current state of the blockchain
    for block in blockchain.blocks.iter() {
        println!("{:?}", block);
    }
}