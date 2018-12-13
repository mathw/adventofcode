#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    child_nodes: Vec<Box<Node>>,
    metadata: Vec<u8>,
}

impl Node {
    pub fn from_u8<I: IntoIterator<Item = u8>>(numbers: I) -> Result<Node, String> {
        Err("Not implemented".into())
    }
}

#[test]
fn parse_empty_node() {
    let nums = vec![0, 0];

    let node = Node::from_u8(nums);

    assert_eq!(
        node,
        Ok(Node {
            child_nodes: vec![],
            metadata: vec![]
        })
    );
}
