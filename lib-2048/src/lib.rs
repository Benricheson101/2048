use std::fmt;

#[cfg(not(test))]
use rand::{rngs::OsRng, seq::SliceRandom, Rng};

/// Default dimensions of the [`GameBoard`]
pub const GAME_BOARD_SIZE: usize = 4;
pub const STARTING_TILES: usize = 2;

/// A representation of the location of a space on the game board
pub type GameBoardLocation = (usize, usize);

/// Represents the grid of tiles making up the game
///
/// The `(0,0)` origin of the board is located in the top-left corner of the
/// board, with coordinates increasing as you move toward the bottom-right of
/// the board. Coordinates are in the form (row, column)
#[derive(Debug, Clone)]
pub struct GameBoard {
    pub cells: [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE],
    pub score: usize,
    #[cfg(not(test))]
    rng: OsRng,
}

impl GameBoard {
    /// Creates a new square [`GameBoard`](Self) with dimensions [`GAME_BOARD_SIZE`], prefilled
    /// with two tiles
    pub fn new() -> Self {
        let mut board = Self::empty();

        for _ in 0..STARTING_TILES {
            board.add_random_tile();
        }

        board
    }

    /// Creates a new blank square [`GameBoard`](Self) with dimensions
    /// [`GAME_BOARD_SIZE`]
    ///
    /// ```
    /// use lib_2048::{BoardSpace::*, GameBoard};
    ///
    /// assert_eq!(
    ///     GameBoard::empty().cells,
    ///     [
    ///         [Vacant, Vacant, Vacant, Vacant],
    ///         [Vacant, Vacant, Vacant, Vacant],
    ///         [Vacant, Vacant, Vacant, Vacant],
    ///         [Vacant, Vacant, Vacant, Vacant],
    ///     ]
    /// );
    /// ```
    pub fn empty() -> Self {
        Self {
            cells: [[BoardSpace::Vacant; GAME_BOARD_SIZE]; GAME_BOARD_SIZE],
            score: 0,
            #[cfg(not(test))]
            rng: OsRng {},
        }
    }

    /// Gets the value of a cell on the game board
    pub fn get(&self, (x, y): GameBoardLocation) -> BoardSpace {
        self.cells[y][x]
    }

    /// Sets the value of a cell on the game board
    pub fn set(&mut self, (x, y): GameBoardLocation, val: BoardSpace) {
        self.cells[y][x] = val;
    }

    /// Moves all tiles on the board, merging any adjacent tiles of the same numeric value
    pub fn r#move(&mut self, dir: MoveDirection) {
        let rot = dir as usize;
        self.rotate(rot);

        let mut moved = false;

        for y in 0..self.cells.len() {
            for x in 0..self.cells.len() {
                if let BoardSpace::Tile(t) = self.cells[y][x] {
                    for x2 in (x + 1)..self.cells.len() {
                        match self.cells[y][x2] {
                            BoardSpace::Tile(t2) if t == t2 => {
                                let new_val = t * 2;
                                self.score += new_val;

                                self.cells[y][x] = BoardSpace::Tile(new_val);
                                self.cells[y][x2] = BoardSpace::Vacant;

                                moved = true;
                                break;
                            },

                            BoardSpace::Tile(_) => break,
                            _ => continue,
                        }
                    }
                }
            }

            for x in 0..self.cells.len() {
                if let BoardSpace::Vacant = self.cells[y][x] {
                    for x2 in x..self.cells.len() {
                        if let BoardSpace::Tile(_) = self.cells[y][x2] {
                            self.cells[y].swap(x, x2);

                            moved = true;
                            break;
                        }
                    }
                }
            }
        }

        self.rotate(self.cells.len() - rot);

        if moved {
            self.add_random_tile();
        }
    }

    pub fn has_lost(&self) -> bool {
        !self.can_move()
    }

    // TODO: make this not mutate?
    fn can_move(&self) -> bool {
        // TODO: ugly hack, is there a better way to do this?
        let mut cells = self.cells.clone();

        for n in 0..4 {
            rotate(&mut cells, n);

            for y in 0..cells.len() {
                for x in 0..(cells.len() - 1) {
                    match cells[y][x] {
                        BoardSpace::Vacant => {
                            rotate(&mut cells, 4 - n);
                            return true;
                        },

                        BoardSpace::Tile(t) => match cells[y][x + 1] {
                            BoardSpace::Vacant => {
                                rotate(&mut cells, 4 - n);
                                return true;
                            },

                            BoardSpace::Tile(t2) if t == t2 => {
                                rotate(&mut cells, 4 - n);
                                return true;
                            },

                            _ => continue,
                        },
                    }
                }
            }
        }

        false
    }

    fn rotate(&mut self, times: usize) {
        rotate(&mut self.cells, times);
    }

    fn all_empty_spaces(&self) -> Vec<GameBoardLocation> {
        let mut locations: Vec<GameBoardLocation> = vec![];

        for y in 0..self.cells.len() {
            for x in 0..self.cells.len() {
                if let BoardSpace::Vacant = self.cells[y][x] {
                    locations.push((x, y));
                }
            }
        }

        locations
    }

    #[cfg(test)]
    fn add_random_tile(&mut self) {}

    #[cfg(not(test))]
    fn add_random_tile(&mut self) {
        let free_spaces = self.all_empty_spaces();

        if !free_spaces.is_empty() {
            let tile = if self.rng.gen_bool(1.0 / 10.0) {
                BoardSpace::Tile(4)
            } else {
                BoardSpace::Tile(2)
            };

            let pos = free_spaces
                .choose(&mut self.rng)
                .expect("failed to choose random position");

            self.set(*pos, tile);
        }
    }
}

