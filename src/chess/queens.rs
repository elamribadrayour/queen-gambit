use crate::chess::Queen;

use rand::{Rng, RngCore};
pub struct Queens {
    queens: Vec<Queen>,
    mutation_likelihood: f32,
    crossover_likelihood: f32,
}

impl Queens {
    pub fn new(
        rng: &mut dyn RngCore,
        nb_queens: usize,
        board_size: usize,
        mutation_likelihood: f32,
        crossover_likelihood: f32,
    ) -> Self {
        let queens = (0..nb_queens)
            .map(|_| {
                let x = rng.gen_range(0..board_size);
                let y = rng.gen_range(0..board_size);
                Queen::new((x, y), 0.0)
            })
            .collect();

        Self {
            queens,
            mutation_likelihood,
            crossover_likelihood,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Queen> {
        self.queens.iter()
    }

    pub fn evaluate(&mut self) {
        for i in 0..self.queens.len() {
            let (left, right) = self.queens.split_at_mut(i + 1);
            let queen1 = &mut left[i];
            queen1.evaluate(&right.iter().collect::<Vec<&Queen>>());
        }
    }

    fn select(&self, ids: &[usize], rng: &mut dyn RngCore) -> &Queen {
        // Roulette wheel selection
        // https://en.wikipedia.org/wiki/Fitness_proportionate_selection
        let fitnesses = self.fitnesses();
        let total_fitness = fitnesses.iter().sum::<f32>();
        let mut pick = rng.gen_range(0.0..total_fitness);
        for (i, fitness) in (0..ids.len()).zip(fitnesses) {
            pick -= fitness;
            if pick <= 0.0 {
                return &self.queens[ids[i]];
            }
        }
        &self.queens[0]
    }

    pub fn fitnesses(&self) -> Vec<f32> {
        self.queens
            .iter()
            .map(|q| q.fitness())
            .collect::<Vec<f32>>()
    }

    pub fn elite_ids(&self) -> Vec<usize> {
        // Select elitist ids of queens to crossover (fitness > mean + var)
        let fitnesses = self.fitnesses();
        (0..self.queens.len())
            .filter(|&i| fitnesses[i] > 0.5)
            .collect::<Vec<usize>>()
    }

    pub fn crossover(&mut self, rng: &mut dyn RngCore) {
        // Crossover between best queens
        // https://en.wikipedia.org/wiki/Crossover_(genetic_algorithm)

        let ids = self.elite_ids();
        let len = self.queens.len();
        let mut childs = Vec::with_capacity(len);
        (0..len).for_each(|_| {
            let parent1 = self.select(&ids, rng);
            let parent2 = self.select(&ids, rng);
            let child = Queen::crossover(parent1, parent2, self.crossover_likelihood, rng);
            childs.push(child);
        });
        self.queens = childs;
    }

    pub fn mutate(&mut self, rng: &mut dyn RngCore, board_size: usize) {
        self.queens.iter_mut().for_each(|queen| {
            queen.mutate(self.mutation_likelihood, board_size, rng);
        });
    }

    pub fn fitness(&self) -> f32 {
        self.queens.iter().map(|q| q.fitness()).sum::<f32>() / self.queens.len() as f32
    }
}
