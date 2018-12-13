#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<u8>,
}

impl Node {
    pub fn from_u8<I: Iterator<Item = u8>>(numbers: &mut I) -> Result<Node, String> {
        let num_children = numbers
            .next()
            .ok_or("Iterator did not yield number of child nodes".to_owned())?;
        let num_metadata = numbers
            .next()
            .ok_or("Iterator did not yield count of metadata".to_owned())?;

        let mut children = Vec::new();
        for _ in 0..num_children {
            children.push(Node::from_u8(numbers)?);
        }

        let metadata = numbers.take(num_metadata as usize).collect();

        Ok(Node { children, metadata })
    }

    pub fn sum_metadata(&self) -> u16 {
        let me = self.metadata.iter().fold(0, |acc, x| acc + *x as u16);
        let children: u16 = self.children.iter().map(|c| c.sum_metadata()).sum();
        me + children
    }

    pub fn value(&self) -> u16 {
        if self.children.is_empty() {
            self.metadata.iter().fold(0, |acc, x| acc + *x as u16)
        } else {
            self.metadata.iter().fold(0, |acc, index| {
                if *index == 0 {
                    0
                } else {
                    acc + self
                        .children
                        .get((*index - 1) as usize) // indexes are 1-based!!
                        .map(|n| n.value())
                        .unwrap_or(0)
                }
            })
        }
    }
}

#[test]
fn parse_empty_node() {
    let nums = vec![0, 0];

    let node = Node::from_u8(&mut nums.into_iter());

    assert_eq!(
        node,
        Ok(Node {
            children: vec![],
            metadata: vec![]
        })
    );
}

#[test]
fn parse_node_with_no_children_and_metadata() {
    let nums = vec![0, 3, 1, 2, 3];

    let node = Node::from_u8(&mut nums.into_iter());

    assert_eq!(
        node,
        Ok(Node {
            children: vec![],
            metadata: vec![1, 2, 3]
        })
    );
}

#[test]
fn parse_node_with_no_children_and_metadata_and_leftover_numbers() {
    let nums = vec![0, 3, 1, 2, 3, 5, 6, 45];
    let mut iter = nums.into_iter();

    let node = Node::from_u8(&mut iter);

    assert_eq!(
        node,
        Ok(Node {
            children: vec![],
            metadata: vec![1, 2, 3]
        })
    );

    assert_eq!(iter.next(), Some(5));
}

#[test]
fn parse_node_with_one_child_and_two_metadata() {
    let nums = vec![1, 2, 0, 2, 55, 66, 1, 2];

    let node = Node::from_u8(&mut nums.into_iter());

    assert_eq!(
        node,
        Ok(Node {
            children: vec![Node {
                children: vec![],
                metadata: vec![55, 66]
            }],
            metadata: vec![1, 2]
        })
    );
}

#[test]
fn parse_nested_children() {
    let nums = vec![2, 0, 1, 0, 0, 0, 0, 0];
    let node = Node::from_u8(&mut nums.into_iter());

    assert_eq![
        node,
        Ok(Node {
            children: vec![
                Node {
                    children: vec![Node {
                        children: vec![],
                        metadata: vec![]
                    }],
                    metadata: vec![]
                },
                Node {
                    children: vec![],
                    metadata: vec![]
                }
            ],
            metadata: vec![]
        })
    ]
}

#[test]
fn part_one_example() {
    let nums = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];

    let root = Node::from_u8(&mut nums.into_iter()).expect("Should parse example nodes");

    let sum = root.sum_metadata();

    assert_eq!(sum, 138);
}

#[test]
fn node_value_no_children() {
    let nums = vec![0, 2, 5, 6];

    let root = Node::from_u8(&mut nums.into_iter());

    let value = root.map(|n| n.value());

    assert_eq!(value, Ok(11));
}

#[test]
fn node_value_children() {
    let nums = vec![2, 2, 0, 1, 4, 0, 1, 1, 1, 6];

    let root = Node::from_u8(&mut nums.into_iter());

    let value = root.map(|n| n.value());

    assert_eq!(value, Ok(4));
}
