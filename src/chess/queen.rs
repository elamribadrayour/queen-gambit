use nannou::App;
use nannou_wgpu::Texture;

use rand::{RngCore, Rng};

pub struct Queen {
    pub position: (usize, usize),
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
        let x = (self.position.0 + rng.gen_range(0..board_size)) % board_size;
        let y = (self.position.1 + rng.gen_range(0..board_size)) % board_size;
        self.position = (x, y);
    }

    pub fn fitness(&self, queens: &[Queen]) -> f32 {
        let mut intersections = 0;

        for queen in queens {
            if self.position == queen.position {
                continue;
            }

            let (x1, y1) = self.position;
            let (x2, y2) = queen.position;

            if x1 == x2 || y1 == y2 || (x1 as isize - x2 as isize).abs() == (y1 as isize - y2 as isize).abs() {
                intersections += 1;
            }
        }

        1.0 / (intersections as f32 + 1.0)
    }
}
