mod wallet; // Import the wallet.rs file
use wallet::Wallet; // Import the Wallet struct from wallet.rs
use std::time::SystemTime;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[derive(Clone, Debug)]
pub struct NFT {
    pub owner: String,
    pub token_id: String,
}

#[derive(Clone, Debug)]
pub struct Transaction {
    sender: Wallet,
    receiver: Wallet,
    value: u64,
    nft: Option<NFT>,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub prev_block_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub nfts: Vec<NFT>,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub wallets: Vec<Wallet>,
}

impl Transaction {
    pub fn new(sender: Wallet, receiver: Wallet, value: u64, nft: Option<NFT>) -> Self {
        Transaction {
            sender,
            receiver,
            value,
            nft,
        }
    }
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, prev_block_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let nfts = transactions
            .iter()
            .filter_map(|transaction| transaction.nft.clone())
            .collect();

        let mut block = Block {
            index,
            timestamp,
            transactions,
            prev_block_hash,
            hash: String::new(),
            nonce: 0,
            nfts,
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
            wallets: vec![],
        }
    }

    pub fn add_block(&mut self, sender_address: &str, receiver_address: &str, value: u64, nft: Option<NFT>) {
        let sender = self.find_wallet(sender_address).unwrap().clone();
        let receiver = self.find_wallet(receiver_address).unwrap().clone();
        let transaction = Transaction::new(sender, receiver, value, nft);

        let prev_block_hash = self.blocks.last().unwrap().hash.clone();
        let new_block = Block::new((self.blocks.len() as u64), vec![transaction], prev_block_hash);
        self.blocks.push(new_block);
    }

    pub fn create_wallet(&mut self) -> Wallet {
        let wallet = Wallet::new();
        self.wallets.push(wallet.clone());
        wallet
    }

    fn find_wallet(&self, address: &str) -> Option<&Wallet> {
        self.wallets.iter().find(|wallet| wallet.address == address)
    }
}