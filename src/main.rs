use crate::block::Block;

mod block;
mod util;
fn main() {
    let index = 1;
    let timestamp = 1753948887;
    let data = String::from("test");
    let prev_hash = String::from("");
    let nonce = Some(String::from("nonce"));
    let block = Block::new(index, timestamp, data, prev_hash, nonce);
    println!("{:?}", block);
}
