!# /usr/bin/python
import * as crypto from 'crypto'
import stringify
from datetime import datetime

class Transaction:
  def __int__(self, amount, payer, payee):
    self.amount = amount
    self.payer = payer
    self.payee = payee
   def toString:
    return 'Amount:', str(amount), 'Payer:', str(payer), 'Payee:', str(payee)
  
class Block:
  def __int__(self, lastHash, transaction, timestamp):
    self.lastHash = string
    self.transaction = Transaction
    self.timestamp = now.strftime("%d/%m/%Y %H:%M:%S")
  public nonce = Math.round(Math.random() * 999999999)
  def hash(self) 
    const str = JSON.stringify(this)
    const hash = crypto.createHash('SHA256')
    hash.update(str).end()
    return hash.digest('hex')

class Chain:
  
class Wallet:
  
  
const satoshi = new Wallet();
const bob = new Wallet();
const alice = new Wallet();

satoshi.sendMoney(50, bob.publicKey);
bob.sendMoney(23, alice.publicKey);
alice.sendMoney(5, bob.publicKey);

console.log(Chain.instance)
