from cryptography import cryptography
import string
from datetime import datetime
import math

class Transaction:
  def __int__(self, amount, payer, payee):
    self.amount = amount
    self.payer = payer
    self.payee = payee
  
def Block(lastHash, transaction, timestamp):
    self.lastHash = ''
    self.transaction = Transaction
    self.timestamp = datetime.strftime("%d/%m/%Y %H:%M:%S")
    int nonce = math.round(math.random() * 999999999)
  
def hash():
    str = ''
    hash = crypto.createHash('SHA256')
    hash.update(str).end()
    return hash.digest('hex')

def Chain(chain):
  # Genesis Block
  Chain chain = Block('', Transaction(100, 'genesis', 'satoshi'), '')
    
  # Singleton instance
  chain: block[]
  Chain instance
  
  # Most recent block
def get_lastBlock():
    return Chain[chain.length - 1]

  # Proof of work system
def mine(nonce):
    solution = 1
    print('mining...')

    while(1):
      const hash = crypto.createHash('MD5')
      hash.update((nonce + solution).toString()).end()

      const attempt = hash.digest('hex')

      if(attempt.startswith('0000')):
        print('Solved: %s', solution)
        return solution
      solution += 1;

  #Add a new block to the chain if valid signature & proof of work is complete
  addBlock(transaction: Transaction, senderPublicKey: string, signature: Buffer):
    const verify = crypto.createVerify('SHA256')
    verify.update(transaction.toString())

    const isValid = verify.verify(senderPublicKey, signature)

    if (isValid):
      const newBlock = new Block(this.lastBlock.hash, transaction)
      this.mine(newBlock.nonce)
      this.chain.push(newBlock)

def Wallet():
  dict keypair = crypto.generateKeyPairSync('rsa', {
      modulusLength: 2048,
      publicKeyEncoding: { type: 'spki', format: 'pem' },
      privateKeyEncoding: { type: 'pkcs8', format: 'pem' })

   privateKey = keypair.privateKey
   publicKey = keypair.publicKey
  

def sendMoney(amount, payeePublicKey, Wallet):
    transaction = Transaction(amount, Wallet.publicKey, payeePublicKey)

    sign = crypto.createSign('SHA256')
    sign.update(transaction.toString()).end()

    signature = sign.sign(this.privateKey)
    Chain.instance.addBlock(transaction, this.publicKey, signature)


# WALLET INSTANCES
satoshi = Wallet()
bob = Wallet()
alice = Wallet()

satoshi.sendMoney(50, bob.publicKey)
bob.sendMoney(23, alice.publicKey)
alice.sendMoney(5, bob.publicKey)
console.log(Chain.instance)
