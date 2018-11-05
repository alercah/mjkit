/// The three suits of numbered tiles.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum Suit {
    Manzu,
    Souzu,
    Pinzu,
}
use Suit::*;

/// The four cardinal directions, used for seating and for wind tiles.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum Direction {
    East,
    South,
    West,
    North,
}
use Direction::*;

impl Direction {
    /// Get the next direction around the table, looping around. In Mahjong, the winds are arranged
    /// following the traditional ordering, even though the counterclockwise play means that they
    /// are mirrored from cardinal directions.
    pub fn next(self) -> Self {
        match self {
            East => South,
            South => West,
            West => North,
            North => East,
        }
    }
}

/// The three colours of dragon tiles.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum Dragon {
    White,
    Green,
    Red,
}
use Dragon::*;

impl Dragon {
    /// Get the next dragon tile in order, looping around. This ordering is mostly used only for
    /// dora indication.
    pub fn next(self) -> Dragon {
        match self {
            White => Green,
            Green => Red,
            Red => White,
        }
    }
}

/// The value of a suited tile. It is a separate type so that it can be enforced that a tile always
/// corresponds to a real tile value.
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Val(u8);

impl Val {
    /// Construct a value. The argument must be between 1 and 9, inclusive.
    pub fn new(n: u8) -> Val {
        if 1 <= n && n <= 9 {
            Val(n)
        } else {
            panic!("tile value must be between 1 and 9 inclusive, got {}", n)
        }
    }

    /// Returns the numeric value.
    pub fn val(self) -> u8 {
        self.0
    }
}

impl From<Val> for u8 {
    fn from(v: Val) -> u8 {
        v.val()
    }
}

/// A single mahjong tile. This accounts only for tile values, not for additional properties that a
/// tile may have, such as being red dora or haku pocchi.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum Tile {
    Suited(Suit, Val),
    Wind(Direction),
    Dragon(Dragon),
}
use Tile::*;

impl Tile {
    /// Returns the tile that this tile indicates as dora. That is, if `self` is a dora indicator,
    /// `indicated_dora` is the dora tile.
    pub fn indicated_dora(self) -> Tile {
        match self {
            Suited(s, Val(9)) => Suited(s, Val(1)),
            Suited(s, Val(n)) => Suited(s, Val(n + 1)),
            Wind(w) => Wind(w.next()),
            Dragon(d) => Dragon(d.next()),
        }
    }

    /// Returns the value of a suited tile.
    pub fn val(self) -> Option<Val> {
        match self {
            Suited(_, v) => Some(v),
            _ => None,
        }
    }

    /// Returns `true` if `self` follows `t` in sequence,
    pub fn follows(self, t: Tile) -> bool {
        Some(self) == t.following()
    }

    /// Returns the next tile in sequence. Sequence, in this cases, refers only to shuntsu, not to
    /// dora.
    pub fn following(self) -> Option<Tile> {
        match self {
            Suited(_, Val(9)) => None,
            Suited(..) => Some(self.indicated_dora()),
            _ => None,
        }
    }

    /// Returns `true` if this tile is yakuhai, given the relevant round and
    /// seat winds.
    pub fn is_yakuhai(self, round: Direction, seat: Direction) -> bool {
        match self {
            Dragon(_) => true,
            Wind(w) => w == round || w == seat,
            _ => false,
        }
    }

    /// Returns `true` if this tile is green, qualifying it for ryūiisō.
    pub fn is_green(self) -> bool {
        match self {
            Dragon(Green) => true,
            Suited(Souzu, Val(n)) => [2, 3, 4, 6, 8].contains(&n),
            _ => false,
        }
    }

    /// Returns `true` if this is an honour tile: a wind or a dragon.
    pub fn is_honour(self) -> bool {
        match self {
            Dragon(_) | Wind(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if this is a terminal tile: a suited 1 or 9.
    pub fn is_terminal(self) -> bool {
        match self {
            Suited(_, Val(1)) | Suited(_, Val(9)) => true,
            _ => false,
        }
    }

    /// Returns an iterator over all the different tiles, in sorted order.
    pub fn all() -> impl Iterator<Item = Tile> {
        AllTiles {
            next: Some(Suited(Manzu, Val(1))),
        }
    }
}

struct AllTiles {
    next: Option<Tile>,
}

impl Iterator for AllTiles {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.next;
        if let Some(val) = cur {
            self.next = match val {
                Suited(Manzu, Val(9)) => Some(Suited(Souzu, Val(1))),
                Suited(Souzu, Val(9)) => Some(Suited(Pinzu, Val(9))),
                Suited(Pinzu, Val(9)) => Some(Wind(East)),
                Wind(North) => Some(Dragon(White)),
                Dragon(Red) => None,
                _ => Some(val.indicated_dora()),
            };
        }
        cur
    }
}
