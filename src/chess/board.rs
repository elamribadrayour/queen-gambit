use crate::chess::{Queen, Queens, Square};

use nannou::geom::Rect;
use rand::RngCore;

pub struct Board {
    pub queens: Queens,
    pub board_size: usize,
    pub squares: Vec<Square>,
}

impl Board {
    pub fn new(
        win: Rect,
        rng: &mut dyn RngCore,
        board_size: usize,
        nb_queens: usize,
        mutation_likelihood: f32,
        crossover_likelihood: f32,
    ) -> Self {
        let mut squares = vec![];
        let size = win.w().min(win.h()) / board_size as f32;

        (0..board_size).for_each(|i| {
            (0..board_size).for_each(|j| {
                let x = win.left() + size * i as f32 + size / 2.0;
                let y = win.bottom() + size * j as f32 + size / 2.0;
                let color = if (i + j) % 2 == 0 { "black" } else { "white" };
                squares.push(Square::new(x, y, color));
            });
        });

        let queens = Queens::new(
            rng,
            nb_queens,
            board_size,
            mutation_likelihood,
            crossover_likelihood,
        );

        Self {
            queens,
            board_size,
            squares,
        }
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

    pub fn evaluate(&mut self) {
        self.queens.evaluate()
    }

    pub fn crossover(&mut self, rng: &mut dyn RngCore) {
        self.queens.crossover(rng);
    }

    pub fn mutate(&mut self, rng: &mut dyn RngCore) {
        self.queens.mutate(rng, self.board_size);
    }

    pub fn fitness(&self) -> f32 {
        self.queens.fitness()
    }
}
