extern crate time;
extern crate bincode;
extern crate sha1;

use block_content::BlockContent;

#[derive(Clone, Debug)]
pub struct Block {
    index: i64,
    timestamp: i64,
    previous_hash: String,
    hash: String,
    content: BlockContent,
}

impl Block {
    pub fn new(index: i64, previous_hash: &String, content: &String) -> Block {
        let timestamp = time::now_utc().to_timespec().sec;
        let block_content = BlockContent::new(content);
        let mut content_for_hash: String = "".to_owned();

        content_for_hash.push_str(&index.to_string());
        content_for_hash.push_str(&timestamp.to_string());
        content_for_hash.push_str(previous_hash);
        content_for_hash.push_str(content);

        let hash = sha1::Sha1::from(content_for_hash).hexdigest();

        Block {
            index,
            timestamp,
            previous_hash: previous_hash.to_string(),
            hash,
            content: block_content
        }
    }

    pub fn index(&self) -> i64 {
        self.index
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn previous_hash(&self) -> &str {
        &self.previous_hash
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn content(&self) -> &str {
        &self.content.content()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_block() {
        let content = String::from("content");
        let hash = sha1::Sha1::from("hash").hexdigest();
        let block = Block::new(0, &hash, &content);

        assert_eq!(0, block.index());
        assert_eq!(&hash, block.previous_hash());
    }
}
