use lib_2048::{BoardSpace::*, GameBoard, MoveDirection};

fn main() {
    let mut board = GameBoard::from([
        [Tile(2), Tile(2), Tile(2), Tile(2)],
        [Tile(2), Tile(8), Tile(1), Tile(1)],
        [Vacant, Vacant, Vacant, Vacant],
        [Tile(2), Tile(4), Tile(1), Tile(2)],
    ]);

    print_grid(&board);

    board.r#move(MoveDirection::Right);
    println!("------");
    print_grid(&board);

    board.r#move(MoveDirection::Up);
    println!("------");
    print_grid(&board);

    board.r#move(MoveDirection::Right);
    println!("------");
    print_grid(&board);

    board.r#move(MoveDirection::Left);
    println!("------");
    print_grid(&board);

    board.r#move(MoveDirection::Up);
    println!("------");
    print_grid(&board);

    board.r#move(MoveDirection::Left);
    println!("------");
    print_grid(&board);

    board.r#move(MoveDirection::Left);
    println!("------");
    print_grid(&board);

    board.r#move(MoveDirection::Up);
    println!("------");
    print_grid(&board);

    board.r#move(MoveDirection::Up);
    println!("------");
    print_grid(&board);
}

fn print_grid(board: &GameBoard) {
    println!("   {:^9} {:^9} {:^9} {:^9}", "0", "1", "2", "3");
    for (row, items) in board.cells.iter().enumerate() {
        for (col, cell) in items.iter().enumerate() {
            if col == 0 {
                print!("{row} |{:^9}|", format!("{}", cell));
            } else {
                print!("{:^9}|", format!("{}", cell));
            }
        }

        println!();
    }
}
