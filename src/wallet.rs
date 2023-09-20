// Kyber and other such latice algorithms are quantum safe
// and will be used in this and all future blockchain projects

use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::{EncodeRsaPublicKey, LineEnding};
use sha3::{Digest, Keccak256};

#[derive(Clone, Debug)]
pub struct Wallet {
    pub address: String,
    pub priv_key: rsa::RsaPrivateKey,
}

impl Wallet {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Generate a new key pair
        let key_pair = generate_key_pair()?;

        Ok(Wallet {
            address: key_pair.address,
            priv_key: key_pair.priv_key,
        })
    }
}

struct KeyPair {
    address: String,
    priv_key: RsaPrivateKey,
}

fn generate_key_pair() -> Result<KeyPair, rsa::errors::Error> {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    let public_key_pem = pub_key.to_pkcs1_pem(LineEnding::CRLF)?;

    Ok(KeyPair {
        address: generate_address(public_key_pem.as_bytes()),
        priv_key,
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
