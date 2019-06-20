#![deny(clippy::pedantic)]
#![allow(dead_code)]
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

    fn move_entity(&mut self, index: EntityIndex, position: PositionComponent) {
        self.positions.insert(index, position);
    }

    fn get_position(&self, i: &EntityIndex) -> Option<&PositionComponent> {
        self.positions.get(i)
    }
}

impl fmt::Display for WorldState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for ant_id in &self.ants {
            let pos = &self.positions[ant_id];
            write!(f, "ant (ID: {}) at {}, {}", ant_id, pos.x, pos.y)?;
        }

        Ok(())
    }
}

fn main() {
    let mut state = WorldState::init();
    state.create_entity(&EntityType::Ant);
    println!("{}", state);
}
