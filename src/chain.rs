use std::time::SystemTime;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[derive(Clone, Debug)]
pub struct Transaction {
    sender: String,
    receiver: String,
    value: u64,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub prev_block_hash: String,
    pub hash: String,
    pub nonce: u64,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, value: u64) -> Self {
        Transaction {
            sender,
            receiver,
            value,
        }
    }
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, prev_block_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut block = Block {
            index,
            timestamp,
            transactions,
            prev_block_hash,
            hash: String::new(),
            nonce: 0,
        };

        block.hash = block.compute_hash();
        block.mine_block(4);

        block
    }

    fn compute_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.input_str(&format!(
            "{}{}{:?}{}{}",
            self.index, self.timestamp, self.transactions, self.prev_block_hash, self.nonce
        ));

        hasher.result_str()
    }

    fn mine_block(&mut self, difficulty: usize) {
        let target = String::from_utf8(vec![b'0'; difficulty]).unwrap();
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.compute_hash();
        }
    }
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, vec![], String::from("0"));
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let prev_block_hash = self.blocks.last().unwrap().hash.clone();
        let new_block = Block::new((self.blocks.len() as u64), transactions, prev_block_hash);
        self.blocks.push(new_block);
    }
}
