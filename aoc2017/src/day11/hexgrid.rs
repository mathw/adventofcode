use std::ops::Add;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct HexCoordinate {
    q: isize,
    r: isize,
}

impl HexCoordinate {
    pub fn new(q: isize, r: isize) -> HexCoordinate {
        HexCoordinate { q: q, r: r }
    }

    pub fn q(&self) -> isize {
        self.q
    }

    pub fn r(&self) -> isize {
        self.r
    }

    pub fn s(&self) -> isize {
        -self.q - self.r
    }

    pub fn surrounds(&self) -> Vec<HexCoordinate> {
        SURROUND_VECTORS.iter().map(|v| self + v).collect()
    }
}

impl Add for HexCoordinate {
    type Output = HexCoordinate;

    fn add(self, other: HexCoordinate) -> HexCoordinate {
        HexCoordinate::new(self.q + other.q, self.r + other.r)
    }
}

impl<'a> Add for &'a HexCoordinate {
    type Output = HexCoordinate;

    fn add(self, other: &'a HexCoordinate) -> HexCoordinate {
        HexCoordinate::new(self.q + other.q, self.r + other.r)
    }
}

impl<'a> Add<&'a HexCoordinate> for HexCoordinate {
    type Output = HexCoordinate;

    fn add(self, other: &'a HexCoordinate) -> HexCoordinate {
        HexCoordinate::new(self.q + other.q, self.r + other.r)
    }
}

impl<'a> Add<HexCoordinate> for &'a HexCoordinate {
    type Output = HexCoordinate;

    fn add(self, other: HexCoordinate) -> HexCoordinate {
        HexCoordinate::new(self.q + other.q, self.r + other.r)
    }
}

static SURROUND_VECTORS: &'static Vec<HexCoordinate> = &vec![HexCoordinate::new(1, 0),
                                                             HexCoordinate::new(1, -1),
                                                             HexCoordinate::new(0, -1),
                                                             HexCoordinate::new(-1, 0),
                                                             HexCoordinate::new(-1, 1),
                                                             HexCoordinate::new(0, 1)];