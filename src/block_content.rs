#[derive(Clone, Debug)]
pub struct BlockContent {
    content: String,
}

impl BlockContent {
    pub fn new(content: &String) -> BlockContent {
        BlockContent { content: content.to_string() }
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_block_content() {
        let content = String::from("content");
        let block_content = BlockContent::new(&content);

        assert_eq!(&content, block_content.content());
    }
}
