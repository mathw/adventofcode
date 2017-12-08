#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Node {
    pub self_weight: u32,
    pub total_weight: u32,
    pub name: String,
    pub supporting: Vec<Node>,
}

impl Node {
    pub fn new(name: String, weight: u32, supporting: Vec<Node>) -> Node {
        let child_weight = total_weight(&supporting);

        Node {
            self_weight: weight,
            name: name,
            supporting: supporting,
            total_weight: child_weight + weight,
        }
    }

    pub fn is_balanced(&self) -> bool {
        let min = self.supporting.iter().map(|s| s.total_weight).min();
        let max = self.supporting.iter().map(|s| s.total_weight).max();

        min == max
    }

    pub fn get_unbalanced_child(&self) -> Option<&Node> {
        self.supporting.iter().filter(|n| !n.is_balanced()).next()
    }
}

fn total_weight(nodes: &Vec<Node>) -> u32 {
    nodes.iter().map(|x| x.total_weight).sum()
}