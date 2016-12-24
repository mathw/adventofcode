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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Shortest,
    Longest,
}

#[derive(Debug)]
struct Maze {
    width: usize,
    height: usize,
    destination: Point,
    calculated: HashMap<Point, HashMap<Vec<Dir>, Vec<Dir>>>,
    passcode: String,
    mode: Mode,
}

impl Maze {
    fn new(passcode: String, mode: Mode) -> Maze {
        Maze {
            width: 4,
            height: 4,
            destination: Point { x: 3, y: 3 },
            calculated: HashMap::new(),
            passcode: passcode,
            mode: mode,
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

fn longest_vec(vecs: &Vec<Vec<Dir>>) -> Option<Vec<Dir>> {
    let mut longest_so_far = None;
    let mut longest_size = 0;
    for v in vecs {
        if v.len() > longest_size {
            longest_so_far = Some(v);
            longest_size = v.len();
        }
    }

    longest_so_far.map(|v| v.clone())
}

// fn normalise_path(path: &Vec<Dir>) -> Vec<Dir> {
//     if path.len() == 0 {
//         return Vec::new();
//     }
//
//     if path.len() == 1 {
//         return path.clone();
//     }
//
//     let mut normalised = Vec::new();
//     let mut i = 0;
//
//     loop {
//         if i > path.len() - 2 {
//             // can't do a pair comp as there's no pair Left
//             normalised.push(path[i]);
//             break;
//         }
//
//         match (path[i], path[i + 1]) {
//             (Dir::Down, Dir::Up) |
//             (Dir::Up, Dir::Down) |
//             (Dir::Left, Dir::Right) |
//             (Dir::Right, Dir::Left) => {
//                 // totally useless movement, skip it!
//                 i += 2;
//             }
//             _ => {
//                 // valid movement
//                 normalised.push(path[i]);
//                 i += 1;
//             }
//         }
//
//         if i > path.len() - 1 {
//             break;
//         }
//     }
//     normalised
// }

fn find_shortest_route_with_key(key: &str) -> String {
    let mut maze = Maze::new(key.to_owned(), Mode::Shortest);
    maze.follow_all_routes();

    let routes = maze.get_routes_for_destination();
    let shortest = shortest_vec(&routes);

    format_route(&shortest)
}

fn find_longest_route_length_with_key(key: &str) -> usize {
    let mut maze = Maze::new(key.to_owned(), Mode::Longest);
    maze.follow_all_routes();

    let routes = maze.get_routes_for_destination();
    let longest = longest_vec(&routes);

    match longest {
        lo @ Some(_) => format_route(&lo).len(),
        None => 0,
    }
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
fn test_it_3_shortest() {
    let shortest = find_shortest_route_with_key("ulqzkmiv");

    assert_eq!(shortest, "DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_owned());
    // assert_eq!(longest.len(), 830);
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
    let mut maze = Maze::new("hijkl".to_owned(), Mode::Shortest);

    assert_eq!(maze.get_doors_for(Point { x: 0, y: 0 }, vec![]),
               vec![Dir::Up, Dir::Down, Dir::Left]);

    // check it cached it
    assert_eq!(*maze.calculated.get(&Point { x: 0, y: 0 }).unwrap().get(&vec![]).unwrap(),
               vec![Dir::Up, Dir::Down, Dir::Left]);
}

// #[test]
// fn test_normalise_path() {
//     let path = vec![Dir::Down];
//     assert_eq!(path, normalise_path(&path));
//
//     let path = vec![Dir::Down, Dir::Up, Dir::Down];
//     assert_eq!(normalise_path(&path), vec![Dir::Down]);
// }
