## Rules




##agent.rs
use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::game::Pipe;
use crate::nn::Net;
use crate::resources::RESOURCES;
use crate::*;

#[derive(Clone)]
pub struct Bird {
    pub score: f32,
    pub is_dead: bool,

    pos: f32,
    vel: f32,
    acc: f32,
    brain: Net,
}

impl Bird {
    pub fn new() -> Self {
        Self {
            pos: screen_height() / 2.0,
            vel: 0.0,
            acc: 0.0,
            brain: Net::new(vec![5, 8, 1]),
            score: 0.0,
            is_dead: false,
        }
    }

    pub fn with_brain(other: &Bird) -> Self {
        let mut new_bird = Bird::new();
        new_bird.brain = other.brain.clone();
        new_bird
    }

    pub fn draw(&self) {
        if self.is_dead {
            return;
        }

        let resources = RESOURCES.get().unwrap();
        let vel = vec2(self.vel, 1.0);
        let heading = vel.y.atan2(vel.x) + PI / 2.0;
        draw_texture_ex(
            resources.bird_texture,
            BIRD_START_POS_X - 15.0,
            self.pos - 15.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(BIRD_TEXTURE_RESIZE.into()),
                flip_x: true,
                flip_y: true,
                rotation: heading,
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self, top_pipe: &Pipe, bottom_pipe: &Pipe) {
        if self.is_dead {
            return;
        }

        self.score += 1.0;
        self.acc += BIRD_GRAVITY;
        self.vel += self.acc;
        self.pos += self.vel;
        self.acc = 0.0;

        // Up-Down collision
        if self.pos >= screen_height() {
            self.mark_dead();
        }
        if self.pos <= 0.0 {
            self.mark_dead();
        }

        // Pipe collision
        if top_pipe.pos.x <= BIRD_START_POS_X && top_pipe.pos.x + PIPE_WIDTH >= BIRD_START_POS_X {
            // Top pipe
            if self.pos >= 0.0 && self.pos <= top_pipe.h {
                self.mark_dead();
            }
            // Bottom pipe
            if self.pos >= screen_height() - bottom_pipe.h && self.pos <= screen_height() {
                self.mark_dead();
            }
        }

        // Brain Inputs
        let out = self.brain.predict(&vec![
            self.pos as f64 / screen_height() as f64,
            top_pipe.h as f64 / screen_height() as f64,
            bottom_pipe.h as f64 / screen_height() as f64,
            top_pipe.pos.x as f64 / screen_width() as f64,
            self.vel as f64 / 10.0,
        ])[0];
        if out >= 0.5 {
            self.up_force();
        }
    }

    pub fn mutate(&mut self) {
        self.brain.mutate();
    }

    fn mark_dead(&mut self) {
        self.is_dead = true;
    }

    fn up_force(&mut self) {
        self.vel = 0.0;
        self.acc += BIRD_UP_FORCE;
    }
}



###config.rs
// Pipes
pub const PIPE_WIDTH: f32 = 75.0;
pub const PIPE_SPEED: f32 = 3.0;
pub const NUM_PIPES: usize = 10;
pub const MIN_VERTICAL_SPACE_BW_PIPES: f32 = 150.0;
pub const MIN_HORIZONTAL_SPACE_BW_PIPES: f32 = 450.0;
pub const MIN_PIPE_HEIGHT: f32 = 10.0;
pub const PIPES_START_X: f32 = 500.0;
pub const PIPE_HEAD_TEXTURE: &str = "./assets/pipe_head.png";
pub const PIPE_TEXTURE: &str = "./assets/pipe_up.png";

// Bird
pub const NUM_BIRDS: usize = 1000;
pub const BIRD_START_POS_X: f32 = 30.0;
pub const BIRD_GRAVITY: f32 = 0.3;
pub const BRAIN_MUTATION_RATE: f32 = 0.02;
pub const BRAIN_MUTATION_VARIATION: f32 = 0.2;
pub const BIRD_UP_FORCE: f32 = -5.0;
pub const BIRD_TEXTURE: &str = "./assets/bird.png";
pub const BIRD_TEXTURE_RESIZE: (f32, f32) = (30.0, 25.0);

// Others
pub const BACKGROUND_TEXTURE: &str = "./assets/background.png";
pub const PARALLAX_SPEED: f32 = 1.5;








###########editor.rs

use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::simulation::Statistics;

pub struct Settings {
    pub is_pause: bool,
    pub is_draw: bool,
    pub is_restart: bool,
    pub is_frame_skip: bool,
    pub is_show_egui: bool,
    pub show_one_bird: bool,
}

pub struct Editor {
    pub settings: Settings,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            is_pause: false,
            is_draw: true,
            is_restart: false,
            is_frame_skip: false,
            is_show_egui: true,
            show_one_bird: false,
        }
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            settings: Settings::new(),
        }
    }

    pub fn update(&mut self) {
        // Handle keyboard input
        if is_key_pressed(KeyCode::Space) {
            self.settings.is_pause = !self.settings.is_pause;
        }
        if is_key_pressed(KeyCode::Escape) {
            self.settings.is_show_egui = !self.settings.is_show_egui;
        }
        if is_key_pressed(KeyCode::R) {
            self.settings.is_restart = true;
        }
    }

    pub fn draw(&mut self, stats: &Statistics) {
        if !self.settings.is_show_egui {
            return;
        }

        egui_macroquad::ui(|ctx| {
            egui::Window::new("No Title")
                .title_bar(false)
                .min_width(200.0)
                .default_pos(egui::pos2(screen_width(), screen_height()))
                .show(ctx, |ui| {
                    egui::CollapsingHeader::new("Stats")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label(format!("FPS: {}", get_fps()));
                            ui.label(format!("Gen: {}", stats.generation_count));
                            ui.label(format!("Birds: {}", stats.birds_alive));
                        });

                    egui::CollapsingHeader::new("Options")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.checkbox(&mut self.settings.is_draw, "Draw");
                            ui.checkbox(&mut self.settings.show_one_bird, "Show One");
                            ui.checkbox(&mut self.settings.is_frame_skip, "Fast mode");
                        });

                    egui::CollapsingHeader::new("Controls")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.checkbox(&mut self.settings.is_pause, "Pause");
                            if ui.add(egui::Button::new("Restart")).clicked() {
                                self.settings.is_restart = true;
                            }
                        });
                });
        });
        egui_macroquad::draw();
    }
}



