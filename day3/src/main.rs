
use std::fmt;
use std::fmt::{Formatter, write};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: i64,
    y: i64,
}

struct Map {
    size: Vec2,
    tiles: Vec<Tile>,
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Tree,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Open
    }
}

// Debug implementation that writes out a graphical implementation of the tile
impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::Open => '.',
            Tile::Tree => '#',
        };
        write!(f, "{}", c)
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..self.size.y {
            for col in 0..self.size.x {
                write!(f, "{:?}", self.get((col, row).into()))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn new(size: Vec2) -> Self {
        let num_tiles = size.x * size.y;
        Self {
            size,
            tiles: (0..num_tiles)
                .into_iter()
                .map(|_| Default::default())
                .collect(),
        }
    }

    // assume every tile outside the map is immutable
    fn set(&mut self, pos: Vec2, tile: Tile) {
        if let Some(index) = self.index(pos) {
            self.tiles[index] = tile
        }
    }

    fn get(&self, pos: Vec2) -> Tile {
        self.index(pos).map(|i| self.tiles[i]).unwrap_or_default()
    }

    // a helper function to extend our map forever to the right as well as to left
    // it returns None for positions outside of the map
    fn normalize_pos(&self, pos: Vec2) -> Option<Vec2> {
        if pos.y < 0 || pos.y >= self.size.y {
            None
        } else {
            let x = pos.x % self.size.x;

            let x = if x < 0 {
                // wrap around for positions to the left of 0
                self.size.x + x
            } else { x };
            Some((x, pos.y).into())
        }
    }

    // a helper function to return index of a tile in our flat storage
    // it will return None for positions that do not exist on map
    fn index(&self, pos: Vec2) -> Option<usize> {
        self.normalize_pos(pos)
            .map(|pos| (pos.x + pos.y * self.size.x) as _)
    }

    fn parse(input: &[u8]) -> Self {
        let mut columns = 0;
        let mut rows = 1;
        for &c in input.iter() {
            if c == b'\n' {
                rows += 1;
                columns = 0;
            } else {
                columns += 1;
            }
        }

        let mut iter = input.iter().copied();
        let mut map = Self::new((columns, rows).into());
        for row in 0..map.size.y {
            for col in 0..map.size.x {
                let tile = match iter.next() {
                    Some(b'.') => Tile::Open,
                    Some(b'#') => Tile::Tree,
                    c => panic!("Expected '.' or '#', but got {:?}", c),
                };
                map.set((col, row).into(), tile);
            }
            iter.next();
        }
        map
    }
}


// Build vec2 from a tuple
impl From<(i64, i64)> for Vec2 {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

fn main() {
    let map = Map::parse(include_bytes!("./input.txt"));
    dbg!(map.size);
    let itinerary = (0..map.size.y).into_iter()
        .map(|y| Vec2::from((y*3, y)));
    let num_trees = itinerary
        .filter(|&pos| map.get(pos) == Tile::Tree).count();
    println!("Trees: {}", num_trees);
}


#[cfg(test)]
mod tests {
    use super::{Vec2, Map};

    #[test]
    fn test_tuple() {
        let v: Vec2 = (5, 8).into();
        assert_eq!(v.x, 5);
        assert_eq!(v.y, 8);
    }

    #[test]
    fn test_normalize_pos() {
        let m = Map::new((2, 2).into());
        assert_eq!(m.normalize_pos((0, 0).into()), Some((0, 0).into()));
        assert_eq!(m.normalize_pos((1, 0).into()), Some((1, 0).into()));
        assert_eq!(m.normalize_pos((2, 0).into()), Some((0, 0).into()));
        assert_eq!(m.normalize_pos((-1, 0).into()), Some((1, 0).into()));
        assert_eq!(m.normalize_pos((-2, 0).into()), Some((0, 0).into()));
        assert_eq!(m.normalize_pos((0, -1).into()), None);
        assert_eq!(m.normalize_pos((0, 2).into()), None);
    }

    #[test]
    fn test_index() {
        let m = Map::new((3, 5).into());
        assert_eq!(m.index((0, 0).into()), Some(0));
        assert_eq!(m.index((2, 0).into()), Some(2));
        assert_eq!(m.index((0, 1).into()), Some(3));
        assert_eq!(m.index((2, 1).into()), Some(5));
    }
}