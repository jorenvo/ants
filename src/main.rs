#![deny(clippy::pedantic)]
extern crate rand;

use rand::Rng;
use std::collections::HashMap;
use std::fmt;

enum EntityType {
    Ant,
}

struct AntEntity {
    id: EntityIndex,
}

impl AntEntity {
    fn move_(&self, state: &mut WorldState) {
        let mut rng = rand::thread_rng();
        if let Some(pos) = state.positions.get_mut(&self.id) {
            pos.x += rng.gen_range(0, 10);
            pos.y += rng.gen_range(0, 10);
        }
    }
}

struct PositionComponent {
    x: u32,
    y: u32,
}

type EntityIndex = usize;

struct WorldState {
    new_index: EntityIndex,
    positions: HashMap<EntityIndex, PositionComponent>,
    ants: HashMap<EntityIndex, AntEntity>,
}

impl WorldState {
    fn init() -> Self {
        Self {
            new_index: 0,
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
                self.ants.insert(index, AntEntity { id: index });
            }
        }

        index
    }

    fn move_entities(&mut self) {
        for (ant_id, ant) in &mut self.ants {
            ant.move_(self);
        }
    }
}

impl fmt::Display for WorldState {
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

fn main() {
    let mut state = WorldState::init();
    for _ in 0..4 {
        state.create_entity(&EntityType::Ant);
    }
    state.move_entities();

    println!("{}", state);
}
