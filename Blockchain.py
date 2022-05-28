from cryptography import cryptography
import string
from datetime import datetime
import math

class Transaction:
  def __int__(self, amount, payer, payee):
    self.amount = amount
    self.payer = payer
    self.payee = payee
  
class Block:
  def __int__(self, lastHash, transaction, timestamp):
    self.lastHash = ''
    self.transaction = Transaction
    self.timestamp = datetime.strftime("%d/%m/%Y %H:%M:%S")
    
  nonce = math.round(math.random() * 999999999)
  
  def hash():
    str = ''
    hash = crypto.createHash('SHA256')
    hash.update(str).end()
    return hash.digest('hex')

class Chain:
  def __init__(self, chain):
    # Genesis Block
    self.chain = Block('', Transaction(100, 'genesis', 'satoshi'), '')
    
  # Singleton instance
  chain: block[]
  public static instance = new Chain()
  
  # Most recent block
  get lastBlock():
    return Chain[chain.length - 1]

  # Proof of work system
  mine(nonce: number):
    solution = 1
    print('⛏️  mining...')

    while(1):
      const hash = crypto.createHash('MD5')
      hash.update((nonce + solution).toString()).end()

      const attempt = hash.digest('hex')

      if(attempt.substr(0,4) === '0000'):
        print(`Solved: ${solution}`)
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

class Wallet:
  publicKey = ''
  privateKey = ''

  def __init__(self):
    const keypair = crypto.generateKeyPairSync('rsa', {
      modulusLength: 2048,
      publicKeyEncoding: { type: 'spki', format: 'pem' },
      privateKeyEncoding: { type: 'pkcs8', format: 'pem' },
    })

    privateKey = keypair.privateKey
    publicKey = keypair.publicKey
  

  sendMoney(amount, payeePublicKey):
    transaction = new Transaction(amount, publicKey, payeePublicKey)

    sign = crypto.createSign('SHA256')
    sign.update(transaction.toString()).end()

    signature = sign.sign(this.privateKey)
    Chain.instance.addBlock(transaction, this.publicKey, signature)


# WALLET INSTANCES
const satoshi = new Wallet()
const bob = new Wallet()
const alice = new Wallet()

satoshi.sendMoney(50, bob.publicKey)
bob.sendMoney(23, alice.publicKey)
alice.sendMoney(5, bob.publicKey)
console.log(Chain.instance)
