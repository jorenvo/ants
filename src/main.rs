#![deny(clippy::pedantic)]
extern crate rand;

mod components;

use components::*;
use rand::Rng;
use std::collections::HashMap;
use std::fmt;


#[derive(Clone)]
enum EntityType {
    Ant,
}

struct AntEntity {}

type EntityIndex = usize;

struct EntityStore {
    new_index: EntityIndex,
    types: HashMap<EntityIndex, EntityType>,
    positions: HashMap<EntityIndex, PositionComponent>,
    // TODO: reverse_positions
    ants: HashMap<EntityIndex, AntEntity>,
}

impl EntityStore {
    fn init() -> Self {
        Self {
            new_index: 0,
            types: HashMap::new(),
            positions: HashMap::new(),
            ants: HashMap::new(),
        }
    }

    fn get_new_index(&mut self) -> EntityIndex {
        self.new_index += 1;
        self.new_index - 1
    }

    fn create_entity(&mut self, entity_type: &EntityType) -> EntityIndex {
        let index = self.get_new_index();
        match entity_type {
            EntityType::Ant => {
                self.positions
                    .insert(index, PositionComponent { x: 0, y: 0 });
                self.ants.insert(index, AntEntity {});
            }
        }
        self.types.insert(index, entity_type.clone());

        index
    }
}

impl fmt::Display for EntityStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--- ANTS ---")?;
        for ant_id in self.ants.keys() {
            if let Some(pos) = self.positions.get(ant_id) {
                writeln!(f, "id {} at {}, {}", ant_id, pos.x, pos.y)?;
            } else {
                writeln!(f, "id {} has no position!", ant_id)?;
            }
        }

        Ok(())
    }
}

struct Game {
    entity_store: EntityStore,
    rng: rand::rngs::ThreadRng,
}

impl Game {
    fn move_ants(&mut self) {
        let ants = &self.entity_store.ants;
        for (id, _) in ants {
            if let Some(pos) = self.entity_store.positions.get_mut(id) {
                pos.x += self.rng.gen_range(0, 10);
                pos.y += self.rng.gen_range(0, 10);
            }
        }
    }

    fn move_entities(&mut self) {
        self.move_ants();
    }
}

fn main() {
    let mut game = Game {
        entity_store: EntityStore::init(),
        rng: rand::thread_rng(),
    };
    for _ in 0..4 {
        game.entity_store.create_entity(&EntityType::Ant);
    }

    game.move_entities();

    println!("{}", game.entity_store);
}
