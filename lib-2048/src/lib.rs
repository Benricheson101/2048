use std::fmt;

/// Default dimensions of the [`GameBoard`]
pub const GAME_BOARD_SIZE: usize = 4;

/// A representation of the location of a space on the game board
pub type GameBoardLocation = (usize, usize);

/// Represents the grid of tiles making up the game
///
/// The `(0,0)` origin of the board is located in the top-left corner of the
/// board, with coordinates increasing as you move toward the bottom-right of
/// the board. Coordinates are in the form (row, column)
#[derive(Debug)]
pub struct GameBoard(pub [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE]);

impl GameBoard {
    /// Creates a new square [`GameBoard`](Self) with dimensions [`GAME_BOARD_SIZE`], prefilled
    /// with two tiles
    pub fn new() -> Self {
        // TODO: fil with two tiles at random places
        Self([[BoardSpace::Vacant; GAME_BOARD_SIZE]; GAME_BOARD_SIZE])
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
        Self([[BoardSpace::Vacant; GAME_BOARD_SIZE]; GAME_BOARD_SIZE])
    }

    /// Gets the value of a cell on the game board
    pub fn get(&self, (x, y): GameBoardLocation) -> BoardSpace {
        self.0[y][x]
    }

    /// Sets the value of a cell on the game board
    pub fn set(&mut self, (x, y): GameBoardLocation, val: BoardSpace) {
        self.0[y][x] = val;
    }

    /// Moves all tiles on the board
    pub fn r#move(&mut self, dir: MoveDirection) {
        match dir {
            MoveDirection::Up => {
                for x in 0..self.0.len() {
                    for y in 0..self.0.len() {
                        if let BoardSpace::Tile(t) = self.0[y][x] {
                            for y2 in (y + 1)..self.0.len() {
                                match self.0[y2][x] {
                                    BoardSpace::Tile(t2) if t == t2 => {
                                        self.0[y][x] = BoardSpace::Tile(t * 2);
                                        self.0[y2][x] = BoardSpace::Vacant;
                                        break;
                                    },
                                    BoardSpace::Tile(_) => break,
                                    _ => continue,
                                }
                            }
                        }
                    }

                    for y in 0..self.0.len() {
                        if let BoardSpace::Vacant = self.0[y][x] {
                            for y2 in (y + 1)..self.0.len() {
                                if let BoardSpace::Tile(_) = self.0[y2][x] {
                                    self.0[y][x] = self.0[y2][x];
                                    self.0[y2][x] = BoardSpace::Vacant;
                                    break;
                                }
                            }
                        }
                    }
                }
            },

            MoveDirection::Down => {
                for x in 0..self.0.len() {
                    for y in (0..self.0.len()).rev() {
                        if let BoardSpace::Tile(t) = self.0[y][x] {
                            for y2 in (0..y).rev() {
                                match self.0[y2][x] {
                                    BoardSpace::Tile(t2) if t == t2 => {
                                        self.0[y][x] = BoardSpace::Tile(t * 2);
                                        self.0[y2][x] = BoardSpace::Vacant;
                                        break;
                                    },
                                    BoardSpace::Tile(_) => break,
                                    _ => continue,
                                }
                            }
                        }
                    }

                    for y in (0..self.0.len()).rev() {
                        if let BoardSpace::Vacant = self.0[y][x] {
                            for y2 in (0..y).rev() {
                                if let BoardSpace::Tile(_) = self.0[y2][x] {
                                    self.0[y][x] = self.0[y2][x];
                                    self.0[y2][x] = BoardSpace::Vacant;
                                    break;
                                }
                            }
                        }
                    }
                }
            },

            MoveDirection::Left => {
                for y in 0..self.0.len() {
                    for x in 0..self.0.len() {
                        if let BoardSpace::Tile(t) = self.0[y][x] {
                            for x2 in (x + 1)..self.0.len() {
                                match self.0[y][x2] {
                                    BoardSpace::Tile(t2) if t == t2 => {
                                        self.0[y][x] = BoardSpace::Tile(t * 2);
                                        self.0[y][x2] = BoardSpace::Vacant;
                                        break;
                                    },

                                    BoardSpace::Tile(_) => break,
                                    _ => continue,
                                }
                            }
                        }
                    }

                    for x in 0..self.0.len() {
                        if let BoardSpace::Vacant = self.0[y][x] {
                            for x2 in x..self.0.len() {
                                if let BoardSpace::Tile(_) = self.0[y][x2] {
                                    self.0[y].swap(x, x2);
                                    break;
                                }
                            }
                        }
                    }
                }
            },

            MoveDirection::Right => {
                for y in 0..self.0.len() {
                    for x in (0..self.0.len()).rev() {
                        if let BoardSpace::Tile(t) = self.0[y][x] {
                            for x2 in (0..x).rev() {
                                match self.0[y][x2] {
                                    BoardSpace::Tile(t2) if t == t2 => {
                                        self.0[y][x] = BoardSpace::Tile(t * 2);
                                        self.0[y][x2] = BoardSpace::Vacant;
                                        break;
                                    },

                                    BoardSpace::Tile(_) => break,
                                    _ => continue,
                                }
                            }
                        }
                    }

                    for x in (0..self.0.len()).rev() {
                        if let BoardSpace::Vacant = self.0[y][x] {
                            for x2 in (0..x).rev() {
                                if let BoardSpace::Tile(_) = self.0[y][x2] {
                                    self.0[y].swap(x, x2);
                                    break;
                                }
                            }
                        }
                    }
                }
            },
        }
    }
}

