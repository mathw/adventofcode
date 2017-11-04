use std::str::FromStr;

struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn smallest_sides(&self) -> (u32, u32) {
        let mut ordered = [self.l, self.w, self.h];
        ordered.sort();
        let mut ordered_iter = ordered.iter();
        let smallest = ordered_iter.next().unwrap();
        let next_smallest = ordered_iter.next().unwrap();

        (*smallest, *next_smallest)
    }

    fn smallest_side(&self) -> u32 {
        let (smallest, next_smallest) = self.smallest_sides();
        smallest * next_smallest
    }

    fn required_area(&self) -> u32 {
        (2 * self.w * self.h) + (2 * self.w * self.l) + (2 * self.l * self.h) + self.smallest_side()
    }

    fn smallest_perimeter(&self) -> u32 {
        let (smallest, next_smallest) = self.smallest_sides();
        smallest + smallest + next_smallest + next_smallest
    }

    fn volume(&self) -> u32 {
        self.w * self.l * self.h
    }
}

pub fn run() {
    let presents = parse_input(include_str!("input.txt"));

    println!("{} presents", presents.len());

    let areas: u32 = presents.iter().map(|p| p.required_area()).sum();

    println!("{} sq ft required", areas);

    let ribbon_lengths: u32 = presents.iter().map(|p| p.volume() + p.smallest_perimeter()).sum();

    println!("{} ft of ribbon required", ribbon_lengths);
}

fn parse_input(input: &str) -> Vec<Present> {
    input.lines()
        .map(|line| {
            let mut bits = line.split("x");
            let l = u32::from_str(bits.next().unwrap()).unwrap();
            let w = u32::from_str(bits.next().unwrap()).unwrap();
            let h = u32::from_str(bits.next().unwrap()).unwrap();
            Present { l: l, w: w, h: h }
        })
        .collect()
}

#[test]
fn test_smallest_side() {
    let present = Present { l: 2, w: 3, h: 4 };
    assert_eq!(6, present.smallest_side());

    let present = Present { l: 3, w: 3, h: 3 };
    assert_eq!(9, present.smallest_side());
}
