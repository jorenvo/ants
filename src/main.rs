#![deny(clippy::pedantic)]
extern crate rand;

use rand::Rng;
use std::collections::HashMap;
use std::fmt;

enum EntityType {
    Ant,
}

struct PositionComponent {
    x: u32,
    y: u32,
}

type EntityIndex = usize;

struct WorldState {
    new_index: EntityIndex,
    positions: HashMap<EntityIndex, PositionComponent>,
    ants: Vec<EntityIndex>,
}

impl WorldState {
    fn init() -> Self {
        Self {
            new_index: 0,
            positions: HashMap::new(),
            ants: Vec::new(),
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
                self.ants.push(index);
            }
        }

        index
    }

    fn move_entities(&mut self) {
        let mut rng = rand::thread_rng();
        for ant_id in &self.ants {
            if let Some(pos) = self.positions.get_mut(ant_id) {
                pos.x += rng.gen_range(0, 10);
                pos.y += rng.gen_range(0, 10);
            }
        }
    }
}

impl fmt::Display for WorldState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--- ANTS ---")?;
        for ant_id in &self.ants {
            let pos = &self.positions[ant_id];
            writeln!(f, "id {} at {}, {}", ant_id, pos.x, pos.y)?;
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
