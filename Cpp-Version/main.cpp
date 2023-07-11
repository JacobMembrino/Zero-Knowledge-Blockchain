#include "chain.cpp"

int main()
{
    Blockchain blockchain;
    Wallet wallet1 = blockchain.CreateWallet();
    Wallet wallet2 = blockchain.CreateWallet();

    std::cout << "Created two wallets" << std::endl;
    std::cout << "Wallet1 address: " << wallet1.address << std::endl;
    std::cout << "Wallet2 address: " << wallet2.address << std::endl;

    NFT nft = {"owner1", "token1"};
    blockchain.AddBlock(wallet1.address, wallet2.address, 50, nft);

    std::cout << "Added a block to the blockchain" << std::endl;

    return 0;
}