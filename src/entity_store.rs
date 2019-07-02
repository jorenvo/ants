use crate::components::*;
use crate::entities::*;
use std::collections::{BTreeMap, HashSet};
use std::fmt;

type EntityIndex = usize;

#[derive(Default)]
pub struct EntityStore {
    pub new_index: EntityIndex,
    pub types: BTreeMap<EntityIndex, EntityType>,
    pub positions: BTreeMap<EntityIndex, PositionComponent>,
    pub positions_lookup: BTreeMap<PositionComponent, HashSet<EntityIndex>>,
    pub edibles: BTreeMap<EntityIndex, EdibleComponent>,
    pub ants: BTreeMap<EntityIndex, AntEntity>,
    pub pheromones: BTreeMap<EntityIndex, PheromoneEntity>,
    pub sugars: BTreeMap<EntityIndex, SugarEntity>,
}

impl EntityStore {
    fn get_new_index(&mut self) -> EntityIndex {
        self.new_index += 1;
        self.new_index - 1
    }

    fn update_position(
        &mut self,
        id: EntityIndex,
        old_pos: Option<&PositionComponent>,
        new_pos: &PositionComponent,
    ) {
        if let Some(old_pos) = old_pos {
            if let Some(entities) = self.positions_lookup.get_mut(&old_pos) {
                entities.remove(&id);
            }
        }

        self.positions.insert(id, new_pos.clone());

        if self.positions_lookup.get(new_pos).is_none() {
            self.positions_lookup
                .insert(new_pos.clone(), HashSet::new());
        }

        self.positions_lookup.get_mut(new_pos).unwrap().insert(id);
    }

    pub fn create_entity(&mut self, entity_type: &EntityType) -> EntityIndex {
        let index = self.get_new_index();
        match entity_type {
            EntityType::Ant => {
                self.update_position(index, None, &PositionComponent::default());
                self.ants.insert(index, AntEntity {});
            }
            EntityType::Pheromone => {
                self.update_position(index, None, &PositionComponent::default());
                self.pheromones.insert(index, PheromoneEntity {});
            }
            EntityType::Sugar => {
                self.update_position(index, None, &PositionComponent { x: 10, y: 10 });
                self.edibles.insert(index, EdibleComponent::default());
                self.sugars.insert(index, SugarEntity {});
            }
        }
        self.types.insert(index, entity_type.clone());

        index
    }

    pub fn get_pheromone_at(&self, search_pos: &PositionComponent) -> Option<EntityIndex> {
        // TODO this should probably return Vec<EntityIndex>
        if let Some(entities_at_pos) = self.positions_lookup.get(search_pos) {
            for id in entities_at_pos {
                if self.pheromones.get(&id).is_some() {
                    return Some(*id);
                }
            }
        }

        None
    }
}

impl fmt::Display for EntityStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.new_index {
            if self.ants.get(&i).is_some() {
                if let Some(pos) = self.positions.get(&i) {
                    writeln!(f, "ant {} at {}, {}", i, pos.x, pos.y)?;
                } else {
                    writeln!(f, "ant {} has no position!", i)?;
                }
            } else if self.pheromones.get(&i).is_some() {
                if let Some(pos) = self.positions.get(&i) {
                    writeln!(f, "pheromone {} at {}, {}", i, pos.x, pos.y)?;
                } else {
                    writeln!(f, "pheromone {} has no position!", i)?;
                }
            } else if self.sugars.get(&i).is_some() {
                if let Some(pos) = self.positions.get(&i) {
                    writeln!(f, "sugar {} at {}, {}", i, pos.x, pos.y)?;
                } else {
                    writeln!(f, "sugar {} has no position!", i)?;
                }
            } else {
                writeln!(f, "unknown entity {}!", i)?;
            }
        }

        Ok(())
    }
}