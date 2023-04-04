
use itertools::Itertools;


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tile {
    Empty,
    Cross,
    Nought,
}
impl Tile {
    pub fn char(&self) -> &'static str {
        match self {
            Self::Empty   => " ",
            Self::Cross  => "X",
            Self::Nought => "O",
        }
    }

    pub fn opposite(&self) -> Option<Tile> {
        match &self {
            Self::Cross => Some(Self::Nought),
            Self::Nought => Some(Self::Cross),
            Self::Empty => None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BoardStatus {
    Winner(Tile),
    Tie,
    Continue,
}

pub struct Board {
    tiles: Vec<Vec<Tile>>,
    length: usize,
    win_row_length: usize,
}
impl Board {
    pub fn new(length: usize, win_row_length: usize) -> Self {
        Self {
            tiles: vec![vec![Tile::Empty; length]; length],
            length: length,
            win_row_length: win_row_length,
        }
    }

    pub fn print(&self) {
        const HORIZONTAL: char = '=';
        const VERTICAL: char   = '|';

        for row in &self.tiles {
            for _ in row {
                print!("{}{}", HORIZONTAL, HORIZONTAL);
            }
            println!("{}", HORIZONTAL);

            for t in row {
                print!("{}{}", VERTICAL, t.char());
            }
            println!("{}", VERTICAL);
        }

        println!();
    }

    fn get<T: TryInto<usize>>(&self, row: T, col: T) -> Option<Tile> {
        let tile = self.tiles
            .get(row.try_into().ok()?)?
            .get(col.try_into().ok()?)?;

        Some(*tile)
    }

    pub fn set(&mut self, tile: Tile, row: usize, col: usize) -> Result<(), &'static str> {
        let slot = self.tiles
                .get_mut(row).ok_or("Row index out of bounds.")?
                .get_mut(col).ok_or("Column index out of bounds.")?;

        if *slot != Tile::Empty {
            return Err("Already occupied tile.");
        }

        *slot = tile;
        Ok(())
    }

    pub fn board_status(&self) -> BoardStatus {
        for row in 0..self.length {
            for col in 0..self.length {
                let lines = [
                    get_line(self, (row, col), ( 1,  0)),
                    get_line(self, (row, col), (-1,  0)),
                    get_line(self, (row, col), ( 0,  1)),
                    get_line(self, (row, col), ( 0, -1)),

                    get_line(self, (row, col), ( 1,  1)),
                    get_line(self, (row, col), ( 1, -1)),
                    get_line(self, (row, col), (-1,  1)),
                    get_line(self, (row, col), (-1, -1)),
                ];

                for l in lines {
                    if l.iter().all(|t| *t == Some(Tile::Cross )) {
                        return BoardStatus::Winner(Tile::Cross);
                    }
                    if l.iter().all(|t| *t == Some(Tile::Nought)) {
                        return BoardStatus::Winner(Tile::Nought);
                    }
                }
            }
        }

        let is_tie = !self.tiles
            .iter()
            .flatten()
            .any(|tile| *tile == Tile::Empty);
        
        if is_tie {
            return BoardStatus::Tie;
        }

        return BoardStatus::Continue;
        
        fn get_line(
            self_board: &Board,
            (start_row, start_col): (usize, usize),
            (row_change, col_change): (i32, i32),
        ) -> Vec<Option<Tile>>
        {
            let length = self_board.win_row_length;
            (0..length).map(|i| {
                let row = start_row as i32 + i as i32 * row_change;
                let col = start_col as i32 + i as i32 * col_change;
                self_board.get(row, col)
            }).collect()
        }
    }

    pub fn make_random_move(&mut self, side: Tile) {
        use rand::seq::SliceRandom;

        let mut empty_tiles: Vec<&mut Tile> = self.tiles
            .iter_mut()
            .flatten()
            .filter(|t| **t == Tile::Empty)
            .collect();

        **empty_tiles
            .choose_mut(&mut rand::thread_rng())
            .unwrap()
            = side;
    }
    
    pub fn make_perfect_move(&mut self, side: Tile) {
        let move_at = (0..self.length).cartesian_product(0..self.length)
            .filter(|(row, col)| self.get(*row, *col).unwrap() == Tile::Empty)
            .collect::<Vec<(usize, usize)>>()
            .iter()
            .max_by(|pos1, pos2| {
                self.value_of_move(side, pos1.0, pos1.1)
                    .cmp(&self.value_of_move(side, pos2.0, pos2.1))
            })
            .unwrap()
            .clone();

        self.set(side, move_at.0, move_at.1).unwrap();
    }

    // //Private function where row and col always should be correct.
    fn value_of_move(&mut self, side: Tile, move_row: usize, move_col: usize) -> i8 {
        const WIN_VALUE: i8 = 1;
        const DRAW_VALUE: i8 = 0;
        const LOOSE_VALUE: i8 = -1;

        assert_eq!(self.get(move_row, move_col).unwrap(), Tile::Empty);
        self.set(side, move_row, move_col).unwrap();

        let value = match self.board_status() {
            BoardStatus::Winner(tile) => {
                if tile == side {WIN_VALUE}
                else {LOOSE_VALUE}
            },
            BoardStatus::Tie => DRAW_VALUE,
            BoardStatus::Continue => {
                let opponent_move = (0..self.length).cartesian_product(0..self.length)
                    .filter(|(row, col)| self.get(*row, *col).unwrap() == Tile::Empty)
                    .collect::<Vec<(usize, usize)>>()
                    .iter()
                    .map(|(row, col)| self.value_of_move(side.opposite().unwrap(), *row, *col))
                    .max()
                    .unwrap();
                
                -opponent_move
            }
        };

        self.tiles[move_row][move_col] = Tile::Empty;
        value
    }
    // fn foo(cord: (usize, usize), b: &mut Board) {
    //     b.value_of_move(side, move_row, move_col)
    // }
}

#[cfg(test)]
mod tests {
    use super::{Tile::*, BoardStatus::*, Board};

    #[test]
    fn board_status() {
        let mut b = Board::new(3, 3);

        b.tiles = vec![
            vec![Cross, Nought, Cross],
            vec![Nought, Cross, Empty],
            vec![Nought, Empty, Cross],
        ];
        assert_eq!(b.board_status(), Winner(Cross));

        b.tiles = vec![
            vec![Cross, Nought, Cross],
            vec![Cross, Nought, Empty],
            vec![Nought, Cross, Cross],
        ];
        assert_eq!(b.board_status(), Continue);

        b.tiles = vec![
            vec![ Cross, Nought,  Cross],
            vec![Nought, Nought, Nought],
            vec![ Cross,  Cross,  Empty],
        ];
        assert_eq!(b.board_status(), Winner(Nought));

        b.tiles = vec![
            vec![ Cross, Nought,  Cross],
            vec![Nought,  Cross, Nought],
            vec![Nought,  Cross, Nought],
        ];
        assert_eq!(b.board_status(), Tie);


        let mut b2 = Board::new(4, 2);
        b2.tiles = vec![
            vec![ Cross, Empty,  Cross, Empty],
            vec![Nought,  Empty, Nought, Empty],
            vec![ Empty,  Empty,  Empty, Cross],
            vec![ Nought,  Cross, Empty, Nought],
        ];
        assert_eq!(b2.board_status(), Continue);

        b2.tiles = vec![
            vec![ Cross, Empty,  Cross, Empty],
            vec![Nought,  Empty, Nought, Empty],
            vec![ Cross,  Empty,  Empty, Empty],
            vec![ Nought,  Cross, Empty, Cross],
        ];
        assert_eq!(b2.board_status(), Winner(Cross));
    }
}
