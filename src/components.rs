use crate::entities::*;
use crate::entity_store::*;
use crate::utils::*;
use std::cmp::Ordering;

#[derive(Clone, Debug, Default)]
pub struct PositionComponent {
    pub x: f64,
    pub y: f64,
}

impl PartialEq for PositionComponent {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for PositionComponent {}

impl PartialOrd for PositionComponent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PositionComponent {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering_y = cmp_float(self.y, other.y);

        if ordering_y == Ordering::Equal {
            cmp_float(self.x, other.x)
        } else {
            ordering_y
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CoarsePositionComponent {
    pub x: u64,
    pub y: u64,
}

impl PartialEq for CoarsePositionComponent {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for CoarsePositionComponent {}

impl PartialOrd for CoarsePositionComponent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CoarsePositionComponent {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering_y = self.y.cmp(&other.y);

        if ordering_y == Ordering::Equal {
            self.x.cmp(&other.x)
        } else {
            ordering_y
        }
    }
}

impl From<PositionComponent> for CoarsePositionComponent {
    fn from(pos: PositionComponent) -> Self {
        CoarsePositionComponent {
            x: pos.x.floor() as u64,
            y: pos.y.floor() as u64,
        }
    }
}

impl From<&PositionComponent> for CoarsePositionComponent {
    fn from(pos: &PositionComponent) -> Self {
        CoarsePositionComponent {
            x: pos.x.floor() as u64,
            y: pos.y.floor() as u64,
        }
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct EdibleComponent {}

#[derive(PartialEq, Debug)]
pub struct PheromoneTypeComponent {
    pub ph_type: PheromoneType,
}

#[derive(PartialEq, Debug, Default)]
pub struct IntensityComponent {
    pub strength: u32,
}

#[derive(PartialEq, Debug)]
pub struct ReleasingPheromoneComponent {
    pub ticks_left: u32,
    pub ph_type: PheromoneType,
}

#[derive(PartialEq, Debug)]
pub struct DirectionComponent {
    pub x: f64,
    pub y: f64,
}

#[derive(PartialEq, Debug)]
pub struct SensedPheromoneComponent {
    pub id: EntityIndex,
}