impl Default for GameBoard {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<[[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE]> for GameBoard {
    fn from(def: [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE]) -> Self {
        Self {
            cells: def,
            ..Default::default()
        }
    }
}

/// The direction of a [`move`](GameBoard::move)
#[derive(Debug)]
pub enum MoveDirection {
    Left = 0,
    Up = 1,
    Right = 2,
    Down = 3,
}

/// A space on the [`GameBoard`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoardSpace {
    /// An empty space
    Vacant,
    /// A tile with a numeric value
    Tile(usize),
}

impl fmt::Display for BoardSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vacant => write!(f, ""),
            Self::Tile(n) => write!(f, "{n}"),
        }
    }
}

fn rotate(
    arrs: &mut [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE],
    times: usize,
) {
    let n = arrs.len();

    // credit: someone on stackoverflow idk
    for _ in 0..times {
        for i in 0..(n / 2) {
            for j in i..(n - i - 1) {
                let tmp = arrs[i][j];
                arrs[i][j] = arrs[j][n - i - 1];
                arrs[j][n - i - 1] = arrs[n - i - 1][n - j - 1];
                arrs[n - i - 1][n - j - 1] = arrs[n - j - 1][i];
                arrs[n - j - 1][i] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BoardSpace::*, *};

    const SAMPLE_GAME_BOARD: [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE] = [
        [Tile(2), Tile(2), Tile(2), Tile(2)],
        [Tile(2), Tile(8), Tile(1), Vacant],
        [Vacant, Vacant, Vacant, Vacant],
        [Tile(2), Tile(4), Tile(1), Tile(2)],
    ];

    #[test]
    fn move_up() {
        let mut board = GameBoard::from(SAMPLE_GAME_BOARD);

        board.r#move(MoveDirection::Up);

        const EXPECTED: [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE] = [
            [Tile(4), Tile(2), Tile(2), Tile(4)],
            [Tile(2), Tile(8), Tile(2), Vacant],
            [Vacant, Tile(4), Vacant, Vacant],
            [Vacant, Vacant, Vacant, Vacant],
        ];

        assert_eq!(board.cells, EXPECTED);
    }

    #[test]
    fn move_down() {
        let mut board = GameBoard::from(SAMPLE_GAME_BOARD);

        board.r#move(MoveDirection::Down);

        const EXPECTED: [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE] = [
            [Vacant, Vacant, Vacant, Vacant],
            [Vacant, Tile(2), Vacant, Vacant],
            [Tile(2), Tile(8), Tile(2), Vacant],
            [Tile(4), Tile(4), Tile(2), Tile(4)],
        ];

        assert_eq!(board.cells, EXPECTED);
    }

    #[test]
    fn move_left() {
        let mut board = GameBoard::from(SAMPLE_GAME_BOARD);

        board.r#move(MoveDirection::Left);

        const EXPECTED: [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE] = [
            [Tile(4), Tile(4), Vacant, Vacant],
            [Tile(2), Tile(8), Tile(1), Vacant],
            [Vacant, Vacant, Vacant, Vacant],
            [Tile(2), Tile(4), Tile(1), Tile(2)],
        ];

        assert_eq!(board.cells, EXPECTED);
    }

    #[test]
    fn move_right() {
        let mut board = GameBoard::from(SAMPLE_GAME_BOARD);

        board.r#move(MoveDirection::Right);

        const EXPECTED: [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE] = [
            [Vacant, Vacant, Tile(4), Tile(4)],
            [Vacant, Tile(2), Tile(8), Tile(1)],
            [Vacant, Vacant, Vacant, Vacant],
            [Tile(2), Tile(4), Tile(1), Tile(2)],
        ];

        assert_eq!(board.cells, EXPECTED);
    }

    #[test]
    fn get() {
        let board = GameBoard::from(SAMPLE_GAME_BOARD);
        let got = board.get((0, 0));
        assert_eq!(got, BoardSpace::Tile(2));
    }

    #[test]
    fn set() {
        let mut board = GameBoard::from(SAMPLE_GAME_BOARD);
        board.set((0, 0), BoardSpace::Tile(2048));

        const EXPECTED: [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE] = [
            [Tile(2048), Tile(2), Tile(2), Tile(2)],
            [Tile(2), Tile(8), Tile(1), Vacant],
            [Vacant, Vacant, Vacant, Vacant],
            [Tile(2), Tile(4), Tile(1), Tile(2)],
        ];

        assert_eq!(board.cells, EXPECTED);
    }

    #[test]
    fn all_empty_spaces() {
        let board = GameBoard::from(SAMPLE_GAME_BOARD);

        let expected = vec![(3, 1), (0, 2), (1, 2), (2, 2), (3, 2)];

        let got = board.all_empty_spaces();
        assert_eq!(got, expected);
    }
}
