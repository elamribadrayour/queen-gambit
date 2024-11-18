use nannou::App;
use nannou_wgpu::Texture;

use rand::{Rng, RngCore};

pub struct Queen {
    fitness: f32,
    position: (usize, usize),
}

impl Queen {
    pub fn new(position: (usize, usize), fitness: f32) -> Self {
        Self { position, fitness }
    }

    pub fn texture(&self, app: &App) -> Texture {
        Texture::from_path(app, "assets/images/queen.png").unwrap()
    }

    pub fn crossover(
        parent1: &Self,
        parent2: &Self,
        crossover_likelihood: f32,
        rng: &mut dyn RngCore,
    ) -> Self {
        if !rng.gen_bool(crossover_likelihood as f64) {
            return Self::new(parent1.position, parent1.fitness);
        }

        let use_parent1 = rng.gen_bool(crossover_likelihood as f64);
        if use_parent1 {
            Self::new(parent1.position, parent1.fitness)
        } else {
            Self::new(parent2.position, parent2.fitness)
        }
    }

    pub fn mutate(&mut self, mutation_likelihood: f32, board_size: usize, rng: &mut dyn RngCore) {
        if !rng.gen_bool(mutation_likelihood as f64) {
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

    pub fn evaluate(&mut self, queens: &[&Queen]) {
        let mut intersections = 0.0;

        queens.iter().for_each(|queen| {
            intersections += self.intersect(queen);
        });

        self.fitness = 1.0 / (intersections + 1.0);
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }

    pub fn fitness(&self) -> f32 {
        self.fitness
    }
}
