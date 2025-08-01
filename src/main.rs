use crate::blockchain::Blockchain;

mod block;
mod blockchain;
mod transaction;
mod util;

fn main() {
    let blockchain = Blockchain::new();

    let is_valid = blockchain.validate();
    println!("Blockchain is valid {:?}", is_valid);
    println!("Blockchain {:?}", blockchain);
    for block in blockchain.blocks {
        println!("Block {:?}", block);
    }
}
