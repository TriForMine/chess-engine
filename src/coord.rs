#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

impl Coord {
    pub fn new(x: u8, y: u8) -> Coord {
        Coord { x, y }
    }

    pub fn from_index(index: u8) -> Coord {
        Coord {
            x: index % 8,
            y: index / 8,
        }
    }

    pub fn from_str(s: &str) -> Option<Coord> {
        if s.len() != 2 {
            return None;
        }

        let x = s.chars().nth(0).unwrap() as u8 - 97;
        let y = s.chars().nth(1).unwrap() as u8 - 49;

        Some(Coord::new(x, y))
    }

    pub fn to_str(&self) -> String {
        // Should be wither letter and number
        // We can use the ASCII table to convert

        let mut s = String::new();

        s.push((self.x + 97) as char);
        s.push((self.y + 49) as char);

        s
    }

    #[inline(always)]
    pub fn to_index(&self) -> u8 {
        self.x + self.y * 8
    }

    #[inline(always)]
    pub fn in_bounds(&self) -> bool {
        self.x < 8 && self.y < 8
    }
}