##########game.rs
use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::resources::RESOURCES;
use crate::*;

pub struct Game {
    pipe_manager: PipeManager,
    parallax: ParallaxBackground,
}

#[derive(Clone)]
pub struct Pipe {
    pub pos: Vec2,
    pub h: f32,
}

pub struct PipeManager {
    pipes: Vec<Pipe>,
    num_removed: u32,
}

struct ParallaxBackground {
    p1: f32,
    p2: f32,
}

impl Pipe {
    fn new(pos: Vec2, h: f32) -> Self {
        Self { pos, h }
    }

    fn draw(&self) {
        let resources = RESOURCES.get().unwrap();
        let is_top_pipe = self.pos.y == 0.0;

        draw_texture_ex(
            resources.pipe_texture,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                flip_y: is_top_pipe,
                dest_size: Some((PIPE_WIDTH, self.h).into()),
                ..Default::default()
            },
        );

        // TODO Get to draw the entire pipe all at once!!
        if is_top_pipe {
            draw_texture_ex(
                resources.pipe_head_texture,
                self.pos.x - 2.0,
                self.pos.y + self.h - 12.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some((PIPE_WIDTH + 4.0, 24.0).into()),
                    ..Default::default()
                },
            );
        } else {
            draw_texture_ex(
                resources.pipe_head_texture,
                self.pos.x - 2.0,
                self.pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some((PIPE_WIDTH + 4.0, 24.0).into()),
                    ..Default::default()
                },
            );
        }
    }

    fn update(&mut self) {
        self.pos.x -= PIPE_SPEED;
    }
}

