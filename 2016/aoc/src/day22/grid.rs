use super::node::Node;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x: x, y: y }
    }

    fn get_possible_adjacents(&self) -> Vec<Point> {
        match (self.x, self.y) {
            (0, 0) => vec![Point::new(1, 0), Point::new(0, 1)],
            (0, y) => vec![Point::new(1, y), Point::new(0, y + 1), Point::new(0, y - 1)],
            (x, 0) => vec![Point::new(x, 1), Point::new(x + 1, 0), Point::new(x - 1, 0)],
            (x, y) => {
                vec![Point::new(x - 1, y),
                     Point::new(x + 1, y),
                     Point::new(x, y - 1),
                     Point::new(x, y + 1)]
            }
        }
    }
}

pub struct Grid {
    nodes: HashMap<Point, Node>,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            nodes: HashMap::new(),
            max_x: 0,
            max_y: 0,
        }
    }

    pub fn add_node(&mut self, node: Node) {
        if node.x > self.max_x {
            self.max_x = node.x;
        }
        if node.y > self.max_y {
            self.max_y = node.y;
        }

        self.nodes.insert(Point::new(node.x, node.y), node);
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    fn nodes_adjacent_to(&self, point: &Point) -> Vec<Node> {
        point.get_possible_adjacents().iter().filter_map(|p| self.nodes.get(p)).cloned().collect()
    }

    pub fn viable_pairs_with(&self, node: &Node) -> Vec<(Node, Node)> {
        self.nodes
            .values()
            .filter(|b| pair_is_viable(node, b))
            .map(|b| (node.clone(), b.clone()))
            .collect()
    }

    pub fn target_node_coordinates(&self) -> Option<Point> {
        let mut x0s = self.nodes.keys().filter(|p| p.y == 0).cloned().collect::<Vec<Point>>();
        x0s.sort_by_key(|p| p.x);
        x0s.reverse();
        if x0s.len() > 0 { Some(x0s[0]) } else { None }
    }

    fn get_smallest_node_size(&self) -> u16 {
        let mut sizes = self.nodes.values().map(|n| n.size).collect::<Vec<_>>();
        sizes.sort();
        sizes[0]
    }

    pub fn render_grid_symbolic(&self, target_node: &Node) -> String {
        let smallest_size = self.get_smallest_node_size();

        let mut result = String::new();
        for y in 0..self.max_y + 1 {
            let node_chars = (0..self.max_x + 1)
                .map(|x| {
                    let this_point = Point { x: x, y: y };
                    let this_node = self.nodes.get(&this_point);
                    match this_node {
                        Some(node) => {
                            (x,
                             y,
                             if node.used == 0 {
                                 '_'
                             } else if node.used > smallest_size {
                                 '#'
                             } else {
                                 '.'
                             })
                        }
                        None => (x, y, '!'),
                    }
                })
                .map(|(x, y, c)| if x == target_node.x && y == target_node.y {
                    " G ".to_owned()
                } else if x == 0 && y == 0 {
                    format!("({})", c)
                } else {
                    format!(" {} ", c)
                });
            for n in node_chars {
                result.push_str(&n);
            }
            result.push_str("\n");
        }

        result
    }

    pub fn get_node_at(&self, point: &Point) -> Option<&Node> {
        self.nodes.get(point)
    }

    pub fn swap_nodes(&mut self, a: &Point, b: &Point) {
        let a_node = self.nodes.get(a).unwrap().clone();
        let b_node = self.nodes.get(b).unwrap().clone();

        *(self.nodes.get_mut(b).unwrap()) = a_node;
        *(self.nodes.get_mut(a).unwrap()) = b_node;
    }

    pub fn get_empty_node(&self) -> Option<&Node> {
        self.nodes.values().filter(|n| n.used == 0).next()
    }
}

fn pair_is_viable(a: &Node, b: &Node) -> bool {
    a.used != 0 && (a.x != b.x || a.y != b.y) && a.used <= b.free
}

#[test]
fn test_nodes_adjacent_to() {
    let node0_0 = Node {
        x: 0,
        y: 0,
        size: 1,
        used: 0,
        free: 1,
    };
    let node0_1 = Node {
        x: 0,
        y: 1,
        size: 2,
        used: 1,
        free: 1,
    };
    let node1_1 = Node {
        x: 1,
        y: 1,
        size: 3,
        used: 2,
        free: 1,
    };

    let mut grid = Grid::new();
    grid.add_node(node0_0.clone());
    grid.add_node(node0_1.clone());
    grid.add_node(node1_1.clone());

    let adjs = grid.nodes_adjacent_to(&Point::new(0, 0));
    assert_eq!(adjs, vec![node0_1]);
}

#[test]
fn test_viable_pairs_with() {
    let node0_0 = Node {
        x: 0,
        y: 0,
        size: 1,
        used: 0,
        free: 1,
    };
    let node0_1 = Node {
        x: 0,
        y: 1,
        size: 2,
        used: 1,
        free: 1,
    };
    let node1_1 = Node {
        x: 1,
        y: 1,
        size: 3,
        used: 2,
        free: 1,
    };

    let mut grid = Grid::new();
    grid.add_node(node0_0.clone());
    grid.add_node(node0_1.clone());
    grid.add_node(node1_1.clone());

    // there are no viable pairs with 0,0 as its used is 0
    let pairs = grid.viable_pairs_with(&node0_0);
    assert_eq!(pairs.len(), 0);

    let pairs = grid.viable_pairs_with(&node0_1);
    assert_eq!(pairs.len(), 2);
    let pair1 = (node0_1.clone(), node0_0.clone());
    let pair2 = (node0_1.clone(), node1_1.clone());
    assert!(pairs[0] == pair1 || pairs[1] == pair1);
    assert!(pairs[0] == pair2 || pairs[1] == pair2);

    // no viable pairs with 1,1 as it has too much data on it
    let pairs = grid.viable_pairs_with(&node1_1);
    assert_eq!(pairs.len(), 0);
}

#[test]
fn test_get_target_node() {
    let node0_0 = Node {
        x: 0,
        y: 0,
        size: 1,
        used: 0,
        free: 1,
    };
    let node0_1 = Node {
        x: 0,
        y: 1,
        size: 2,
        used: 1,
        free: 1,
    };
    let node1_1 = Node {
        x: 1,
        y: 1,
        size: 3,
        used: 2,
        free: 1,
    };

    let mut grid = Grid::new();
    grid.add_node(node0_0.clone());
    grid.add_node(node0_1.clone());
    grid.add_node(node1_1.clone());

    assert_eq!(grid.target_node_coordinates(), Some(Point { x: 0, y: 0 }));
}

#[test]
fn test_render_grid_symbolic() {
    let node0_0 = Node {
        x: 0,
        y: 0,
        size: 1,
        used: 0,
        free: 1,
    };
    let node1_0 = Node {
        x: 1,
        y: 0,
        size: 4,
        used: 1,
        free: 3,
    };
    let node0_1 = Node {
        x: 0,
        y: 1,
        size: 2,
        used: 1,
        free: 1,
    };
    let node1_1 = Node {
        x: 1,
        y: 1,
        size: 3,
        used: 2,
        free: 1,
    };

    let mut grid = Grid::new();
    grid.add_node(node0_0.clone());
    grid.add_node(node1_0.clone());
    grid.add_node(node0_1.clone());
    grid.add_node(node1_1.clone());

    assert_eq!(grid.render_grid_symbolic(&node1_0), "(_) G \n .  . \n")
}
