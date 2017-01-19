use std::str::FromStr;
use regex::Regex;

/// A structure holds information about a storage node
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub x: usize,
    pub y: usize,
    pub size: u16,
    pub used: u16,
    pub free: u16,
}

impl FromStr for Node {
    type Err = String;

    fn from_str(src: &str) -> Result<Node, String> {
        lazy_static! {
            static ref NODE: Regex = Regex::new(r"^/dev/grid/node-x(?P<x>\d+)-y(?P<y>\d+)\s+(?P<size>\d+)T\s+(?P<used>\d+)T\s+(?P<free>\d+)T").unwrap();
        }

        if let Some(caps) = NODE.captures(src) {
            if let (Some(x), Some(y), Some(size), Some(used), Some(free)) =
                (caps.name("x"),
                 caps.name("y"),
                 caps.name("size"),
                 caps.name("used"),
                 caps.name("free")) {
                let x = usize::from_str(x).map_err(|_| "Unable to convert x to usize".to_owned())?;
                let y = usize::from_str(y).map_err(|_| "Unable to convert y to usize".to_owned())?;
                let size =
                    u16::from_str(size).map_err(|_| "Unable to convert size to u16".to_owned())?;
                let used =
                    u16::from_str(used).map_err(|_| "Unable to convert used to u16".to_owned())?;
                let free =
                    u16::from_str(free).map_err(|_| "Unable to convert used to u16".to_owned())?;

                return Ok(Node {
                    x: x,
                    y: y,
                    size: size,
                    used: used,
                    free: free,
                });
            } else {
                return Err("Unable to match x, y, size and used from input string".to_owned());
            }
        }

        return Err("Unable to match source string against regex".to_owned());
    }
}


#[test]
fn test_from_str_node() {
    let input = "/dev/grid/node-x0-y6     88T   67T    21T   76%";
    let r = Node::from_str(input);

    if let Ok(node) = r {
        assert_eq!(node,
                   Node {
                       x: 0,
                       y: 6,
                       size: 88,
                       used: 67,
                       free: 21,
                   });
        assert_eq!(node.available(), 21);
        assert_eq!(node.size - node.used, node.free);
    } else {
        assert!(false);
    }
}

#[test]
fn test_from_str_bad_str() {
    let input = "/dev/grid/node-x8  88T 19T";
    let r = Node::from_str(input);
    assert!(r.is_err());
}
