use crate::wallet::Wallet;
use ring::digest::{Context, Digest, SHA256};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

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
        let data = format!(
            "{}{}{:?}{}{}",
            self.index, self.timestamp, self.transactions, self.prev_block_hash, self.nonce
        );
        hasher.input(data.as_bytes());
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

    pub fn add_block(
        &mut self,
        sender_address: &str,
        receiver_address: &str,
        value: u64,
        nft: Option<NFT>,
    ) -> Result<(), BlockchainError> {
        let sender = self
            .find_wallet(sender_address)
            .ok_or(BlockchainError::WalletNotFound)?;
        let receiver = self
            .find_wallet(receiver_address)
            .ok_or(BlockchainError::WalletNotFound)?;
        let transaction = Transaction::new(sender.clone(), receiver.clone(), value, nft);

        let prev_block_hash = self.blocks.last().unwrap().hash.clone();
        let new_block = Block::new(
            (self.blocks.len() as u64),
            vec![transaction],
            prev_block_hash,
        );
        self.blocks.push(new_block);
        Ok(())
    }

    pub fn create_wallet(&mut self) -> Result<Wallet, Box<dyn std::error::Error>> {
        let wallet = Wallet::new()?;
        self.wallets.push(wallet.clone());
        Ok(wallet)
    }

    fn find_wallet(&self, address: &str) -> Option<&Wallet> {
        self.wallets.iter().find(|wallet| wallet.address == address)
    }
}

#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("Wallet not found")]
    WalletNotFound,
}
