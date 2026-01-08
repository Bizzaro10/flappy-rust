use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::resources::{SimulationState, GameMode};
use crate::components::Bird;

#[derive(Resource)]
pub struct UiState {
    pub show_one_bird: bool,
    pub fast_mode: bool,
    pub show_ui: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            show_one_bird: false,
            fast_mode: false,
            show_ui: true,
        }
    }
}

pub fn ui_system(
    mut contexts: EguiContexts,
    sim_state: Res<SimulationState>,
    mut ui_state: ResMut<UiState>,
    mut bird_query: Query<(&mut Visibility, &Bird)>,
    mut time: ResMut<Time<Virtual>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        ui_state.show_ui = !ui_state.show_ui;
    }
    
    if !ui_state.show_ui { return; }

    egui::Window::new("Flappy AI")
        .default_pos(egui::pos2(10.0, 10.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Stats");
            ui.label(format!("Generation: {}", sim_state.generation));
            ui.label(format!("Alive: {}", sim_state.birds_alive));
            ui.label(format!("Mode: {:?}", sim_state.mode));

            ui.separator();

            ui.heading("Options");
            if ui.checkbox(&mut ui_state.show_one_bird, "Show One Bird").changed() {
                // If unchecked, ensure all visible
                if !ui_state.show_one_bird {
                    for (mut vis, _) in bird_query.iter_mut() {
                        *vis = Visibility::Visible; 
                    }
                }
            }
            
            if ui.checkbox(&mut ui_state.fast_mode, "Fast Mode (5x Speed)").changed() {
                 if ui_state.fast_mode {
                     time.set_relative_speed(5.0);
                 } else {
                     time.set_relative_speed(1.0);
                 }
            }

            ui.separator();
            ui.label("Controls:");
            ui.label("M: Toggle Mode");
            ui.label("Esc: Toggle UI");
        });

    // Handle "Show One Bird" logic per frame if active
    if ui_state.show_one_bird && sim_state.mode == GameMode::AI {
         // Show only the first living bird
         let mut found_one = false;
         for (mut vis, bird) in bird_query.iter_mut() {
             if !found_one && !bird.is_dead {
                 *vis = Visibility::Visible;
                 found_one = true;
             } else {
                 *vis = Visibility::Hidden;
             }
         }
    }
}
