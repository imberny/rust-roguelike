use std::f32::consts::PI;

use rltk::Point;

use crate::core::types::Facing;

#[derive(Debug, Clone, Copy)]
pub struct FieldOfView {
    range: u32,
    is_directed: bool,
}

impl FieldOfView {
    pub fn infinite() -> Self {
        Self {
            range: u32::MAX,
            is_directed: false,
        }
    }

    pub fn new_omni(range: u32) -> Self {
        Self {
            range,
            is_directed: false,
        }
    }

    pub fn new_directed(range: u32) -> Self {
        Self {
            range,
            is_directed: true,
        }
    }

    pub fn sees_around(&self, from: Point, to: Point) -> bool {
        rltk::DistanceAlg::Chebyshev.distance2d(from, to) <= self.range as f32
    }

    pub fn sees_towards(&self, from: Point, to: Point, towards: Facing) -> bool {
        let direction = (to - from).to_vec2();
        let distance = rltk::DistanceAlg::Chebyshev.distance2d(from, to);
        let angle = direction.normalized().dot(towards.to_vec2()).acos();
        if angle.is_nan() {
            return true;
        }
        angle.abs() <= PI / 4.0 && distance as u32 <= self.range
    }
}

#[cfg(test)]
mod tests {
    use rltk::Point;

    use crate::core::constants::facings;

    use super::FieldOfView;

    #[test]
    fn infinite_fov() {
        let fov = FieldOfView::infinite();

        let origin = Point::new(0, 0);
        let targets = vec![
            Point::new(0, -100_000),
            Point::new(100_000, 0),
            Point::new(0, 100_000),
            Point::new(-100_000, 0),
            Point::new(0, 0),
            Point::new(100_000, 100_000),
        ];

        for target in targets {
            let is_seen = fov.sees_around(origin, target);
            assert!(is_seen);
        }
    }

    #[test]
    fn tiny_fov() {
        let fov = FieldOfView::new_omni(1);

        let origin = Point::new(0, 0);
        let far_targets = vec![
            Point::new(0, -100_000),
            Point::new(100_000, 0),
            Point::new(0, 100_000),
            Point::new(-100_000, 0),
            Point::new(100_000, 100_000),
        ];

        let near_targets = vec![Point::new(0, 0), Point::new(1, 1), Point::new(-1, -1)];

        for target in far_targets {
            let is_seen = fov.sees_around(origin, target);
            assert!(!is_seen);
        }

        for target in near_targets {
            let is_seen = fov.sees_around(origin, target);
            assert!(is_seen);
        }
    }

    #[test]
    fn directed_fov() {
        let fov = FieldOfView::new_directed(5);
        let origin = Point::new(0, 0);
        let facing = facings::NORTH;

        let north_targets = vec![Point::new(0, 0), Point::new(1, -1), Point::new(0, -3)];
        for target in north_targets {
            let is_seen = fov.sees_towards(origin, target, facing);
            assert!(is_seen);
        }

        let targets_along_diagonal_nw = vec![
            Point::new(-1, -1),
            Point::new(-2, -2),
            Point::new(-3, -3),
            Point::new(-4, -4),
            Point::new(-5, -5),
        ];

        for target in targets_along_diagonal_nw {
            let is_seen = fov.sees_towards(origin, target, facing);
            assert!(is_seen);
        }

        let is_seen = fov.sees_towards(origin, Point::new(-6, -6), facing);
        assert!(!is_seen);
    }
}
