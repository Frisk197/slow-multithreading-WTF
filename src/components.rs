use std::sync::{Arc, Mutex};
use bevy::prelude::Component;
use bevy::utils::HashSet;
use crate::uVec3::uVec3;

#[derive(Component)]
pub struct CurrentTileMap{
    pub running: bool,
    pub current_state: Arc<Mutex<HashSet<uVec3>>>,

}

#[derive(Component)]
pub struct CurrentTileMap2{
    pub running: bool,
    pub current_state: HashSet<uVec3>,

}

#[derive(Component)]
pub struct NewTileMap{
    pub new_state: Arc<Mutex<HashSet<uVec3>>>
}