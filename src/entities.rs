#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EntityType {
    Ant,
    Pheromone,
    Sugar,
}

pub struct AntEntity {}
pub struct PheromoneEntity {}
pub struct SugarEntity {}
