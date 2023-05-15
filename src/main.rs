fn main() {
    // Create a new blockchain
    let mut blockchain = Blockchain::new();

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