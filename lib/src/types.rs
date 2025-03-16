use crate::U256;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct Blockchain {
    pub blocks: Vec<Block>,
}



impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>

}

impl Block { 
    pub fn new(
        header: BlockHeader, 
        transactions: Vec<Transaction>
    )-> Self{ 
        Block { 
            header, 
            transactions,
        }
    }

    pub fn hash(&self) -> ! { 
        unimplemented!()
    }
}

pub struct BlockHeader { 
    pub timestamp: DateTime<Utc>,
    pub nonce: u64, 
    pub prev_block_hash: [u8; 32], //array of 32 u8 integers.
    pub merkle_root: [u8; 32],
    pub target: U256,
}

impl BlockHeader { 
    pub fn new(timestamp: DateTime<Utc>, 
        nonce: u64, 
        prev_block_hash: [u8; 32],
        merkle_root: [u8; 32],
        target: U256,
    )-> Self { 
        BlockHeader { 
            timestamp, 
            nonce, 
            prev_block_hash, 
            merkle_root, 
            target
        }
    }
    pub fn hash() -> ! { 
        unimplemented!()
    }
}

pub struct TransactionInput { 
    pub prev_transaction_output_hash: [u8; 32],
    ///Only kept signature field, but in future versions I'll probably try to 
    ///implement a real Script field
    pub signature: [u8; 64]
}

pub struct TransactionOutput { 
    pub value: u64, 
    pub unique_id: Uuid, ///hash of each tx output should be unique (to identify multiple hashes)
    ///0x02 if the Y-coordinate is even.
    ///0x03 if the Y-coordinate is odd (in pubkey)
    pub pubkey: [u8; 33], 
}

pub struct Transaction { 
    inputs :Vec<TransactionInput>, 
    outputs: Vec<TransactionOutput>
}

impl Transaction { 
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self { 
        Transaction ( 
            inputs, 
            outputs
        )
    }

    pub fn hash() -> ! { 
        unimplemented!()
    }
}



