use bevy::prelude::*;
use crate::ai::AIBird;

#[derive(Component)]
pub struct AIBirdMarker;

#[derive(Component)]
pub struct GenerationText;

#[derive(Component)]
pub struct PopulationText;

#[derive(Component)]
pub struct BestFitnessText;

#[derive(Component)]
pub struct AverageFitnessText;

#[derive(Component)]
pub struct AliveCountText;