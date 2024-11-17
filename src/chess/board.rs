use crate::chess::{Queen, Square, Queens};

use rand::RngCore;
use nannou::geom::Rect;

pub struct Board {
    pub queens: Queens,
    pub board_size: usize,
    pub squares: Vec<Square>,
}

impl Board {
    pub fn new(win: Rect, rng: &mut dyn RngCore, board_size: usize, nb_queens: usize) -> Self {
        let mut squares = vec![];
        let size = win.w().min(win.h()) / board_size as f32;

        for i in 0..board_size {
            for j in 0..board_size {
                let x = win.left() + size * i as f32 + size / 2.0;
                let y = win.bottom() + size * j as f32 + size / 2.0;
                let color = if (i + j) % 2 == 0 { "black" } else { "white" };
                squares.push(Square::new(x, y, color));
            }
        }

        Self { squares, board_size, queens: Queens::new(rng, nb_queens, board_size) }
    }

    pub fn squares(&self) -> impl Iterator<Item = &Square> {
        self.squares.iter()
    }

    pub fn queens(&self) -> impl Iterator<Item = &Queen> {
        self.queens.iter()
    }

    pub fn position(&self, position: (usize, usize)) -> (f32, f32) {
        let square = &self.squares[position.1 * self.board_size + position.0];
        (square.x, square.y)
    }

    pub fn evaluate(&self) -> f32 {
        self.queens.evaluate()
    }

    pub fn crossover(&mut self, rng: &mut dyn RngCore) {
        self.queens.crossover(rng);
    }

    pub fn mutate(&mut self, rng: &mut dyn RngCore) {
        self.queens.mutate(rng, self.board_size);
    }
}
