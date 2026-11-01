//! Axial hex coordinates (pointy-top).
//!
//! `q` grows to the east, `r` grows to the south-east. The third cube
//! coordinate is `s = -q - r`. Coordinates serialize as `[q, r]`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(from = "[i32; 2]", into = "[i32; 2]")]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}

impl From<[i32; 2]> for Hex {
    fn from([q, r]: [i32; 2]) -> Self {
        Hex { q, r }
    }
}

impl From<Hex> for [i32; 2] {
    fn from(h: Hex) -> Self {
        [h.q, h.r]
    }
}

impl Hex {
    pub const fn new(q: i32, r: i32) -> Self {
        Hex { q, r }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(self, o: Hex) -> Hex {
        Hex::new(self.q + o.q, self.r + o.r)
    }

    pub fn scale(self, k: i32) -> Hex {
        Hex::new(self.q * k, self.r * k)
    }

    pub fn neighbor(self, d: Dir) -> Hex {
        self.add(d.offset())
    }

    /// Neighbors in [`Dir::ALL`] order, so iteration order is fixed.
    pub fn neighbors(self) -> [Hex; 6] {
        Dir::ALL.map(|d| self.neighbor(d))
    }

    pub fn distance(self, o: Hex) -> i32 {
        let (dq, dr) = (self.q - o.q, self.r - o.r);
        (dq.abs() + dr.abs() + (dq + dr).abs()) / 2
    }
}

/// The six directions of a pointy-top hex, counter-clockwise from east as
/// seen on screen (y down).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Dir {
    E,
    NE,
    NW,
    W,
    SW,
    SE,
}

impl Dir {
    pub const ALL: [Dir; 6] = [Dir::E, Dir::NE, Dir::NW, Dir::W, Dir::SW, Dir::SE];

    pub fn offset(self) -> Hex {
        match self {
            Dir::E => Hex::new(1, 0),
            Dir::NE => Hex::new(1, -1),
            Dir::NW => Hex::new(0, -1),
            Dir::W => Hex::new(-1, 0),
            Dir::SW => Hex::new(-1, 1),
            Dir::SE => Hex::new(0, 1),
        }
    }

    pub fn index(self) -> usize {
        Dir::ALL.iter().position(|&d| d == self).unwrap()
    }
}