impl Default for GameBoard {
    fn default() -> Self {
        Self::empty()
    }
}

/// The direction of a [`move`](GameBoard::move)
#[derive(Debug)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
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
        let mut board = GameBoard(SAMPLE_GAME_BOARD);

        board.r#move(MoveDirection::Up);

        const EXPECTED: [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE] = [
            [Tile(4), Tile(2), Tile(2), Tile(4)],
            [Tile(2), Tile(8), Tile(2), Vacant],
            [Vacant, Tile(4), Vacant, Vacant],
            [Vacant, Vacant, Vacant, Vacant],
        ];

        assert_eq!(board.0, EXPECTED);
    }

    #[test]
    fn move_down() {
        let mut board = GameBoard(SAMPLE_GAME_BOARD);

        board.r#move(MoveDirection::Down);

        const EXPECTED: [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE] = [
            [Vacant, Vacant, Vacant, Vacant],
            [Vacant, Tile(2), Vacant, Vacant],
            [Tile(2), Tile(8), Tile(2), Vacant],
            [Tile(4), Tile(4), Tile(2), Tile(4)],
        ];

        assert_eq!(board.0, EXPECTED);
    }

    #[test]
    fn move_left() {
        let mut board = GameBoard(SAMPLE_GAME_BOARD);

        board.r#move(MoveDirection::Left);

        const EXPECTED: [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE] = [
            [Tile(4), Tile(4), Vacant, Vacant],
            [Tile(2), Tile(8), Tile(1), Vacant],
            [Vacant, Vacant, Vacant, Vacant],
            [Tile(2), Tile(4), Tile(1), Tile(2)],
        ];

        assert_eq!(board.0, EXPECTED);
    }

    #[test]
    fn move_right() {
        let mut board = GameBoard(SAMPLE_GAME_BOARD);

        board.r#move(MoveDirection::Right);

        const EXPECTED: [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE] = [
            [Vacant, Vacant, Tile(4), Tile(4)],
            [Vacant, Tile(2), Tile(8), Tile(1)],
            [Vacant, Vacant, Vacant, Vacant],
            [Tile(2), Tile(4), Tile(1), Tile(2)],
        ];

        assert_eq!(board.0, EXPECTED);
    }

    #[test]
    fn get() {
        let board = GameBoard(SAMPLE_GAME_BOARD);
        let got = board.get((0, 0));
        assert_eq!(got, BoardSpace::Tile(2));
    }

    #[test]
    fn set() {
        let mut board = GameBoard(SAMPLE_GAME_BOARD);
        board.set((0, 0), BoardSpace::Tile(2048));

        const EXPECTED: [[BoardSpace; GAME_BOARD_SIZE]; GAME_BOARD_SIZE] = [
            [Tile(2048), Tile(2), Tile(2), Tile(2)],
            [Tile(2), Tile(8), Tile(1), Vacant],
            [Vacant, Vacant, Vacant, Vacant],
            [Tile(2), Tile(4), Tile(1), Tile(2)],
        ];

        assert_eq!(board.0, EXPECTED);
    }
}
