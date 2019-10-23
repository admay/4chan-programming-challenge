use std::{fmt, process};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "knights_tour")]
struct Opt {
    /// Starting x position of knight (must be between 0 and 7)
    #[structopt(short="x", long="x-pos")]
    x: i32,

    /// Starting y position of knight (must be between 0 and 7)
    #[structopt(short="y", long="y-pos")]
    y: i32
}

// Board size
const BOARD_SIZE: usize = 8;

// Possible knight moves
const MOVES: [(i32, i32); 8] = [
    (2, 1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (-1, -2),
    (1, -2),
    (2, -1),
];

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// Moves a point's position from (self.x, self.y) by (dx, dy)
    fn mov(&self, &(dx, dy): &(i32, i32)) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

struct Board {
    field: [[i32; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    /// Creates a new board of size `BOARD_SIZE`
    fn new() -> Self {
        Self {
            field: [[0; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    /// Check is a point is both on the board and open
    fn available(&self, p: Point) -> bool {
        0 <= p.x
            && p.x < BOARD_SIZE as i32
            && 0 <= p.y
            && p.y < BOARD_SIZE as i32
            && self.field[p.x as usize][p.y as usize] == 0
    }

    /// Counts the number of available moves on a board
    fn count_degree(&self, p: Point) -> i32 {
        let mut count = 0;
        for dir in MOVES.iter() {
            let next = p.mov(dir);
            if self.available(next) {
                count += 1;
            }
        }
        count
    }
}

// Display implementation for Board
// Allows for us to use printing macros with {} on a board
impl fmt::Display for Board {
    /// Prints the board so that it is printed one row at a time
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.field.iter() {
            for x in row.iter() {
                write!(f, "{:3} ", x)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn knights_tour(x: i32, y: i32) -> Option<Board> {
    let mut board = Board::new();
    let mut p = Point { x: x, y: y };
    let mut step = 1;

    // The positions on the board will be filled by the
    // value indicating which step the Kinght came through that point at
    board.field[p.x as usize][p.y as usize] = step;
    step += 1;

    while step <= (BOARD_SIZE * BOARD_SIZE) as i32 {
        let mut candidates = vec![];

        // For all of the possible moves,
        // check if the position is available.
        // If it is, get the degree count and
        // push it to the candidates.
        for dir in MOVES.iter() {
            let adj = p.mov(dir);
            if board.available(adj) {
                let degree = board.count_degree(adj);
                candidates.push((degree, adj));
            }
        }

        match candidates.iter().min() {
            // move to the next square
            Some(&(_, adj)) => p = adj,
            // can't move, tour failed
            None => return None,
        }

        board.field[p.x as usize][p.y as usize] = step;
        step += 1;
    }
    Some(board)
}

fn main() {
    let opt = Opt::from_args();
    let (x, y) = (opt.x, opt.y);

    if (x + 1) > BOARD_SIZE as i32 || (y + 1) > BOARD_SIZE as i32 {
        println!("The x and y values must be between 0 and 7!");
        process::exit(1);
    }

    println!("Board Size: {}", BOARD_SIZE);
    println!("Starting Position: ({}, {})",x, y);

    match knights_tour(x, y) {
        Some(board) => println!("{}", board),
        None => println!("The knight failed his tour."),
    }
}