impl ParallaxBackground {
    fn new() -> Self {
        Self {
            p1: 0.0,
            p2: screen_width(),
        }
    }

    fn update(&mut self) {
        self.p1 -= PARALLAX_SPEED;
        self.p2 -= PARALLAX_SPEED;

        if self.p1 <= -1.0 * screen_width() {
            self.p1 = screen_width() - 5.0;
        }
        if self.p2 <= -1.0 * screen_width() {
            self.p2 = screen_width() - 5.0;
        }
    }

    fn draw(&self) {
        let resources = RESOURCES.get().unwrap();
        draw_texture_ex(
            resources.background_texture,
            self.p1,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some((screen_width(), screen_height()).into()),
                ..Default::default()
            },
        );
        draw_texture_ex(
            resources.background_texture,
            self.p2,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some((screen_width(), screen_height()).into()),
                ..Default::default()
            },
        );
    }
}

impl PipeManager {
    fn new() -> Self {
        let mut x_pos = PIPES_START_X;
        let mut pm = Self {
            pipes: Vec::new(),
            num_removed: 0,
        };

        for _ in 0..NUM_PIPES {
            pm.add_pipe(x_pos);
            x_pos += MIN_HORIZONTAL_SPACE_BW_PIPES;
        }

        pm
    }

    fn update(&mut self) -> (Pipe, Pipe) {
        // Move pipes closer to bird
        self.pipes.iter_mut().for_each(|p| p.update());

        // Remove pipes beyond the screen
        self.pipes.retain(|p| p.pos.x >= -1.0 * PIPE_WIDTH);

        // Add new pipes
        let num_pipes = NUM_PIPES - (self.pipes.len() as f64 / 2.0) as usize;
        let x_pos = self.pipes.last().unwrap().pos.x + MIN_HORIZONTAL_SPACE_BW_PIPES;
        for _ in 0..num_pipes {
            self.add_pipe(x_pos);
            self.num_removed += 1;
        }

        self.get_nearest_pipes()
    }

    fn draw(&self) {
        let score = format!("Score: {}", self.num_removed);
        self.pipes.iter().for_each(|p| p.draw());
        draw_text(
            score.as_str(),
            screen_width() / 2.0 - 90.0,
            200.0,
            40.0,
            WHITE,
        );
    }

    fn get_nearest_pipes(&self) -> (Pipe, Pipe) {
        let top_pipe_height = self.pipes[0].clone();
        let bottom_pipe_height = self.pipes[1].clone();

        (top_pipe_height, bottom_pipe_height)
    }

    fn add_pipe(&mut self, x_pos: f32) {
        // Upper pipe
        let pipe_height = gen_range(
            MIN_PIPE_HEIGHT,
            screen_height() - MIN_PIPE_HEIGHT - MIN_VERTICAL_SPACE_BW_PIPES,
        );
        self.pipes.push(Pipe::new(vec2(x_pos, 0.0), pipe_height));

        // lower pipe
        let pipe_height = screen_height() - MIN_VERTICAL_SPACE_BW_PIPES - pipe_height;
        self.pipes.push(Pipe::new(
            vec2(x_pos, screen_height() - pipe_height),
            pipe_height,
        ));
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            pipe_manager: PipeManager::new(),
            parallax: ParallaxBackground::new(),
        }
    }

    pub fn update(&mut self) -> (Pipe, Pipe) {
        self.parallax.update();
        self.pipe_manager.update()
    }

    pub fn draw(&self) {
        self.parallax.draw();
        self.pipe_manager.draw();
    }

    pub fn reset(&mut self) {
        self.pipe_manager = PipeManager::new();
    }
}








########lib.rs
pub mod agent;
pub mod configs;
pub mod editor;
pub mod game;
pub mod nn;
pub mod resources;
pub mod simulation;

pub use configs::*;
pub use simulation::Simulation;





##########main.rs


use flappy_ai::resources::init_resources;
use flappy_ai::simulation::Statistics;
use macroquad::prelude::*;

use flappy_ai::editor::Editor;
use flappy_ai::*;

