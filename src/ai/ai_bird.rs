use bevy::prelude::*;
use super::neural_network::NeuralNetwork;

#[derive(Component)]
pub struct AIBird {
    pub brain: NeuralNetwork,
    pub fitness: f32,
    pub alive: bool,
    pub timer: Timer,
    pub velocity: f32,
    pub score: u32,
    pub frames_alive: u32,
}

impl AIBird {
    pub fn new(brain: NeuralNetwork) -> Self {
        Self {
            brain,
            fitness: 0.0,
            alive: true,
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            velocity: 0.0,
            score: 0,
            frames_alive: 0,
        }
    }
    
    pub fn think(&mut self, inputs: &[f32]) -> bool {
        if !self.alive {
            return false;
        }
        
        let outputs = self.brain.forward(inputs);
        // If output > 0.5, jump
        outputs[0] > 0.5
    }
    
    pub fn calculate_fitness(&mut self) {
        // Fitness based on survival time and score
        self.fitness = (self.frames_alive as f32 * 0.1) + (self.score as f32 * 100.0);
        
        // Bonus for staying alive longer
        if self.frames_alive > 1000 {
            self.fitness += (self.frames_alive as f32 - 1000.0) * 0.5;
        }
    }
    
    pub fn die(&mut self) {
        self.alive = false;
        self.calculate_fitness();
    }
    
    pub fn update_frames(&mut self) {
        if self.alive {
            self.frames_alive += 1;
        }
    }
}