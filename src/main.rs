mod board;
use board::{Board, Tile::{*, self}, BoardStatus};
use std::{thread::sleep, time::Duration};

const RESPONSE_PAUSE: Duration = Duration::from_millis(800);

enum Player {
    Human(Tile),
    RandomAi(Tile),
}
impl Player {
    fn make_move(&self, board: &mut Board) {
        match self {
            Self::Human(tile)    => ensure_human_move(board, *tile),
            Self::RandomAi(tile) => board.make_random_move(*tile),
        }
    }

    fn tile(&self) -> Tile {
        match self {
            Self::Human(tile)    => *tile,
            Self::RandomAi(tile) => *tile,
        }
    }
}

fn main() {
    let mut b = Board::new(3, 3);

    let players = [
        Player::Human(Cross),
        Player::RandomAi(Nought),
    ];

    loop {
        for p in &players {
            p.make_move(&mut b);
            println!("{:?} move:", p.tile());
            b.print();

            sleep(RESPONSE_PAUSE);

            match b.board_status() {
                BoardStatus::Winner(tile) => {
                    println!("{:?} has won!", tile);
                    return;
                }
                BoardStatus::Tie => {
                    println!("Tie!");
                    return;
                }
                BoardStatus::Continue => ()
            }
        }
    }
}

fn ensure_human_move(board: &mut Board, side: Tile) {
    human_make_move(board, side.clone()).unwrap_or_else(|err| {
        println!("{}", err);
        sleep(RESPONSE_PAUSE);
        ensure_human_move(board, side.clone());
    });
}

fn human_make_move(board: &mut Board, side: Tile) -> Result<(), &'static str> {
    println!("Make move (x, y): ");

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).or(Err("Couldn't read input."))?;

    let cordinates: Vec<usize> = buf
        .split(',')
        .map(|s|{
            s.trim()
                .parse()
                .or(Err("You need to input proper numbers."))
        })
        .collect::<Result<Vec<usize>, &str>>()?;

    if cordinates.len() != 2 {return Err("Incorrect number of arguments.")}

    board.set(side, cordinates[1], cordinates[0])?;
    
    Ok(())
}