#[macroquad::main("Flappy AI")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as _);
    init_resources().await;

    let mut editor = Editor::new();
    let mut simulation = Simulation::new();
    let mut stats = Statistics::new();

    loop {
        clear_background(LIGHTGRAY);

        stats = simulation.update(&editor.settings).unwrap_or(stats);
        simulation.draw(&editor.settings);
        let gen_label = format!("Gen: {}", stats.generation_count);
        draw_text(&gen_label, screen_width() / 2.0 - 90.0, 150.0, 40.0, WHITE);

        editor.update();
        editor.draw(&stats);

        // Restart simulation
        if editor.settings.is_restart {
            editor.settings.is_restart = false;
            simulation = Simulation::new();
        }
        // Render skip
        if editor.settings.is_frame_skip {
            for _ in 0..10 {
                stats = simulation.update(&editor.settings).unwrap_or(stats);
            }
        }

        if is_key_pressed(KeyCode::Q) {
            break;
        }
        next_frame().await
    }
}




#############nn.rs
use macroquad::rand::gen_range;

use crate::*;

#[derive(Clone)]
pub struct Net {
    n_inputs: usize,
    layers: Vec<Layer>,
}

#[derive(Clone)]
struct Layer {
    nodes: Vec<Vec<f64>>,
}

impl Net {
    pub fn new(layer_sizes: Vec<usize>) -> Self {
        if layer_sizes.len() < 2 {
            panic!("Need at least 2 layers");
        }
        for &size in layer_sizes.iter() {
            if size < 1 {
                panic!("Empty layers not allowed");
            }
        }

        let mut layers = Vec::new();
        let first_layer_size = *layer_sizes.first().unwrap();
        let mut prev_layer_size = first_layer_size;

        for &layer_size in layer_sizes[1..].iter() {
            layers.push(Layer::new(layer_size, prev_layer_size));
            prev_layer_size = layer_size;
        }

        Self {
            layers,
            n_inputs: first_layer_size,
        }
    }

    pub fn predict(&self, inputs: &Vec<f64>) -> Vec<f64> {
        if inputs.len() != self.n_inputs {
            panic!("Bad input size");
        }

        let mut outputs = Vec::new();
        outputs.push(inputs.clone());
        for (layer_index, layer) in self.layers.iter().enumerate() {
            let layer_results = layer.predict(&outputs[layer_index]);
            outputs.push(layer_results);
        }

        outputs.pop().unwrap()
    }

    pub fn mutate(&mut self) {
        self.layers.iter_mut().for_each(|l| l.mutate());
    }
}

impl Layer {
    fn new(layer_size: usize, prev_layer_size: usize) -> Self {
        let mut nodes: Vec<Vec<f64>> = Vec::new();

        for _ in 0..layer_size {
            let mut node: Vec<f64> = Vec::new();
            for _ in 0..prev_layer_size + 1 {
                let random_weight: f64 = gen_range(-1.0f64, 1.0f64);
                node.push(random_weight);
            }
            nodes.push(node);
        }

        Self { nodes }
    }

    fn predict(&self, inputs: &Vec<f64>) -> Vec<f64> {
        let mut layer_results = Vec::new();
        for node in self.nodes.iter() {
            layer_results.push(self.sigmoid(self.dot_prod(&node, &inputs)));
        }

        layer_results
    }

    fn mutate(&mut self) {
        for n in self.nodes.iter_mut() {
            for val in n.iter_mut() {
                if gen_range(0.0, 1.0) >= BRAIN_MUTATION_RATE {
                    continue;
                }

                *val += gen_range(-BRAIN_MUTATION_VARIATION, BRAIN_MUTATION_VARIATION) as f64;
            }
        }
    }

    fn dot_prod(&self, node: &Vec<f64>, values: &Vec<f64>) -> f64 {
        let mut it = node.iter();
        let mut total = *it.next().unwrap();
        for (weight, value) in it.zip(values.iter()) {
            total += weight * value;
        }

        total
    }

    fn sigmoid(&self, y: f64) -> f64 {
        1f64 / (1f64 + (-y).exp())
    }
}




