use bevy::math::IVec2;
pub use bevy::prelude::Component;

use super::AreaGrid;

#[derive(Debug)]
pub struct OffsetArea(pub IVec2, pub AreaGrid);

#[derive(Debug, Default, Component)]
pub struct WorldMap {
    areas: Vec<OffsetArea>,
}

impl WorldMap {
    pub fn insert_offset(&mut self, offset: &IVec2, area_grid: AreaGrid) {
        self.areas.push(OffsetArea(*offset, area_grid));
    }

    pub fn get_area_from_pos(&self, pos: &IVec2) -> Option<&OffsetArea> {
        self.areas.iter().find(|offset_area| {
            let OffsetArea(offset, area) = offset_area;
            area.tile_at(&(*pos - *offset)).is_some()
        })
    }

    pub fn get_area_from_pos_mut(&mut self, pos: &IVec2) -> Option<&mut OffsetArea> {
        self.areas.iter_mut().find(|offset_area| {
            let OffsetArea(offset, area) = offset_area;
            area.tile_at(&(*pos - *offset)).is_some()
        })
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::IVec2;

    use crate::world::AreaGrid;

    use super::WorldMap;

    #[test]
    fn get_area_from_pos() {
        let mut world_map = WorldMap::default();
        world_map.insert_offset(&IVec2::ZERO, AreaGrid::new(&IVec2::new(15, 15)));
        let area = world_map.get_area_from_pos(&IVec2::new(0, 0));
        assert!(area.is_some());

        let area = world_map.get_area_from_pos(&IVec2::new(16, 16));
        assert!(area.is_none());

        world_map.insert_offset(&IVec2::new(15, 15), AreaGrid::new(&IVec2::new(10, 12)));
        let area = world_map.get_area_from_pos(&IVec2::new(16, 16));
        assert!(area.is_some());
    }

    #[test]
    fn get_offset_area_from_pos() {
        let mut world_map = WorldMap::default();
        world_map.insert_offset(&IVec2::new(10, 10), AreaGrid::new(&IVec2::new(15, 15)));
        let area = world_map.get_area_from_pos(&IVec2::ZERO);
        assert!(area.is_none());
    }
}
