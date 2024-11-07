use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, KeyCode, Query, Res, Time};
use bevy::utils::HashSet;
use crate::components::*;
use crate::uVec3::uVec3;

pub fn setup_tilemap(mut commands: Commands){
    let mut tileMap: HashSet<uVec3> = HashSet::new();
    tileMap.insert(uVec3::new(0,0,0));
    tileMap.insert(uVec3::new(1,0,0));
    tileMap.insert(uVec3::new(2,0,0));
    tileMap.insert(uVec3::new(3,0,0));


    commands.spawn((CurrentTileMap2{
        running: false,
        current_state: tileMap.clone(),
    }));
    commands.spawn((CurrentTileMap{
        running: false,
        current_state: Arc::new(Mutex::new(tileMap)),
    }));
    commands.spawn((NewTileMap{
        new_state: Arc::new(Mutex::new(HashSet::new())),
    }));
}

pub fn run_simulation_multithread(
    mut current_tilemap_query: Query<&mut CurrentTileMap>,
    new_tilemap_query: Query<&NewTileMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
){
    let currentTileMap = current_tilemap_query.single().current_state.clone();
    let mut tileMap = current_tilemap_query.single_mut();


    if(keyboard_input.just_pressed(KeyCode::Space)){
        tileMap.running = !tileMap.running;
        if(tileMap.running){
            println!("Simulation started");
        } else {
            println!("Simulation stopped");
        }
    }

    if(!tileMap.running){
        return;
    }

    // let newTileMap = new_tilemap_query.single().new_state.clone();
    let mut tmIter = hsetCloner(currentTileMap.clone());
    let nbTiles = tmIter.len();
    println!("len : {}, time : {}", nbTiles, time.delta().as_secs_f64());
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for pos in tmIter.iter(){
        let clonedTileMap = currentTileMap.clone();
        // let clonedNewTileMap = newTileMap.clone();
        let a = pos.clone();
        // println!("{}, {}, {}", a.x, a.y, a.z);
        handles.push(thread::spawn(move ||{
            let mut myClonedTileMap = clonedTileMap.lock().unwrap();
            // println!("does tm contain {}, {}, {} : {}", a.x, a.y, a.z, myClonedTileMap.contains(&uVec3::new(a.x, a.y, a.z)));
            for i in 0..100{
                myClonedTileMap.insert(uVec3::new(a.x, a.y+1, a.z+i));
            }
        }));
    }
    for h in handles{
        h.join().unwrap();
    }
}

pub fn run_simulation(
    mut current_tilemap_query: Query<&mut CurrentTileMap2>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
){
    let mut tileMap = current_tilemap_query.single_mut();
    let mut newTileMap = tileMap.current_state.clone();


    if(keyboard_input.just_pressed(KeyCode::Space)){
        tileMap.running = !tileMap.running;
        if(tileMap.running){
            println!("Simulation started");
        } else {
            println!("Simulation stopped");
        }
    }

    if(!tileMap.running){
        return;
    }
    let nbTiles = tileMap.current_state.len();
    println!("len : {}, time : {}", nbTiles, time.delta().as_secs_f64());
    for pos in tileMap.current_state.iter(){
        for i in 0..100{
            newTileMap.insert(uVec3::new(pos.x, pos.y+1, pos.z+i));
        }
    }

    tileMap.current_state = newTileMap;
}

fn hsetCloner(tilemap:Arc<Mutex<HashSet<uVec3>>>)->HashSet<uVec3>{
    tilemap.lock().unwrap().clone()
}