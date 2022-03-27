#[derive(Debug)]
pub(crate) struct NodeKey {
    pub(crate) node: String,
    pub(crate) key: String
}

impl ToString for NodeKey {
    fn to_string(&self) -> String {
        format!("node: {}, key: {}", self.node, self.key)
    }
}