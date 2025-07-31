use crate::{block::Block, blockchain::Blockchain};

mod block;
mod blockchain;
mod util;

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block(1753948887, "First block".into());
    blockchain.add_block(1753948888, "Second block".into());
    blockchain.add_block(1753948889, "Third block".into());
    blockchain.add_block(1753948890, "fourth block".into());
    blockchain.add_block(1753948891, "fifth block".into());

    let is_valid = blockchain.validate();
    println!("Blockchain is valid {:?}", is_valid);
    for block in blockchain.blocks {
        println!("Block {:?}", block);
    }
}
