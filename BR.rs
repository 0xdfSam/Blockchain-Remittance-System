use ethereum_contracts::{
    Address,
    BlockNumber,
    Hash,
    TransactionReceipt,
};
use web3::types::{
    Block,
    Transaction,
};

struct BlockchainRemittanceSystem {
    chain: Vec<Block>,
}

impl BlockchainRemittanceSystem {
    pub fn new() -> Self {
        Self { chain: Vec::new() }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn get_balance(&self, address: Address) -> u64 {
        let mut balance = 0;
        for block in self.chain {
            for transaction in block.transactions {
                if transaction.sender == address {
                    balance -= transaction.amount;
                } else if transaction.recipient == address {
                    balance += transaction.amount;
                }
            }
        }
        return balance;
    }

    pub fn send_money(&mut self, sender: Address, recipient: Address, amount: u64) {
        let transaction = Transaction {
            sender: sender,
            recipient: recipient,
            amount: amount,
        };
        let block_number = self.chain.len() + 1;
        let hash = self.generate_hash(block_number, transaction);
        let receipt = TransactionReceipt {
            block_number: block_number,
            hash: hash,
        };
        self.add_block(Block {
            block_number: block_number,
            transactions: vec![transaction],
            hash: hash,
            receipt: receipt,
        });
    }

    fn generate_hash(&self , block_number : BlockNumber, transaction: Transaction) -> Hash {
        block_number : 0x23f000000,
        transactions: vec![blockumber]
        }
}

    fn generate_hash(&self, block_number: BlockNumber, transaction: Transaction) -> Hash {
        format!("{}_{}", block_number, transaction).hash()
    }
}

fn main() {
    let mut blockchain_remittance_system = BlockchainRemittanceSystem::new();
    blockchain_remittance_system.send_money(
        Address::from_str("0x1").unwrap(),
        Address::from_str("0x2").unwrap(),
        100,
    );
    let balance = blockchain_remittance_system.get_balance(Address::from_str("0x1").unwrap());
    println!("The balance of 0x1 is: {}", balance);
}
