#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

use std::usize;

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: vec![T::default(); rows * cols],
        }
    }
    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        assert_eq!(grid.len(), rows * cols);
        Self {
            rows,
            cols,
            grid: Vec::from(grid),
        }
    }

    pub fn get_grid(&self) -> Vec<T> {
        self.grid.clone()
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        self.grid.get(row * self.cols + col).unwrap()
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        *(self.grid.get_mut(row * self.cols + col).unwrap()) = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
        // return
        return match (row, col) {
            // rows == 1 && cols == 1
            (0, 0) if self.rows == 1 && self.cols == 1 => Neighbours::new(),
            // rows == 1
            (0, 0) if self.rows == 1 => {
                let mut n = Neighbours::new();
                n.set((0, 1));
                n
            }
            (0, c) if self.rows == 1 && c == self.cols - 1 => {
                let mut n = Neighbours::new();
                n.set((0, self.cols - 2));
                n
            }
            (0, 0) if self.rows == 1 => {
                let mut n = Neighbours::new();
                n.set((0, col - 1));
                n.set((0, col + 1));
                n
            }
            // cols == 1
            (0, 0) if self.rows == 1 => {
                let mut n = Neighbours::new();
                n.set((1, 0));
                n
            }
            (0, 0) if self.rows == 1 => {
                let mut n = Neighbours::new();
                n.set((self.rows - 2, 0));
                n
            }
            (0, 0) if self.rows == 1 => {
                let mut n = Neighbours::new();
                n.set((row - 1, 0));
                n.set((row + 1, 0));
                n
            }
            // cols > 1 && rows > 1
            (0, 0) => {
                let mut n = Neighbours::new();
                n.set((0, 1));
                n.set((1, 0));
                n.set((1, 1));
                n
            }
            (0, c) if c == self.cols - 1 => {
                let mut n = Neighbours::new();
                n.set((row, col - 1));
                n.set((row + 1, col - 1));
                n.set((row + 1, col));
                n
            }
            (r, 0) if r == self.rows - 1 => {
                let mut n = Neighbours::new();
                n.set((row - 1, col));
                n.set((row - 1, col + 1));
                n.set((row, col + 1));
                n
            }
            (r, c) if r == self.rows - 1 && c == self.cols - 1 => {
                let mut n = Neighbours::new();
                n.set((row - 1, col - 1));
                n.set((row - 1, col));
                n.set((row, col - 1));
                n
            }
            (0, _) => {
                let mut n = Neighbours::new();
                n.set((row, col - 1));
                n.set((row, col + 1));
                n.set((row + 1, col - 1));
                n.set((row + 1, col));
                n.set((row + 1, col + 1));
                n
            }
            (_, 0) => {
                let mut n = Neighbours::new();
                n.set((row - 1, col));
                n.set((row - 1, col + 1));
                n.set((row, col + 1));
                n.set((row + 1, col));
                n.set((row + 1, col + 1));
                n
            }
            (r, _) if r == self.rows - 1 => {
                let mut n = Neighbours::new();
                n.set((row - 1, col - 1));
                n.set((row - 1, col));
                n.set((row - 1, col + 1));
                n.set((row, col - 1));
                n.set((row, col + 1));
                n
            }
            (_, c) if c == self.cols - 1 => {
                let mut n = Neighbours::new();
                n.set((row - 1, col - 1));
                n.set((row - 1, col));
                n.set((row, col - 1));
                n.set((row + 1, col - 1));
                n.set((row + 1, col));
                n
            }
            _ => {
                let mut n = Neighbours::new();
                n.set((row - 1, col - 1));
                n.set((row - 1, col));
                n.set((row - 1, col + 1));
                n.set((row, col - 1));
                n.set((row, col + 1));
                n.set((row + 1, col - 1));
                n.set((row + 1, col));
                n.set((row + 1, col + 1));
                n
            }
        };
    }
}

pub struct Neighbours {
    data: [(usize, usize); 8],
    len: usize,
    index: usize,
}

impl Neighbours {
    fn new() -> Neighbours {
        Neighbours {
            data: [(0, 0); 8],
            len: 0,
            index: 0,
        }
    }

    fn set(&mut self, data: (usize, usize)) {
        self.data[self.len] = data;
        self.len += 1;
    }
}

impl Iterator for Neighbours {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if self.index < self.len {
            let item = self.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    #[default]
    Dead,
    Alive,
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let mut updates = Vec::new();
        for i in 0..self.grid.rows {
            for j in 0..self.grid.cols {
                let cell = self.grid.get(i, j);
                let n = self.grid.neighbours(i, j);
                let ln = n
                    .into_iter()
                    .filter(|x| *(self.grid.get(x.0, x.1)) == Cell::Alive)
                    .count();
                match (*cell, ln) {
                    (Cell::Alive, 0) => {
                        updates.push((Cell::Dead, i, j));
                    }
                    (Cell::Alive, 1) => {
                        updates.push((Cell::Dead, i, j));
                    }
                    (Cell::Alive, 4..9) => {
                        updates.push((Cell::Dead, i, j));
                    }
                    (Cell::Dead, 3) => {
                        updates.push((Cell::Alive, i, j));
                    }
                    _ => {}
                };
            }
        }

        for upd in updates {
            self.grid.set(upd.0, upd.1, upd.2);
        }
    }
}
