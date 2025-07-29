use bevy::prelude::*;
use plugin::MyPlugin;
use setup::setup;

mod components;
mod constants;
mod plugin;
mod setup;
mod utils;
 
fn main() {
    App::new().add_systems(Startup, setup).add_plugins(MyPlugin).run();
}