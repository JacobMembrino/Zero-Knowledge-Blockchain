use oqs;
use rust_crypto::digest::Digest;
use rust_crypto::sha3::Keccak256;

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

    let public_key = sig.keypair()?;
    let private_key = sig.export_secret_key()?;

    Ok(KeyPair {
        address: String::from(generate_address), 
        private_key: String::from(private_key),
    })
}

fn generate_address(public_key: &[u8]) -> String {
    let mut hasher = Keccak256::new();
    hasher.input(public_key);
    let hash = hasher.result();

    // Take the last 20 bytes of the hash
    let address_bytes = &hash[hash.len() - 20..];

    // Prepend the hex prefix
    let mut address = String::from("0x");
    for byte in address_bytes {
        address.push_str(&format!("{:02x}", byte));
    }
    address
}