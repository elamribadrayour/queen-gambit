use std::collections::HashSet;

use crate::chess::Queen;

use rand::{Rng, RngCore};
pub struct Queens {
    fitness: f32,
    queens: Vec<Queen>,
}

impl Queens {
    pub fn new(rng: &mut dyn RngCore, nb_queens: usize, board_size: usize) -> Self {
        let queens = (0..nb_queens)
            .map(|_| {
                let x = rng.gen_range(0..board_size);
                let y = rng.gen_range(0..board_size);
                Queen::new((x, y))
            })
            .collect();

        Self {
            fitness: 0.0,
            queens,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Queen> {
        self.queens.iter()
    }

    pub fn len(&self) -> usize {
        self.queens.len()
    }

    pub fn evaluate(&mut self) -> f32 {
        let mut nb_collisions = 0.0;
        self.iter().enumerate().for_each(|(i, queen1)| {
            self.iter().skip(i + 1).for_each(|queen2| {
                nb_collisions += queen1.intersect(queen2);
            });
        });

        let max_collisions = (self.len() * (self.len() - 1) / 2) as f32;
        self.fitness = (max_collisions - nb_collisions) / max_collisions;
        self.fitness
    }

    fn select(
        &self,
        ids: &[usize],
        fitnesses: &[f32],
        total_fitness: f32,
        rng: &mut dyn RngCore,
    ) -> &Queen {
        // Roulette wheel selection
        // https://en.wikipedia.org/wiki/Fitness_proportionate_selection
        let mut pick = rng.gen_range(0.0..total_fitness);
        for (i, &fitness) in (0..ids.len()).zip(fitnesses) {
            pick -= fitness;
            if pick <= 0.0 {
                return &self.queens[ids[i]];
            }
        }
        &self.queens[0]
    }

    pub fn crossover(&mut self, rng: &mut dyn RngCore) {
        if self.fitness == 1.0 {
            return;
        }

        // Calculate fitnesses
        let fitnesses = self
            .queens
            .iter()
            .map(|q| q.fitness(&self.queens))
            .collect::<Vec<f32>>();

        let total_fitness = fitnesses.iter().sum::<f32>();

        // Calculate mean and variance of fitnesses
        let mean = total_fitness / fitnesses.len() as f32;
        let var =
            fitnesses.iter().map(|f| (f - mean).powi(2)).sum::<f32>() / fitnesses.len() as f32;

        // Select elitist ids of queens to crossover
        let ids = (0..self.queens.len())
            .filter(|&i| (fitnesses[i] - mean).abs() > var)
            .collect::<Vec<usize>>();

        // Crossover between best queens
        let len = self.queens.len();
        let mut taken_positions = HashSet::new();
        let mut childs = Vec::with_capacity(len);
        (0..len).for_each(|_| {
            let parent1 = self.select(&ids, &fitnesses, total_fitness, rng);
            let parent2 = self.select(&ids, &fitnesses, total_fitness, rng);
            let child = Queen::crossover(parent1, parent2, rng);
            taken_positions.insert(child.position());
            childs.push(child);
        });
        self.queens = childs;
    }

    pub fn mutate(&mut self, rng: &mut dyn RngCore, board_size: usize) {
        if self.fitness == 1.0 {
            return;
        }

        self.queens.iter_mut().for_each(|queen| {
            queen.mutate(rng, board_size);
        });
    }

    pub fn fitness(&self) -> f32 {
        self.fitness
    }
}
