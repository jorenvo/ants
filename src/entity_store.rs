use crate::components::*;
use crate::entities::*;
use std::collections::{BTreeMap, HashSet};

pub type EntityIndex = usize;
pub type PheromoneGenerationNr = u32;

#[derive(Default)]
pub struct EntityStore {
    pub new_index: EntityIndex,
    pub pheromone_generation: PheromoneGenerationNr,

    pub entity_types: BTreeMap<EntityIndex, EntityType>,

    // Entities
    pub ants: BTreeMap<EntityIndex, AntEntity>,
    pub pheromones: BTreeMap<EntityIndex, PheromoneEntity>,
    pub sugars: BTreeMap<EntityIndex, SugarEntity>,
    pub bases: BTreeMap<EntityIndex, BaseEntity>,
    pub walls: BTreeMap<EntityIndex, WallEntity>,

    // Components
    positions: BTreeMap<EntityIndex, PositionComponent>,
    positions_lookup: BTreeMap<CoarsePositionComponent, HashSet<EntityIndex>>,
    directions: BTreeMap<EntityIndex, DirectionComponent>,
    pub edibles: BTreeMap<EntityIndex, EdibleComponent>,
    pub releasing_pheromones: BTreeMap<EntityIndex, ReleasingPheromoneComponent>,
    pub pheromone_generations: BTreeMap<EntityIndex, PheromoneGenerationComponent>,
    pub intensities: BTreeMap<EntityIndex, IntensityComponent>,
    pub pheromone_types: BTreeMap<EntityIndex, PheromoneType>,
    pub carrying_food: BTreeMap<EntityIndex, CarryingFoodComponent>,
    pub builders: BTreeMap<EntityIndex, BuilderComponent>,
    pub impenetrables: BTreeMap<EntityIndex, ImpenetrableComponent>,
}

impl EntityStore {
    fn get_new_index(&mut self) -> EntityIndex {
        self.new_index += 1;
        self.new_index - 1
    }

    pub fn get_position(&self, id: &EntityIndex) -> Option<&PositionComponent> {
        self.positions.get(id)
    }

    pub fn get_direction(&self, id: &EntityIndex) -> Option<&DirectionComponent> {
        self.directions.get(id)
    }

    pub fn get_entities_at(&self, search_pos: &PositionComponent) -> Option<&HashSet<EntityIndex>> {
        self.positions_lookup
            .get(&CoarsePositionComponent::from(search_pos))
    }

    pub fn get_entities_with_type_at(
        &self,
        search_pos: &PositionComponent,
        entity_type: &EntityType,
    ) -> Option<HashSet<EntityIndex>> {
        if let Some(ids) = self.get_entities_at(search_pos) {
            let mut results = HashSet::new();
            for id in ids {
                if self.entity_types.get(&id).unwrap() == entity_type {
                    results.insert(*id);
                }
            }

            if !results.is_empty() {
                Some(results)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn pos_is_impenetrable(&self, pos: &PositionComponent) -> bool {
        if let Some(entities) = self.get_entities_at(pos) {
            entities
                .iter()
                .any(|id| self.impenetrables.get(&id).is_some())
        } else {
            false
        }
    }

    pub fn get_pheromone_with_type_at(
        &self,
        search_pos: &PositionComponent,
        pheromone_type: &PheromoneType,
    ) -> Option<EntityIndex> {
        if let Some(ph_ids) = self.get_entities_with_type_at(&search_pos, &EntityType::Pheromone) {
            let ph_id: Vec<EntityIndex> = ph_ids
                .into_iter()
                .filter(|id| self.pheromone_types.get(id).unwrap() == pheromone_type)
                .collect();

            if ph_id.is_empty() {
                None
            } else if ph_id.len() > 1 {
                panic!("More than one food pheromone in the same position!");
            } else {
                Some(ph_id[0])
            }
        } else {
            None
        }
    }

    pub fn update_position(&mut self, id: &EntityIndex, new_pos: &PositionComponent) {
        let old_pos = self.positions.get(&id);
        if let Some(old_pos) = old_pos {
            self.directions.insert(
                *id,
                DirectionComponent {
                    x: new_pos.x - old_pos.x,
                    y: new_pos.y - old_pos.y,
                },
            );

            if let Some(entities) = self
                .positions_lookup
                .get_mut(&CoarsePositionComponent::from(old_pos))
            {
                entities.remove(&id);

                if entities.is_empty() {
                    self.positions_lookup
                        .remove(&CoarsePositionComponent::from(old_pos));
                }
            }
        }

        self.positions.insert(*id, new_pos.clone());

        if self
            .positions_lookup
            .get(&CoarsePositionComponent::from(new_pos))
            .is_none()
        {
            self.positions_lookup
                .insert(CoarsePositionComponent::from(new_pos), HashSet::new());
        }

        self.positions_lookup
            .get_mut(&CoarsePositionComponent::from(new_pos))
            .unwrap()
            .insert(*id);
    }

    pub fn remove_position(&mut self, id: &EntityIndex) {
        if let Some(pos) = self.get_position(&id) {
            let cloned_pos = pos.clone();
            let entities = self
                .positions_lookup
                .get_mut(&CoarsePositionComponent::from(&cloned_pos))
                .unwrap();
            entities.remove(&id);

            if entities.is_empty() {
                self.positions_lookup
                    .remove(&CoarsePositionComponent::from(&cloned_pos));
            }
        }

        self.positions.remove(&id);
        self.directions.remove(&id);
    }

    pub fn create_entity(&mut self, entity_type: &EntityType) -> EntityIndex {
        let index = self.get_new_index();
        match entity_type {
            EntityType::Ant => {
                self.update_position(&index, &PositionComponent::default());
                self.ants.insert(index, AntEntity {});
            }
            EntityType::Pheromone => {
                self.update_position(&index, &PositionComponent::default());
                self.pheromones.insert(index, PheromoneEntity {});
            }
            EntityType::Sugar => {
                self.update_position(&index, &PositionComponent { x: 8.0, y: 8.0 });
                self.edibles.insert(index, EdibleComponent::default());
                self.sugars.insert(index, SugarEntity {});
            }
            EntityType::Base => {
                self.update_position(&index, &PositionComponent::default());
                self.bases.insert(index, BaseEntity {});
            }
            EntityType::Wall => {
                self.update_position(&index, &PositionComponent::default());
                self.impenetrables
                    .insert(index, ImpenetrableComponent::default());
                self.walls.insert(index, WallEntity {});
            }
        }
        self.entity_types.insert(index, entity_type.clone());

        index
    }
}
