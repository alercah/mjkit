use crate::tile::{Direction, Tile};
use failure::Error;

mod yaku;

/// The seating positions of a player's opponents.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum Opponent {
    Right,
    Across,
    Left,
}

/// A location from which a tile can come, when used in a call or a win.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Location {
    /// The live wall; in other words, a player's normal draw for their turn.
    LiveWall,
    /// The dead wall, drawn from after a kan.
    DeadWall,
    /// Another player's discard.
    Discard(Opponent),
    /// A tile used in a kan by another player.
    Kan(Opponent),
}

impl Location {
    /// Returns `true` if the location corresponds to a tile drawn by the (potentially) winning
    /// player.
    pub fn is_drawn(self) -> bool {
        use Location::*;
        self == LiveWall || self == DeadWall
    }

    /// Returns the opponent, if any, to which the location corresponds.
    pub fn opponent(self) -> Option<Opponent> {
        use Location::*;
        match self {
            Discard(o) | Kan(o) => Some(o),
            _ => None,
        }
    }
}

/// The circumstances surrounding a winning hand. This includes all the information required to
/// determine if a win is legal and to score a hand, other than its contents.
#[derive(Debug, Clone)]
pub struct WinContext {
    /// The winning tile.
    pub agari: Tile,
    /// The location of the winning tile.
    pub source: Location,
    /// Whether the winner had declared riichi.
    pub riichi: bool,
    /// Whether the win occurred on the player's first turn after the start of a hand or after a
    /// riichi. Note that this field is slightly overloaded as a result, however, a player declaring
    /// riichi necessarily means that they have had a turn, so there is no ambiguity. If a call is
    /// successfully made by any player after the start of the hand or the riichi, then it is no
    /// longer considered to be the first turn.
    pub first_turn: bool,
    /// Whether the live wall is empty. Whenthe location is a drawn tile, this means that the live
    /// wall was empty after the player's draw.
    pub wall_empty: bool,
    /// The current round wind.
    pub round: Direction,
    /// The player's seat wind. The player is the dealer if this is [East][Direction::East].
    pub seat: Direction,
    /// The number of honba currently on the table.
    pub honba: u8,
}

/// The kinds of group which can be formed.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum GroupType {
    Sequence,
    Triplet,
    Quad,
}

/// The kinds of wait that a normal hand can have.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum Wait {
    Ryanmen,
    Kanchan,
    Penchan,
    Shanpon,
    Tanki,
}

/// A group of tiles in a player's hand. It can represent both a called group and a closed group
/// that the player completed on their own.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group {
    // The tiles of the group.
    tiles: Vec<Tile>,
    // The player off of whom a tile was called. If set, the tile called will be last in the group,
    // except possibly for an added tile.
    off: Option<Opponent>,
    // Whether this group was created by adding a tile to another group. If set, the added tile will
    // be last in the group.
    added: bool,
    // Whether this group contains the tile used to win the hand. In this case, the winning tile
    // will be last in the group (including when it is called). A group cannot be both the agari and
    // an added group at the same time.
    agari: bool,
}

impl Group {
    /// Returns `true` if any tiles for this group came from another player.
    pub fn is_open(&self) -> bool {
        self.off.is_none()
    }

    /// Returns `true` if the group contains the winning tile.
    pub fn has_agari(&self) -> bool {
        self.agari
    }

    /// Returns the type of the group.
    pub fn ty(&self) -> GroupType {
        if self.tiles.len() == 4 {
            GroupType::Quad
        } else if self.tiles[0] == self.tiles[1] {
            GroupType::Triplet
        } else {
            GroupType::Sequence
        }
    }

    /// Returns all the tiles of the group.
    pub fn tiles(&self) -> &[Tile] {
        &*self.tiles
    }

    /// Returns either a single tile in a triplet/quad, or the first tile in a sequence.
    pub fn first_tile(&self) -> Tile {
        if self.ty() == GroupType::Sequence {
            *self.tiles.iter().min().unwrap()
        } else {
            self.tiles[0]
        }
    }

    /// Returns the wait shape of the group, if it is an agari group.
    pub fn wait(&self) -> Option<Wait> {
        if !self.agari {
            None
        } else if self.ty() != GroupType::Sequence {
            Some(Wait::Shanpon)
        } else {
            let agari = self.tiles[2];
            let val = agari.val().unwrap().val();
            let mut sorted = self.tiles.clone();
            sorted.sort();
            let pos = sorted.iter().position(|&t| t == agari).unwrap();
            if pos == 2 {
                Some(Wait::Kanchan)
            } else if (pos == 1 && val == 7) || (pos == 3 && val == 3) {
                Some(Wait::Penchan)
            } else {
                Some(Wait::Ryanmen)
            }
        }
    }
}

/// A hand of tiles held by a player during the game, including any called groups.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    loose: Vec<Tile>,
    groups: Vec<Group>,
}

impl Hand {}

/// A complete hand of fourteen tiles, arranged into one of the shapes that make a winning hand.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompleteHand {
    /// Kokushimusō.
    Kokushi([Tile; 14]),
    /// Chītoitsu.
    SevenPairs([[Tile; 2]; 7]),
    /// A normal hand of four groups and a pair. At most one group has the agari bit set.
    Standard([Group; 4], [Tile; 2]),
}
