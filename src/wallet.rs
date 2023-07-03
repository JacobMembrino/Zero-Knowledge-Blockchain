// Kyber and other such latice algorithms are quantum safe
// and will be used in this and all future blockchain projects

use rsa::pkcs1::ToRsaPublicKey;
use rsa::RsaPrivateKey;
use sha3::{Digest, Keccak256};

#[derive(Clone, Debug)]
pub struct Wallet {
    pub address: String,
    pub private_key: rsa::RsaPrivateKey,
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
    private_key: RsaPrivateKey,
}

fn generate_key_pair() -> Result<KeyPair, rsa::errors::Error> {
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rand::thread_rng(), bits)?;
    let public_key = private_key.to_public_key();

    let public_key_pem = public_key.to_pkcs1_pem()?;

    Ok(KeyPair {
        address: generate_address(public_key_pem.as_bytes()),
        private_key,
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
