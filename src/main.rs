mod block;
mod block_content;
mod blockchain;
use blockchain::Blockchain;

fn main() {
    let content1 = String::from("content1");
    let content2 = String::from("content2");
    let mut blockchain = Blockchain::new();
    blockchain.create_block(&content1);
    blockchain.create_block(&content2);

    println!("The blockchain ({:?})", blockchain.blocks());
}
