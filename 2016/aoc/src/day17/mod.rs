use md5;
use rustc_serialize::hex::ToHex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn as_char(&self) -> char {
        match *self {
            Dir::Up => 'U',
            Dir::Down => 'D',
            Dir::Left => 'L',
            Dir::Right => 'R',
        }
    }
}

fn path_as_string(path: &Vec<Dir>) -> String {
    path.iter().map(|d| d.as_char()).collect()
}

trait CanBeDoor {
    fn is_door(&self) -> bool;
}

impl CanBeDoor for char {
    fn is_door(&self) -> bool {
        match *self {
            'b' | 'c' | 'd' | 'e' | 'f' => true,
            _ => false,
        }
    }
}

fn open_doors_here(passcode: &str, path: &Vec<Dir>) -> Vec<Dir> {
    let mut hash_source = passcode.to_owned();
    hash_source.push_str(&path_as_string(path));
    let hash = md5::compute(hash_source.as_bytes()).to_hex();

    // println!("hash computed {} from {}",
    //          hash.chars().take(4).collect::<String>(),
    //          hash_source);

    let mut i = hash.chars();
    let first = i.next().unwrap();
    let second = i.next().unwrap();
    let third = i.next().unwrap();
    let fourth = i.next().unwrap();

    let mut result = Vec::new();

    if first.is_door() {
        result.push(Dir::Up);
    }
    if second.is_door() {
        result.push(Dir::Down);
    }
    if third.is_door() {
        result.push(Dir::Left);
    }
    if fourth.is_door() {
        result.push(Dir::Right);
    }

    result
}


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn go(&self, dir: Dir) -> Point {
        match dir {
            Dir::Left => Point { x: self.x - 1, ..*self },
            Dir::Right => Point { x: self.x + 1, ..*self },
            Dir::Up => Point { y: self.y - 1, ..*self },
            Dir::Down => Point { y: self.y + 1, ..*self },
        }
    }
}

#[derive(Debug)]
struct Maze {
    width: usize,
    height: usize,
    destination: Point,
    calculated: HashMap<Point, HashMap<Vec<Dir>, Vec<Dir>>>,
    passcode: String,
}

impl Maze {
    fn new(passcode: String) -> Maze {
        Maze {
            width: 4,
            height: 4,
            destination: Point { x: 3, y: 3 },
            calculated: HashMap::new(),
            passcode: passcode,
        }
    }

    /// have we visited this room and had this set of doors before?
    fn have_visited(&mut self, room: Point, path: &Vec<Dir>) -> bool {
        let doors = self.get_doors_for(room.clone(), path.clone());
        // println!("Have I been to {:?} via {:?} before? I see doors {:?}",
        //          room,
        //          path,
        //          doors);
        let r = self.calculated.get(&room);
        let result = match r {
            Some(r) => {
                let previous_door_sets_here =
                    r.iter().filter(|&(k, _)| k != path).map(|(_, v)| v).collect::<Vec<_>>();
                // println!("Previous doors here {:?}", previous_door_sets_here);
                previous_door_sets_here.into_iter().any(|d| d.clone() == doors)
            }
            None => false,
        };
        result
    }

    fn get_doors_for(&mut self, room: Point, path: Vec<Dir>) -> Vec<Dir> {
        if room == self.destination {
            // don't need to add doors here
            let r = self.calculated.entry(room).or_insert(HashMap::new());
            r.insert(path, Vec::new());
            return Vec::new();
        }
        if let Some(r) = self.calculated.clone().get_mut(&room) {
            match r.clone().get(&path) {
                Some(p) => p.clone(),
                None => {
                    let doors = open_doors_here(&self.passcode, &path);
                    r.insert(path.clone(), doors.clone());
                    doors
                }
            }
        } else {
            let doors = open_doors_here(&self.passcode, &path);
            let mut newmap = HashMap::new();
            newmap.insert(path.clone(), doors.clone());
            self.calculated.insert(room, newmap);
            doors
        }
    }

    fn follow_all_routes(&mut self) {
        // this turns out to be completely specialised for short route finding and is
        // absolutely not finding all routes at all because it doesn't
        // search exhaustively
        let mut current_room = Point { x: 0, y: 0 };
        let mut current_path = Vec::new();

        // collection of (room, path taken to that room, door)
        let mut doors_to_follow = Vec::new();
        let mut iteration = 0;

        loop {
            iteration += 1;
            if iteration % 100 == 0 {
                println!("Iteration {} and I have {} doors to follow",
                         iteration,
                         doors_to_follow.len());
            }

            // println!("I am at {:?} and I came by path {:?}",
            //          current_room,
            //          current_path);
            let doors_here = self.get_doors_for(current_room, current_path.clone())
                .into_iter()
                .filter(|d| !self.is_wall(current_room, *d))
                .collect::<Vec<_>>();

            // println!("These are the doors here: {:?}", doors_here);

            for door in doors_here {
                let mut new_path = current_path.clone();
                new_path.push(door);
                let new_room = current_room.go(door);

                if !self.have_visited(new_room, &new_path) {
                    let to_add = (current_room, current_path.clone(), door);
                    // println!("Adding to my search list {:?}", to_add);
                    doors_to_follow.push(to_add);
                }
            }

            let next_room = doors_to_follow.pop();
            match next_room {
                None => break, // we're done!
                Some((room, path, door)) => {
                    // go to that room
                    current_room = room.go(door);
                    // go through that door
                    current_path = path;
                    current_path.push(door);
                }
            }
        }
    }

