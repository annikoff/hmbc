pub mod block;
pub mod block_content;
use block::Block;

fn main() {
    let content = String::from("content");
    let hash = String::from("hash");
    let block = Block::new(0, &hash, &content);

    println!("The block index ({})", block.index());
}