################ resources.rs

use macroquad::prelude::*;
use once_cell::sync::OnceCell;

use crate::*;

pub static RESOURCES: OnceCell<Resources> = OnceCell::new();

pub struct Resources {
    pub bird_texture: Texture2D,
    pub pipe_texture: Texture2D,
    pub pipe_head_texture: Texture2D,
    pub background_texture: Texture2D,
}

pub async fn init_resources() {
    let resources = Resources::new().await;
    match RESOURCES.set(resources) {
        Ok(_) => println!("Resources init successfull"),
        Err(_) => panic!("Failed to load Resources"),
    };
}

impl Resources {
    pub async fn new() -> Self {
        let bird_texture = load_texture(BIRD_TEXTURE).await.unwrap();
        bird_texture.set_filter(FilterMode::Nearest);
        let pipe_head_texture = load_texture(PIPE_HEAD_TEXTURE).await.unwrap();
        pipe_head_texture.set_filter(FilterMode::Nearest);
        let pipe_texture = load_texture(PIPE_TEXTURE).await.unwrap();
        pipe_texture.set_filter(FilterMode::Nearest);
        let background_texture = load_texture(BACKGROUND_TEXTURE).await.unwrap();
        background_texture.set_filter(FilterMode::Nearest);

        Self {
            bird_texture,
            pipe_texture,
            pipe_head_texture,
            background_texture,
        }
    }
}


############# simulation.rs

use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::thread_rng;

use crate::agent::Bird;
use crate::editor::Settings;
use crate::game::Game;
use crate::*;

pub struct Simulation {
    game: Game,
    birds: Vec<Bird>,
    generation_count: u32,
}

pub struct Statistics {
    pub generation_count: u32,
    pub birds_alive: u32,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            birds: (0..NUM_BIRDS).map(|_| Bird::new()).collect(),
            generation_count: 0,
        }
    }

    pub fn update(&mut self, settings: &Settings) -> Option<Statistics> {
        if settings.is_pause {
            return None;
        }

        let (top_pipe, bottom_pipe) = self.game.update();
        self.birds
            .iter_mut()
            .for_each(|a| a.update(&top_pipe, &bottom_pipe));

        let dead_count = self.birds.iter().filter(|b| b.is_dead).count();
        if dead_count == NUM_BIRDS {
            self.birds = self.selection();
            self.game.reset();
            self.generation_count += 1;
        }

        Some(Statistics {
            birds_alive: (NUM_BIRDS - dead_count) as u32,
            generation_count: self.generation_count,
        })
    }

    pub fn draw(&self, settings: &Settings) {
        if !settings.is_draw {
            return;
        }

        self.game.draw();
        if settings.show_one_bird {
            let alive_birds: Vec<&Bird> = self.birds.iter().filter(|b| !b.is_dead).collect();
            match alive_birds.first() {
                Some(bird) => bird.draw(),
                None => {}
            }
            return;
        }

        self.birds.iter().for_each(|b| b.draw());
    }

    fn selection(&self) -> Vec<Bird> {
        let mut rng = thread_rng();
        let gene_pool = self.calc_fitness();
        let mut new_birds = Vec::new();

        for _ in 0..NUM_BIRDS {
            let rand_bird = self.birds[gene_pool.sample(&mut rng)].clone();
            let mut new_bird = Bird::with_brain(&rand_bird);
            new_bird.mutate();
            new_birds.push(new_bird);
        }

        new_birds
    }

    fn calc_fitness(&self) -> WeightedIndex<f32> {
        let mut max_fitness = 0.0;
        let mut weights = Vec::new();

        for b in self.birds.iter() {
            if b.score > max_fitness {
                max_fitness = b.score;
            }
            weights.push(b.score);
        }
        weights
            .iter_mut()
            .for_each(|i| *i = (*i / max_fitness) * 100.0);

        WeightedIndex::new(&weights).expect("Failed to generate gene pool")
    }
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            generation_count: 0,
            birds_alive: 0,
        }
    }
}
