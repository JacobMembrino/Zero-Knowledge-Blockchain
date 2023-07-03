mod wallet; // Import the wallet.rs file
use crate::blockchain::{Block, Blockchain, Transaction, NFT};
use wallet::Wallet; // Import the Wallet struct from wallet.rs // Import the blockchain

fn main() {
    let mut blockchain = Blockchain::new();
    let wallet1 = Wallet::new().expect("Failed to create wallet1");
    let wallet2 = Wallet::new().expect("Failed to create wallet2");

    blockchain.wallets.push(wallet1.clone());
    blockchain.wallets.push(wallet2.clone());

    let nft = Some(NFT {
        owner: String::from("Alice"),
        token_id: String::from("NFT1"),
    });
    blockchain.add_block(&wallet1.address, &wallet2.address, 10, nft.clone());

    // Add sample transactions and mine a new block
    let transactions = vec![
        Transaction::new(wallet1.clone(), wallet2.clone(), 100, None),
        Transaction::new(wallet2.clone(), wallet1.clone(), 50, None),
    ];
    for transaction in transactions {
        blockchain.add_block(
            &transaction.sender.address,
            &transaction.receiver.address,
            transaction.value,
            transaction.nft.clone(),
        );
    }

    // Add more transactions and mine another block
    let wallet3 = Wallet::new().expect("Failed to create wallet3");
    blockchain.wallets.push(wallet3.clone());
    let transactions = vec![
        Transaction::new(wallet1.clone(), wallet3.clone(), 200, None),
        Transaction::new(wallet3.clone(), wallet2.clone(), 75, None),
    ];
    for transaction in transactions {
        blockchain.add_block(
            &transaction.sender.address,
            &transaction.receiver.address,
            transaction.value,
            transaction.nft.clone(),
        );
    }

    // Display the current state of the blockchain
    for block in blockchain.blocks.iter() {
        println!("{:?}", block);
    }
}
