use nannou::App;
use nannou_wgpu::Texture;

use rand::{Rng, RngCore};

pub struct Queen {
    position: (usize, usize),
}

impl Queen {
    pub fn new(position: (usize, usize)) -> Self {
        Self { position }
    }

    pub fn texture(&self, app: &App) -> Texture {
        Texture::from_path(app, "assets/images/queen.png").unwrap()
    }

    pub fn crossover(parent1: &Self, parent2: &Self, rng: &mut dyn RngCore) -> Self {
        let (x1, y1) = parent1.position;
        let (x2, y2) = parent2.position;

        let use_parent1 = rng.gen_bool(0.5);
        let new_x = if use_parent1 { x1 } else { x2 };
        let new_y = if use_parent1 { y1 } else { y2 };

        Self::new((new_x, new_y))
    }

    pub fn mutate(&mut self, rng: &mut dyn RngCore, board_size: usize) {
        let mutation_likelihood = 0.1;
        if !rng.gen_bool(mutation_likelihood) {
            return;
        }

        let x = (self.position.0 + rng.gen_range(0..board_size)) % board_size;
        let y = (self.position.1 + rng.gen_range(0..board_size)) % board_size;
        self.position = (x, y);
    }

    pub fn intersect(&self, queen: &Queen) -> f32 {
        let (x1, y1) = self.position;
        let (x2, y2) = queen.position;

        if x1 == x2
            || y1 == y2
            || (x1 as isize - x2 as isize).abs() == (y1 as isize - y2 as isize).abs()
        {
            1.0
        } else {
            0.0
        }
    }

    pub fn fitness(&self, queens: &[Queen]) -> f32 {
        let mut intersections = 0.0;

        queens.iter().for_each(|queen| {
            intersections += self.intersect(queen);
        });

        1.0 / (intersections + 1.0)
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }
}
