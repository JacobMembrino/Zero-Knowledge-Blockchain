from cryptography import *
from datetime import *
import string, random

chain = Block[]

class Transaction:
  def __init__(self, amount, payer, payee):
    self.amount = amount
    self.payer = payer
    self.payee = payee
  
class Block:
    int nonce = random.randint(0, 999999999)
    
    def __init__(self, lastHash, transaction):
      self.lastHash = lasthash
      self.transaction = transaction
      self.timestamp = datetime.strftime("%d/%m/%Y %H:%M:%S")
  
    def gethash():
      hash = crypto.createHash('SHA256')
      return(hash.digest('hex'))

class Chain:
  def __init__(self, chain):
    # Genesis Block
    transaction1 = Transaction(100, 'genesis', 'coin_maker')
    chain.append(Block('', transaction1)
  
  # Most recent block
  def get_lastBlock():
    return chain[-1]

  # Proof of work system
  def mine(nonce):
    solution = 1
    print('mining...')

    while(1):
      hash = crypto.createHash('MD5')
      hash = str(hash) + str(nonce) + str(solution)

      attempt = hash.digest('hex')

      if(attempt[:3] == '0000'):
        print('Solved: %s', solution)
        break
      solution += 1
      
   return solution

  #Add a new block to the chain if valid signature & proof of work is complete
  def addBlock(transaction, senderPublicKey, signature):
    verify = crypto.createVerify('SHA256')
    verify = verify + transaction.toString()

    isValid = verify.verify(senderPublicKey, signature)

    if (isValid):
      newBlock = new Block(newBlock.lastBlock.hash, transaction)
      newBlock.mine(newBlock.nonce)
      chain.append(newBlock)

class Wallet:
  privateKey = keypair.privateKey
  publicKey = keypair.publicKey
                 
  def __init__(self):
    dict keypair = crypto.generateKeyPairSync('rsa', {
      modulusLength: 2048,
      publicKeyEncoding: { type: 'spki', format: 'pem' },
      privateKeyEncoding: { type: 'pkcs8', format: 'pem' })

  def sendMoney(amount, payeePublicKey, Wallet):
    transaction = Transaction(amount, Wallet.publicKey, payeePublicKey)

    sign = crypto.createSign('SHA256')
    sign = sign + str(transaction)
    signature = sign.sign(Wallet.privateKey)
      
    chain.append(Chain.addBlock(transaction, publicKey, signature))


# WALLET INSTANCES
coin_maker = Wallet() #you
bob = Wallet()
alice = Wallet()

satoshi.sendMoney(50, bob.publicKey)
bob.sendMoney(23, alice.publicKey)
alice.sendMoney(5, bob.publicKey)
print(chain)
