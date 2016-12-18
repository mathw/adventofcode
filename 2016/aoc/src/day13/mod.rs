pub fn do_day13() {
    let maze = Maze {
        width: 60,
        height: 60,
        magic: 1352,
    };

    println!("Shortest path to 31, 39 is {} steps",
             maze.shortest_path_from_to(&Point { x: 1, y: 1 }, &Point { x: 31, y: 39 }));

    println!("Locations reachable in at most 50 steps = {}",
             maze.find_locations_at_most_steps(&Point { x: 1, y: 1 }, 50)
                 .len() + 1);
}

fn is_wall(x: usize, y: usize, fav: usize) -> bool {
    let a = (x * x) + (3 * x) + (2 * x * y) + y + (y * y);
    is_odd(bit_count(a + fav))
}

fn bit_count(i: usize) -> usize {
    let mut v = i;
    let mut count = 0;

    while v > 0 {
        if v & 1 == 1 {
            count += 1;
        }

        v >>= 1;
    }

    return count;
}

fn is_odd(i: usize) -> bool {
    i % 2 != 0
}

/// from x and y coordinates find the location in the maze vector which has that cell
fn vec_index_for(x: usize, y: usize, width: usize) -> usize {
    (y * width) + x
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

struct Maze {
    width: usize,
    height: usize,
    magic: usize,
}

impl Maze {
    fn is_wall(&self, point: &Point) -> bool {
        is_wall(point.x, point.y, self.magic)
    }

    /// Find reachable cells from a start point given a test function
    /// The test function returns true when the cell is reachable
    fn reachable_cells_from<F>(&self, from: &Point, test: F) -> Vec<Point>
        where F: Fn(&Point) -> bool
    {
        let mut v = Vec::new();
        if from.y > 0 {
            // can go up
            let up = Point { y: from.y - 1, ..*from };
            if test(&up) {
                v.push(up);
            }
        }
        if from.y < self.height - 1 {
            let down = Point { y: from.y + 1, ..*from };
            if test(&down) {
                v.push(down);
            }
        }
        if from.x > 0 {
            let left = Point { x: from.x - 1, ..*from };
            if test(&left) {
                v.push(left);
            }
        }
        if from.x < self.width - 1 {
            let right = Point { x: from.x + 1, ..*from };
            if test(&right) {
                v.push(right);
            }
        }

        v
    }
    /// Mark a maze from start to target
    /// Start is 1, walls/untested are 0
    fn mark_maze(&self, start: &Point, target: &Point) -> Vec<u32> {
        let mut marked = vec![0; self.width * self.height];
        let value_at = |p: &Point, m: &Vec<u32>| m[vec_index_for(p.x, p.y, self.width)];

        // mark start point
        marked[vec_index_for(start.x, start.y, self.width)] = 1;

        // cells to consider from this point
        let mut sources = vec![*start];
        let mut phase = 2;
        let mut new_marks = vec![];

        loop {
            for source in sources {
                // find reachable unmarked non-walls
                let reachables = self.reachable_cells_from(&source, |p| {
                    !self.is_wall(p) && value_at(p, &marked) == 0
                });
                for r in reachables {
                    marked[vec_index_for(r.x, r.y, self.width)] = phase;
                    new_marks.push(r);
                    if r == *target {
                        return marked;
                    }
                }
            }
            sources = new_marks.clone();
            new_marks.clear();
            phase += 1;
        }
    }

    fn find_locations_at_most_steps(&self, start: &Point, steps: u32) -> Vec<Point> {
        let mut marked = vec![0; self.width * self.height];
        let value_at = |p: &Point, m: &Vec<u32>| m[vec_index_for(p.x, p.y, self.width)];

        // mark start point
        marked[vec_index_for(start.x, start.y, self.width)] = 1;

        // cells to consider from this point
        let mut sources = vec![*start];
        let mut phase = 2;
        let mut new_marks = vec![];
        let mut visited = vec![];

        loop {
            for source in sources {
                // find reachable unmarked non-walls
                let reachables = self.reachable_cells_from(&source, |p| {
                    !self.is_wall(p) && value_at(p, &marked) == 0
                });
                for r in reachables {
                    marked[vec_index_for(r.x, r.y, self.width)] = phase;
                    new_marks.push(r);
                    visited.push(r);
                }
            }
            if phase == steps + 1 {
                // all done!
                return visited;
            }
            sources = new_marks.clone();
            new_marks.clear();
            phase += 1;
        }
    }

    fn shortest_path_from_to(&self, from: &Point, to: &Point) -> u32 {
        let marked = self.mark_maze(from, to);
        marked[vec_index_for(to.x, to.y, self.width)] - 1
    }
}

#[test]
fn test_is_odd() {
    assert!(is_odd(57));
    assert!(is_odd(98319873159871));
    assert!(!is_odd(54));
}

#[test]
fn test_bitcount() {
    assert_eq!(bit_count(0b101), 2);
    assert_eq!(bit_count(0b001), 1);
    assert_eq!(bit_count(0), 0);
}

#[test]
fn test_is_wall() {
    let walls1 = vec![false, true, false, true, true, true, true, false, true, true];
    let line1 = (0..10).map(|x| is_wall(x, 0, 10)).collect::<Vec<bool>>();
    assert_eq!(walls1, line1);

    let walls2 = vec![false, false, true, false, false, true, false, false, false, true];
    let line2 = (0..10).map(|x| is_wall(x, 1, 10)).collect::<Vec<bool>>();
    assert_eq!(walls2, line2);

    let walls3 = vec![true, false, false, false, false, true, true, false, false, false];
    let line3 = (0..10).map(|x| is_wall(x, 2, 10)).collect::<Vec<bool>>();
    assert_eq!(walls3, line3);
}

#[test]
fn test_reachable_from() {
    let maze = Maze {
        width: 10,
        height: 7,
        magic: 10,
    };
    let reachables = maze.reachable_cells_from(&Point { x: 1, y: 1 }, |p| !maze.is_wall(p));
    assert_eq!(reachables.len(), 2);
    assert!(reachables.contains(&Point { x: 0, y: 1 }));
    assert!(reachables.contains(&Point { x: 1, y: 2 }));
}

#[test]
fn test_mark_maze() {
    let maze = Maze {
        width: 10,
        height: 7,
        magic: 10,
    };

    let marked = maze.mark_maze(&Point { x: 1, y: 1 }, &Point { x: 7, y: 4 });
    assert_eq!(marked[vec_index_for(1, 1, maze.width)], 1);
    assert_eq!(marked[vec_index_for(0, 1, maze.width)], 2);
    assert_eq!(marked[vec_index_for(1, 2, maze.width)], 2);
    assert_eq!(marked[vec_index_for(2, 2, maze.width)], 3);
    assert_eq!(marked[vec_index_for(3, 2, maze.width)], 4);
    assert_eq!(marked[vec_index_for(3, 1, maze.width)], 5);
    assert_eq!(marked[vec_index_for(4, 2, maze.width)], 5);
    assert_eq!(marked[vec_index_for(3, 3, maze.width)], 5);
    assert_eq!(marked[vec_index_for(7, 4, maze.width)], 12);

    assert_eq!(maze.shortest_path_from_to(&Point { x: 1, y: 1 }, &Point { x: 7, y: 4 }),
               11);
}

#[test]
fn test_number_in_steps() {
    let maze = Maze {
        width: 10,
        height: 7,
        magic: 10,
    };
    let start = Point { x: 1, y: 1 };

    let in_one = maze.find_locations_at_most_steps(&start, 1);
    assert_eq!(in_one.len(), 2);
    assert!(in_one.contains(&Point { x: 0, y: 1 }));
    assert!(in_one.contains(&Point { x: 1, y: 2 }));

    let in_two = maze.find_locations_at_most_steps(&start, 2);
    assert_eq!(in_two.len(), 4);
    assert!(in_two.contains(&Point { x: 0, y: 1 }));
    assert!(in_two.contains(&Point { x: 1, y: 2 }));
    assert!(in_two.contains(&Point { x: 0, y: 0 }));
    assert!(in_two.contains(&Point { x: 2, y: 2 }));
}
