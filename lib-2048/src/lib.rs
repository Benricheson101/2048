use std::fmt;

use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

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
#[derive(Debug)]
pub struct GameBoard {
    pub cells: [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE],
    pub score: usize,
    rng: ThreadRng,
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
    ///     GameBoard::empty().0,
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
            rng: rand::thread_rng(),
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

    /// Moves all tiles on the board
    pub fn r#move(&mut self, dir: MoveDirection) {
        let rot = dir as usize;
        self.rotate(rot);

        for y in 0..self.cells.len() {
            for x in 0..self.cells.len() {
                if let BoardSpace::Tile(t) = self.cells[y][x] {
                    for x2 in (x + 1)..self.cells.len() {
                        match self.cells[y][x2] {
                            BoardSpace::Tile(t2) if t == t2 => {
                                self.cells[y][x] = BoardSpace::Tile(t * 2);
                                self.cells[y][x2] = BoardSpace::Vacant;
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
                            break;
                        }
                    }
                }
            }
        }

        self.rotate(self.cells.len() - rot);

        self.add_random_tile();
    }

    pub fn has_won(&mut self) -> bool {
        !self.can_move()
    }

    // TODO: make this not mutate?
    fn can_move(&mut self) -> bool {
        for n in 0..4 {
            self.rotate(n);

            for y in 0..self.cells.len() {
                for x in 0..(self.cells.len() - 1) {
                    match self.cells[y][x] {
                        BoardSpace::Vacant => {
                            self.rotate(4 - n);
                            return true;
                        },

                        BoardSpace::Tile(t) => match self.cells[y][x + 1] {
                            BoardSpace::Vacant => {
                                self.rotate(4 - n);
                                return true;
                            },

                            BoardSpace::Tile(t2) if t == t2 => {
                                self.rotate(4 - n);
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
        let n = self.cells.len();

        for _ in 0..times {
            // credit: someone on stackoverflow idk
            for i in 0..(n / 2) {
                for j in i..(n - i - 1) {
                    let tmp = self.cells[i][j];
                    self.cells[i][j] = self.cells[j][n - i - 1];
                    self.cells[j][n - i - 1] = self.cells[n - i - 1][n - j - 1];
                    self.cells[n - i - 1][n - j - 1] = self.cells[n - j - 1][i];
                    self.cells[n - j - 1][i] = tmp;
                }
            }
        }
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
            Self::Vacant => write!(f, "-"),
            Self::Tile(n) => write!(f, "{n}"),
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
}