    fn is_wall(&self, room: Point, dir: Dir) -> bool {
        match dir {
            Dir::Left => room.x == 0,
            Dir::Right => room.x >= self.width - 1,
            Dir::Up => room.y == 0,
            Dir::Down => room.y >= self.height - 1,
        }
    }

    fn get_routes_for_destination(&self) -> Vec<Vec<Dir>> {
        let room = self.calculated.get(&self.destination);

        match room {
            None => Vec::new(),
            Some(r) => r.keys().cloned().collect(),
        }
    }

    fn find_longest_route(&self, pos: Point, path: &Vec<Dir>, steps: usize) -> usize {
        // based on the nicely elegant C solution by GitHub user rhardih
        let doors = open_doors_here(&self.passcode, path);
        let mut longest = 0;

        let can_up = doors.contains(&Dir::Up) && !self.is_wall(pos.clone(), Dir::Up);
        let can_down = doors.contains(&Dir::Down) && !self.is_wall(pos.clone(), Dir::Down);
        let can_left = doors.contains(&Dir::Left) && !self.is_wall(pos.clone(), Dir::Left);
        let can_right = doors.contains(&Dir::Right) && !self.is_wall(pos.clone(), Dir::Right);

        // can only go down and we're above the destination
        if pos.x == self.destination.x && pos.y == self.destination.y - 1 && can_down &&
           !can_up && !can_left && !can_right {
            return steps + 1;
        }

        // can only go right and we're left of the destination
        if pos.x == self.destination.x - 1 && pos.y == self.destination.y && can_right &&
           !can_up && !can_left && !can_down {
            return steps + 1;
        }

        // more generally
        for &(can, dir) in [(can_up, Dir::Up),
                            (can_down && pos.go(Dir::Down) != self.destination, Dir::Down),
                            (can_left, Dir::Left),
                            (can_right && pos.go(Dir::Right) != self.destination, Dir::Right)]
            .into_iter() {
            if can {
                let mut try_route = path.clone();
                try_route.push(dir);
                let r = self.find_longest_route(pos.go(dir), &try_route, steps + 1);
                if r > longest {
                    longest = r;
                }
            }
        }

        if longest > 0 {
            return longest;
        }

        if pos.go(Dir::Down) == self.destination && can_down {
            return steps + 1;
        } else if pos.go(Dir::Right) == self.destination && can_right {
            return steps + 1;
        }
        // or there is no path at all
        return 0;
    }
}

fn shortest_vec(vecs: &Vec<Vec<Dir>>) -> Option<Vec<Dir>> {
    let mut shortest_so_far = None;
    let mut shortest_size = usize::max_value();
    for v in vecs {
        if v.len() < shortest_size {
            shortest_so_far = Some(v);
            shortest_size = v.len();
        }
    }

    shortest_so_far.map(|v| v.clone())
}

fn find_shortest_route_with_key(key: &str) -> String {
    let mut maze = Maze::new(key.to_owned());
    maze.follow_all_routes();

    let routes = maze.get_routes_for_destination();
    let shortest = shortest_vec(&routes);

    format_route(&shortest)
}

fn find_longest_route_length_with_key(key: &str) -> usize {
    let maze = Maze::new(key.to_owned());
    maze.find_longest_route(Point { x: 0, y: 0 }, &Vec::new(), 0)
}

fn format_route(route: &Option<Vec<Dir>>) -> String {
    route.clone()
        .map(|p| p.into_iter().map(|d| d.as_char()).collect::<String>())
        .unwrap_or("No route found".to_owned())
}

pub fn do_day17() {
    let key = "gdjjyniy";

    let shortest = find_shortest_route_with_key(key);

    println!("Shortest: {}", shortest);

    println!("Longest: {}", find_longest_route_length_with_key(key));
}

#[test]
fn test_it_1_shortest() {
    let shortest = find_shortest_route_with_key("ihgpwlah");

    assert_eq!(shortest, "DDRRRD".to_owned());
    // assert_eq!(longest.len(), 370);
}

#[test]
fn test_it_1_longest() {
    let longest = find_longest_route_length_with_key("ihgpwlah");
    assert_eq!(longest, 370);
}

#[test]
fn test_it_2_shortest() {
    let shortest = find_shortest_route_with_key("kglvqrro");

    assert_eq!(shortest, "DDUDRLRRUDRD".to_owned());
    // assert_eq!(longest.len(), 492);
}

#[test]
fn test_it_2_longest() {
    assert_eq!(find_longest_route_length_with_key("kglvqrro"), 492);
}

#[test]
fn test_it_3_shortest() {
    let shortest = find_shortest_route_with_key("ulqzkmiv");

    assert_eq!(shortest, "DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_owned());
}

#[test]
fn test_it_3_longest() {
    assert_eq!(find_longest_route_length_with_key("ulqzkmiv"), 830);
}

#[test]
fn test_open_doors_here() {
    assert_eq!(open_doors_here("hijkl", &vec![]),
               vec![Dir::Up, Dir::Down, Dir::Left]);

    assert_eq!(open_doors_here("hijkl", &vec![Dir::Down]),
               vec![Dir::Up, Dir::Left, Dir::Right]);
}

#[test]
fn test_get_doors_for() {
    let mut maze = Maze::new("hijkl".to_owned());

    assert_eq!(maze.get_doors_for(Point { x: 0, y: 0 }, vec![]),
               vec![Dir::Up, Dir::Down, Dir::Left]);

    // check it cached it
    assert_eq!(*maze.calculated.get(&Point { x: 0, y: 0 }).unwrap().get(&vec![]).unwrap(),
               vec![Dir::Up, Dir::Down, Dir::Left]);
}
