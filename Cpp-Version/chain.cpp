#include <chrono>
#include <vector>
#include <optional>
#include <cryptopp/rsa.h>
#include <cryptopp/sha.h>
#include <cryptopp/hex.h>
#include <cryptopp/osrng.h>
#include <cryptopp/filters.h>
#include <cryptopp/files.h>
#include <cryptopp/cryptlib.h>

using namespace CryptoPP;

class NFT
{
public:
    std::string owner;
    std::string token_id;
};

class Wallet
{
public:
    std::string address;
    RSA::PrivateKey private_key;

    Wallet()
    {
        AutoSeededRandomPool rng;
        InvertibleRSAFunction params;
        params.GenerateRandomWithKeySize(rng, 2048);

        private_key = RSA::PrivateKey(params);
        RSA::PublicKey public_key(params);

        // Serialize public key
        std::string public_key_str;
        StringSink sink(public_key_str);
        public_key.DEREncode(sink);
        sink.MessageEnd();

        // Generate address using last 20 bytes of hashed public key
        address = GenerateAddress(public_key_str);
    }

    std::string GenerateAddress(const std::string &public_key)
    {
        SHA256 hash;
        std::string digest;
        StringSource(public_key, true, new HashFilter(hash, new HexEncoder(new StringSink(digest))));

        // Take the last 20 bytes of the hash
        return "0x" + digest.substr(digest.size() - 20);
    }
};

class Transaction
{
public:
    Wallet sender;
    Wallet receiver;
    unsigned long long value;
    std::optional<NFT> nft;

    Transaction(Wallet sender, Wallet receiver, unsigned long long value, std::optional<NFT> nft)
        : sender(sender), receiver(receiver), value(value), nft(nft) {}
};

class Block
{
public:
    unsigned long long index;
    unsigned long long timestamp;
    std::vector<Transaction> transactions;
    std::string prev_block_hash;
    std::string hash;
    unsigned long long nonce;
    std::vector<NFT> nfts;

    Block(unsigned long long index, std::vector<Transaction> transactions, std::string prev_block_hash)
        : index(index), transactions(transactions), prev_block_hash(prev_block_hash), nonce(0)
    {
        timestamp = std::chrono::duration_cast<std::chrono::seconds>(std::chrono::system_clock::now().time_since_epoch()).count();

        for (const auto &transaction : transactions)
        {
            if (transaction.nft.has_value())
            {
                nfts.push_back(transaction.nft.value());
            }
        }

        hash = ComputeHash();
        MineBlock(4);
    }

    std::string ComputeHash()
    {
        SHA256 hash;
        std::string data = std::to_string(index) + std::to_string(timestamp) + prev_block_hash + std::to_string(nonce);
        std::string digest;
        StringSource(data, true, new HashFilter(hash, new HexEncoder(new StringSink(digest))));
        return digest;
    }

    void MineBlock(size_t difficulty)
    {
        std::string target(difficulty, '0');
        while (hash.substr(0, difficulty) != target)
        {
            nonce++;
            hash = ComputeHash();
        }
    }
};

class Blockchain
{
public:
    std::vector<Block> blocks;
    std::vector<Wallet> wallets;

    Blockchain()
    {
        blocks.push_back(Block(0, {}, "0"));
    }

    void AddBlock(std::string sender_address, std::string receiver_address, unsigned long long value, std::optional<NFT> nft)
    {
        Wallet sender = FindWallet(sender_address);
        Wallet receiver = FindWallet(receiver_address);
        Transaction transaction(sender, receiver, value, nft);

        std::string prev_block_hash = blocks.back().hash;
        blocks.push_back(Block(blocks.size(), {transaction}, prev_block_hash));
    }

    Wallet CreateWallet()
    {
        Wallet wallet;
        wallets.push_back(wallet);
        return wallet;
    }

    Wallet FindWallet(std::string address)
    {
        for (const auto &wallet : wallets)
        {
            if (wallet.address == address)
            {
                return wallet;
            }
        }
        throw std::runtime_error("Wallet not found");
    }
};