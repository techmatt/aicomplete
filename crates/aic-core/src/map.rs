//! The board: one country piece per seat, placed by the layout for that
//! number of countries. Geometry is derived from content + config and is not
//! part of the game state; the state holds only what sits on each hex.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::content::{Content, HexKind};
use crate::hex::Hex;

/// The most hexes a board may have: a threat's reach is a `u128` bit mask
/// (`threats::near`). Content maps are at most 64.
pub const MAX_HEXES: usize = 128;

#[derive(Debug, Clone, Serialize)]
pub struct Map {
    /// All hexes, country by country, each country in piece order
    /// (`i * size + j` for piece position `(i, j)`). A hex's index here is
    /// its index in `GameState::hexes`.
    pub hexes: Vec<Cell>,
    pub countries: Vec<CountryGeo>,
    #[serde(skip)]
    index: BTreeMap<Hex, usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Cell {
    pub hex: Hex,
    pub country: u8,
    /// Position on its piece: steps along the piece's first and second axis.
    pub piece_pos: [u8; 2],
    /// Indices of on-board neighbors, in `Dir::ALL` order.
    pub neighbors: Vec<usize>,
    /// The printed hex number on its piece (1-16), for tie-breaks.
    pub number: u8,
    pub kind: HexKind,
    /// A printed industrial zone: where cities build factories.
    pub industrial: bool,
    /// 137: a printed empty station site, where content may create a neutral
    /// research station (none does yet).
    pub station_site: bool,
    /// A coastal city (027): next to ocean, or printed so on the piece.
    pub coastal: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CountryGeo {
    /// Hex indices of this country, in piece order.
    pub hexes: Vec<usize>,
    pub color: String,
}

impl Map {
    pub fn build(content: &Content, countries: u8) -> Result<Map, String> {
        let layout = content
            .layout(countries)
            .ok_or_else(|| format!("no map layout for {countries} countries (content/map.json)"))?;
        let size = content.map.piece.size as i32;
        let mut hexes = Vec::new();
        let mut index = BTreeMap::new();
        let mut geo = Vec::new();
        for (c, p) in layout.pieces.iter().enumerate() {
            let design = &content.map.countries[c];
            let (a, b) = (p.axes[0].offset(), p.axes[1].offset());
            let mut mine = Vec::new();
            for i in 0..size {
                for j in 0..size {
                    let hex = p.anchor.add(a.scale(i)).add(b.scale(j));
                    if index.insert(hex, hexes.len()).is_some() {
                        return Err(format!(
                            "layout for {countries} countries: pieces overlap at {hex:?}"
                        ));
                    }
                    mine.push(hexes.len());
                    let number = (i * size + j + 1) as u8;
                    hexes.push(Cell {
                        hex,
                        country: c as u8,
                        piece_pos: [i as u8, j as u8],
                        neighbors: Vec::new(),
                        number,
                        kind: design.kind_at(number),
                        industrial: design.industry.contains(&number),
                        station_site: design.station_sites.contains(&number),
                        coastal: design.coast.contains(&number),
                    });
                }
            }
            geo.push(CountryGeo {
                hexes: mine,
                color: p.color.clone(),
            });
        }
        if hexes.len() > MAX_HEXES {
            return Err(format!(
                "layout for {countries} countries: {} hexes, over the {MAX_HEXES} a threat's reach mask holds",
                hexes.len()
            ));
        }
        for cell in hexes.iter_mut() {
            cell.neighbors = cell
                .hex
                .neighbors()
                .iter()
                .filter_map(|n| index.get(n).copied())
                .collect();
        }
        for i in 0..hexes.len() {
            let near_ocean = hexes[i].neighbors.iter().any(|&n| hexes[n].kind == HexKind::Ocean);
            if hexes[i].kind == HexKind::City && near_ocean {
                hexes[i].coastal = true;
            }
        }
        Ok(Map {
            hexes,
            countries: geo,
            index,
        })
    }

    pub fn find(&self, hex: Hex) -> Option<usize> {
        self.index.get(&hex).copied()
    }

    /// Hex index of printed number `n` in country `c`.
    pub fn numbered(&self, c: usize, n: u8) -> usize {
        self.countries[c].hexes[n as usize - 1]
    }

    /// Whether two hexes are adjacent (neighbors on the board).
    pub fn adjacent(&self, a: usize, b: usize) -> bool {
        self.hexes[a].neighbors.contains(&b)
    }

    /// Hex distance between two hexes.
    pub fn distance(&self, a: usize, b: usize) -> i32 {
        self.hexes[a].hex.distance(self.hexes[b].hex)
    }
}
