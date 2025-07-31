use super::neural_network::NeuralNetwork;
use rand::Rng;

pub struct GeneticAlgorithm {
    pub population_size: usize,
    pub mutation_rate: f32,
    pub mutation_strength: f32,
    pub elite_count: usize,
    pub generation: u32,
}

impl GeneticAlgorithm {
    pub fn new(population_size: usize) -> Self {
        Self {
            population_size,
            mutation_rate: 0.1,
            mutation_strength: 0.3,
            elite_count: population_size / 10, // Top 10% are elite
            generation: 0,
        }
    }
    
    pub fn create_initial_population(&self, input_size: usize, hidden_size: usize, output_size: usize) -> Vec<NeuralNetwork> {
        (0..self.population_size)
            .map(|_| NeuralNetwork::new(input_size, hidden_size, output_size))
            .collect()
    }
    
    pub fn evolve(&mut self, population: &[(NeuralNetwork, f32)]) -> Vec<NeuralNetwork> {
        self.generation += 1;
        
        // Sort by fitness (higher is better)
        let mut sorted_population = population.to_vec();
        sorted_population.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        let mut new_population = Vec::new();
        
        // Keep elite individuals
        for i in 0..self.elite_count {
            new_population.push(sorted_population[i].0.clone());
        }
        
        // Generate rest through crossover and mutation
        let mut rng = rand::thread_rng();
        while new_population.len() < self.population_size {
            // Tournament selection
            let parent1 = self.tournament_selection(&sorted_population);
            let parent2 = self.tournament_selection(&sorted_population);
            
            // Crossover
            let mut child = parent1.crossover(parent2);
            
            // Mutation
            child.mutate(self.mutation_rate, self.mutation_strength);
            
            new_population.push(child);
        }
        
        new_population
    }
    
    fn tournament_selection<'a>(&self, population: &'a [(NeuralNetwork, f32)]) -> &'a NeuralNetwork {
        let mut rng = rand::thread_rng();
        let tournament_size = 5;
        
        let mut best_fitness = f32::NEG_INFINITY;
        let mut best_individual = &population[0].0;
        
        for _ in 0..tournament_size {
            let index = rng.gen_range(0..population.len());
            let fitness = population[index].1;
            
            if fitness > best_fitness {
                best_fitness = fitness;
                best_individual = &population[index].0;
            }
        }
        
        best_individual
    }
    
    pub fn get_best_fitness(&self, population: &[(NeuralNetwork, f32)]) -> f32 {
        population.iter()
            .map(|(_, fitness)| *fitness)
            .fold(f32::NEG_INFINITY, f32::max)
    }
    
    pub fn get_average_fitness(&self, population: &[(NeuralNetwork, f32)]) -> f32 {
        let sum: f32 = population.iter().map(|(_, fitness)| *fitness).sum();
        sum / population.len() as f32
    }
}