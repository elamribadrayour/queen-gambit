use crate::chess::Queen;

use rand::{Rng, RngCore};


pub struct Queens {
    queens: Vec<Queen>,
}

impl Queens {
    pub fn new(rng: &mut dyn RngCore, nb_queens: usize, board_size: usize) -> Self {

        let queens = (0..nb_queens).map(|_| {
            let x = rng.gen_range(0..board_size);
            let y = rng.gen_range(0..board_size);
            Queen::new((x, y))
        }).collect();

        Self { queens }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Queen> {
        self.queens.iter()
    }

    pub fn evaluate(&self) -> f32 {
        let mut score = 0.0;
        for (i, queen1) in self.queens.iter().enumerate() {
            for queen2 in self.queens.iter().skip(i + 1) {
                let (x1, y1) = queen1.position;
                let (x2, y2) = queen2.position;

                if x1 != x2 && y1 != y2 && (x1 as isize - x2 as isize).abs() != (y1 as isize - y2 as isize).abs() {
                    score += 1.0;
                }
            }
        }
        score / (self.queens.len() * (self.queens.len() - 1) / 2) as f32
    }

    pub fn crossover(&mut self, rng: &mut dyn RngCore) {
        let fitnesses = self.queens.iter().map(|q| q.fitness(&self.queens)).collect::<Vec<f32>>();
        let mean = fitnesses.iter().sum::<f32>() / fitnesses.len() as f32;
        let var = fitnesses.iter().map(|f| (f - mean).powi(2)).sum::<f32>() / fitnesses.len() as f32;
        let ids = (0..self.queens.len()).filter(|&i| (fitnesses[i] - mean).abs() > var).collect::<Vec<usize>>();

        let len = self.queens.len();
        let mut new_queens = Vec::with_capacity(len);
        for _ in 0..len {
            let i1 = ids[rng.gen_range(0..ids.len())];
            let i2 = ids[rng.gen_range(0..ids.len())];
            let parent1 = &self.queens[i1];
            let parent2 = &self.queens[i2];
            new_queens.push(Queen::crossover(parent1, parent2, rng));
        }
        self.queens = new_queens;
    }

    pub fn mutate(&mut self, rng: &mut dyn RngCore, board_size: usize) {
        self.queens.iter_mut().for_each(|queen| {
            queen.mutate(rng, board_size);
        });
    }
}
