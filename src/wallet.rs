use oqs;
use sha3::{Digest, Keccak256};

pub struct Wallet {
    pub address: String,
    pub private_key: String,
}

impl Wallet {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Generate a new key pair
        let key_pair = generate_key_pair()?;

        Ok(Wallet {
            address: key_pair.address,
            private_key: key_pair.private_key,
        })
    }
}

struct KeyPair {
    address: String,
    private_key: String,
}

fn generate_key_pair() -> Result<KeyPair, Box<dyn std::error::Error>> {
    let mut ctx = oqs::init()?;
    let alg = ctx.default_sig_alg().unwrap();
    let mut sig = ctx.sig_new(alg)?; // Create a signature object

    let (public_key, private_key) = sig.keypair()?;

    Ok(KeyPair {
        address: generate_address(&public_key),
        private_key: String::from_utf8(private_key)?,
    })
}

fn generate_address(public_key: &[u8]) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(public_key);
    let hash = hasher.finalize();

    // Take the last 20 bytes of the hash
    let address_bytes = &hash[hash.len() - 20..];

    // Prepend the hex prefix
    let mut address = String::from("0x");
    for byte in address_bytes {
        address.push_str(&format!("{:02x}", byte));
    }
    address
}
