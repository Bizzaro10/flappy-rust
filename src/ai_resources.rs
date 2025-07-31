use bevy::prelude::*;
use crate::ai::{GeneticAlgorithm, NeuralNetwork};

#[derive(Resource)]
pub struct AITraining {
    pub genetic_algorithm: GeneticAlgorithm,
    pub current_population: Vec<NeuralNetwork>,
    pub population_fitness: Vec<f32>,
    pub training_active: bool,
    pub generation_timer: Timer,
    pub max_generation_time: f32,
    pub alive_count: usize,
}

impl Default for AITraining {
    fn default() -> Self {
        let population_size = 50;
        let mut ga = GeneticAlgorithm::new(population_size);
        let current_population = ga.create_initial_population(4, 8, 1); // 4 inputs, 8 hidden, 1 output
        
        Self {
            genetic_algorithm: ga,
            current_population,
            population_fitness: vec![0.0; population_size],
            training_active: false,
            generation_timer: Timer::from_seconds(30.0, TimerMode::Once), // 30 seconds per generation max
            max_generation_time: 30.0,
            alive_count: 0,
        }
    }
}

#[derive(Resource, Default)]
pub struct AIStats {
    pub best_fitness: f32,
    pub average_fitness: f32,
    pub best_score: u32,
    pub generation: u32,
}