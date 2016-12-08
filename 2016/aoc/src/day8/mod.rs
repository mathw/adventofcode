use regex::Regex;

struct Display {
    width: usize,
    height: usize,
    pixels: Vec<bool>,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Display {
        Display {
            width: width,
            height: height,
            pixels: vec![false; width * height],
        }
    }

    pub fn rect(&mut self, width: usize, height: usize) {
        for line in 0..height {
            for pixel in 0..width {
                self.pixels[(self.width * line) + pixel] = true;
            }
        }
    }

    pub fn rotate_column(&mut self, x: usize, by: usize) {
        fn to_pixel(x: usize, i: usize, width: usize) -> usize {
            x + (width * i)
        }

        let mut column = Vec::new();
        for i in 0..self.height {
            column.push(self.pixels[to_pixel(x, i, self.width)]);
        }

        // now put the column back, in a different place
        for i in 0..self.height {
            self.pixels[to_pixel(x, (i + by) % self.height, self.width)] = column[i];
        }
    }

    pub fn rotate_row(&mut self, y: usize, by: usize) {
        fn to_pixel(y: usize, i: usize, width: usize) -> usize {
            (y * width) + i
        }

        let mut row = Vec::new();
        for i in 0..self.width {
            row.push(self.pixels[to_pixel(y, i, self.width)]);
        }

        for i in 0..self.width {
            self.pixels[to_pixel(y, (i + by) % self.width, self.width)] = row[i];
        }
    }

    pub fn render(&self) -> String {
        let mut lines = String::from("");

        for l in 0..self.height {
            let mut line = String::from("");
            for pixel in 0..self.width {
                line.push(if self.pixels[(self.width * l) + pixel] {
                    '#'
                } else {
                    ' '
                });
            }

            lines.push_str(&line);
            lines.push('\n');
        }

        lines
    }

    pub fn pixels_lit(&self) -> usize {
        self.pixels.iter().map(|&p| if p { 1 } else { 0 }).sum()
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
    Rect { width: usize, height: usize },
    Row { y: usize, by: usize },
    Column { x: usize, by: usize },
}

fn interpret(instruction: &Instruction, display: &mut Display) {
    match instruction {
        &Instruction::Rect { width, height } => {
            display.rect(width, height);
        }
        &Instruction::Row { y, by } => {
            display.rotate_row(y, by);
        }
        &Instruction::Column { x, by } => {
            display.rotate_column(x, by);
        }
    }
}

fn parse_line(line: &str) -> Option<Instruction> {
    lazy_static! {
        static ref RECT_RE: Regex = Regex::new(r"rect (\d+)x(\d+)").unwrap();
        static ref COL_RE: Regex = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
        static ref ROW_RE: Regex = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
    }

    if RECT_RE.is_match(line) {
        let cap = RECT_RE.captures_iter(line).next().unwrap();
        let x = cap.at(1).unwrap().parse::<usize>().unwrap();
        let y = cap.at(2).unwrap().parse::<usize>().unwrap();
        Some(Instruction::Rect {
            width: x,
            height: y,
        })
    } else if COL_RE.is_match(line) {
        let cap = COL_RE.captures_iter(line).next().unwrap();
        let x = cap.at(1).unwrap().parse::<usize>().unwrap();
        let by = cap.at(2).unwrap().parse::<usize>().unwrap();
        Some(Instruction::Column { x: x, by: by })
    } else if ROW_RE.is_match(line) {
        let cap = ROW_RE.captures_iter(line).next().unwrap();
        let y = cap.at(1).unwrap().parse::<usize>().unwrap();
        let by = cap.at(2).unwrap().parse::<usize>().unwrap();
        Some(Instruction::Row { y: y, by: by })
    } else {
        None
    }
}

pub fn do_day8(input: &str) {
    let mut display = Display::new(50, 6);

    for inst in input.lines().filter_map(|l| parse_line(l)) {
        interpret(&inst, &mut display);
    }

    println!("{} pixels lit", display.pixels_lit());


    println!("{}", display.render());
}


#[test]
fn test_rect() {
    let mut disp = Display::new(4, 4);
    disp.rect(2, 3);

    assert_eq!(disp.render(), "##  \n##  \n##  \n    \n");
}

#[test]
fn test_rotate_column() {
    let mut disp = Display::new(4, 4);
    disp.rect(2, 3);
    disp.rotate_column(1, 2);

    assert_eq!(disp.render(), "##  \n#   \n##  \n #  \n");
}

#[test]
fn test_rotate_row() {
    let mut disp = Display::new(4, 4);
    disp.rect(2, 3);
    disp.rotate_row(1, 2);

    assert_eq!(disp.render(), "##  \n  ##\n##  \n    \n");
}

#[test]
fn test_pixels_lit() {
    let mut disp = Display::new(4, 4);
    disp.rect(2, 3);

    assert_eq!(disp.pixels_lit(), 6);
}

#[test]
fn test_parse_line() {
    assert_eq!(parse_line("rect 3x2"),
               Some(Instruction::Rect {
                   width: 3,
                   height: 2,
               }));
    assert_eq!(parse_line("rotate column x=1 by 2"),
               Some(Instruction::Column { x: 1, by: 2 }));
    assert_eq!(parse_line("rotate row y=0 by 4"),
               Some(Instruction::Row { y: 0, by: 4 }));
    assert_eq!(parse_line("bite my shiny metal ass"), None);
}
