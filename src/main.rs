#![deny(clippy::pedantic)]
#![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
extern crate rand;

mod components;
mod entities;
mod entity_store;
mod game;
mod utils;

use components::*;
use entities::*;
use entity_store::*;
use game::*;

fn add_deneubourg_walls(game: &mut Game) {
    let mut index;
    let mut y;
    for i in 0..5 {
        index = game.entity_store.create_entity(&EntityType::Wall);
        y = if i == 0 || i == 4 { 1.5 } else { 0.5 };
        game.entity_store
            .update_position(&index, &PositionComponent { x: i as f64, y: y });

        index = game.entity_store.create_entity(&EntityType::Wall);
        y = if i == 0 || i == 4 { 3.5 } else { 4.5 };
        game.entity_store
            .update_position(&index, &PositionComponent { x: i as f64, y: y })
    }

    // corners
    index = game.entity_store.create_entity(&EntityType::Wall);
    game.entity_store
        .update_position(&index, &PositionComponent { x: 0.5, y: 0.5 });
    index = game.entity_store.create_entity(&EntityType::Wall);
    game.entity_store
        .update_position(&index, &PositionComponent { x: 4.5, y: 0.5 });
    index = game.entity_store.create_entity(&EntityType::Wall);
    game.entity_store
        .update_position(&index, &PositionComponent { x: 0.5, y: 4.5 });
    index = game.entity_store.create_entity(&EntityType::Wall);
    game.entity_store
        .update_position(&index, &PositionComponent { x: 4.5, y: 4.5 });

    // middle
    index = game.entity_store.create_entity(&EntityType::Wall);
    game.entity_store
        .update_position(&index, &PositionComponent { x: 2.5, y: 2.5 });
}

fn main() {
    let mut game = Game::init(EntityStore::default(), 5.0, 5.0);

    for _ in 0..1 {
        game.entity_store.create_entity(&EntityType::Ant);
    }

    game.entity_store.create_entity(&EntityType::Base);
    game.entity_store.create_entity(&EntityType::Sugar);

    add_deneubourg_walls(&mut game);

    for i in 0..300 {
        println!("Tick #{}\n{}", i, game);
        game.tick();
    }
}
