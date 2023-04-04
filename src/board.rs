
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
}

pub struct Board {
    tiles: Vec<Vec<Tile>>,
    win_row_length: usize,
}
impl Board {
    pub fn new(size: usize, win_row_length: usize) -> Self {
        Self {
            tiles: vec![vec![Tile::Empty; size]; size],
            win_row_length: win_row_length
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

    pub fn find_winner(&self) -> Option<Tile> {
        for row in 0..self.tiles.len() {
            for col in 0..self.tiles[row].len() {
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
                    if l.iter().all(|t| *t == Some(Tile::Cross )) {return Some(Tile::Cross);}
                    if l.iter().all(|t| *t == Some(Tile::Nought)) {return Some(Tile::Nought);}
                }
            }
        }
        
        return None;
        
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
}

#[cfg(test)]
mod tests {
    use super::{Tile::*, Board};

    #[test]
    fn find_winner() {
        let mut b = Board::new(3, 3);

        b.tiles = vec![
            vec![Cross, Nought, Cross],
            vec![Nought, Cross, Empty],
            vec![Nought, Empty, Cross],
        ];
        assert_eq!(b.find_winner(), Some(Cross));

        b.tiles = vec![
            vec![Cross, Nought, Cross],
            vec![Cross, Nought, Empty],
            vec![Nought, Cross, Cross],
        ];
        assert_eq!(b.find_winner(), None);

        b.tiles = vec![
            vec![ Cross, Nought,  Cross],
            vec![Nought, Nought, Nought],
            vec![ Cross,  Cross,  Empty],
        ];
        assert_eq!(b.find_winner(), Some(Nought));



        let mut b2 = Board::new(4, 2);
        b2.tiles = vec![
            vec![ Cross, Empty,  Cross, Empty],
            vec![Nought,  Empty, Nought, Empty],
            vec![ Empty,  Empty,  Empty, Cross],
            vec![ Nought,  Cross, Empty, Nought],
        ];
        assert_eq!(b2.find_winner(), None);

        b2.tiles = vec![
            vec![ Cross, Empty,  Cross, Empty],
            vec![Nought,  Empty, Nought, Empty],
            vec![ Cross,  Empty,  Empty, Empty],
            vec![ Nought,  Cross, Empty, Cross],
        ];
        assert_eq!(b2.find_winner(), Some(Cross));
    }
}
