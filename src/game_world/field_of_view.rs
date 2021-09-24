use std::f32::consts::PI;

use ultraviolet as uv;

use crate::core::types::{Facing, Position};

const ORIGIN: Position = Position::constant(0, 0);

pub trait FieldOfView {
    fn sees(&self, delta_position: Position) -> bool;
}

#[derive(Debug, Clone, Copy)]
struct ConeFOV {
    range: u32,
    // angle: f32,
    facing: Facing,
}

#[derive(Debug, Clone, Copy)]
struct QuadraticFOV {
    range: u32,
    facing: Facing,
    a: f32,
    b: f32,
}

#[derive(Debug, Clone, Copy)]
struct OmniFOV {
    range: u32,
}

#[allow(dead_code)]
pub fn new_infinite() -> impl FieldOfView {
    OmniFOV { range: u32::MAX }
}

#[allow(dead_code)]
pub fn new_omni(range: u32) -> impl FieldOfView {
    OmniFOV { range }
}

#[allow(dead_code)]
pub fn new_cone(range: u32, _angle: f32, facing: Facing) -> impl FieldOfView {
    ConeFOV {
        range,
        // angle,
        facing,
    }
}

pub fn new_quadratic(range: u32, facing: Facing, a: f32, b: f32) -> impl FieldOfView {
    QuadraticFOV {
        range,
        facing,
        a,
        b,
    }
}

impl FieldOfView for OmniFOV {
    fn sees(&self, to: Position) -> bool {
        rltk::DistanceAlg::Chebyshev.distance2d(ORIGIN.into(), to.into()) <= self.range as f32
    }
}

impl FieldOfView for ConeFOV {
    fn sees(&self, to: Position) -> bool {
        let direction = to.as_vec2() - ORIGIN.as_vec2();
        let distance = rltk::DistanceAlg::Chebyshev.distance2d(ORIGIN.into(), to.into());
        let angle = direction
            .normalized()
            .dot(self.facing * uv::Vec2::new(0.0, 1.0))
            .acos();
        if angle.is_nan() {
            return true;
        }
        angle.abs() <= PI / 4.0 && distance as u32 <= self.range
    }
}

impl FieldOfView for QuadraticFOV {
    fn sees(&self, to: Position) -> bool {
        let distance = rltk::DistanceAlg::Chebyshev.distance2d(ORIGIN.into(), to.into()) as u32;
        let to_vec2 = to.as_vec2();
        let target = self.facing * to_vec2;
        let curve_line = (target.x * self.a).powi(2) + self.b;

        // let angle = to_vec2
        //     .normalized()
        //     .dot(facing.reversed() * Vec2::new(0.0, 1.0))
        //     .acos();
        // if angle.is_nan() {
        //     return to_vec2.eq(&Vec2::new(0.0, 0.0));
        // }

        // (angle.abs() <= PI / 4.0 || curve_line < target.y)
        curve_line < target.y && distance <= self.range
    }
}

#[cfg(test)]
mod tests {

    use std::f32::consts::PI;

    use crate::{
        core::{constants::*, types::Position},
        game_world::field_of_view::FieldOfView,
    };

    use super::{new_cone, new_infinite, new_omni, new_quadratic};

    #[test]
    fn infinite_fov() {
        let fov = new_infinite();

        let targets = vec![
            Position::new(0, -100_000),
            Position::new(100_000, 0),
            Position::new(0, 100_000),
            Position::new(-100_000, 0),
            Position::new(0, 0),
            Position::new(100_000, 100_000),
        ];

        for target in targets {
            let is_seen = fov.sees(target);
            assert!(is_seen);
        }
    }

    #[test]
    fn tiny_fov() {
        let fov = new_omni(1);

        let far_targets = vec![
            Position::new(0, -100_000),
            Position::new(100_000, 0),
            Position::new(0, 100_000),
            Position::new(-100_000, 0),
            Position::new(100_000, 100_000),
        ];

        let near_targets = vec![
            Position::new(0, 0),
            Position::new(1, 1),
            Position::new(-1, -1),
        ];

        for target in far_targets {
            let is_seen = fov.sees(target);
            assert!(!is_seen);
        }

        for target in near_targets {
            let is_seen = fov.sees(target);
            assert!(is_seen);
        }
    }

    #[test]
    fn directed_fov() {
        let fov = new_cone(5, PI / 2.0, NORTH);

        let north_targets = vec![
            Position::new(0, 0),
            Position::new(1, -1),
            Position::new(0, -3),
        ];
        for target in north_targets {
            let is_seen = fov.sees(target);
            assert!(is_seen);
        }

        for target in targets_along_diagonal_nw() {
            let is_seen = fov.sees(target);
            assert!(is_seen);
        }

        let is_seen = fov.sees(Position::new(-6, -6));
        assert!(!is_seen);
    }

    #[test]
    fn view_curve() {
        let fov = new_quadratic(5, NORTH, 0.5, -1.5);

        let north_targets = vec![
            Position::new(0, 0),
            Position::new(1, -1),
            Position::new(0, -3),
        ];
        for target in north_targets {
            let is_seen = fov.sees(target);
            assert!(is_seen);
        }

        for target in targets_along_diagonal_nw() {
            let is_seen = fov.sees(target);
            assert!(is_seen);
        }

        for target in targets_behind_facing_north() {
            let is_seen = fov.sees(target);
            assert!(is_seen);
        }

        for target in targets_side_facing_north() {
            let is_seen = fov.sees(target);
            assert!(is_seen);
        }
    }

    #[test]
    fn view_curve_east() {
        let fov = new_quadratic(5, EAST, 0.5, -1.5);

        for target in targets_facing_east() {
            let is_seen = fov.sees(target);
            assert!(is_seen);
        }
    }

    #[test]
    fn view_curve_west() {
        let fov = new_quadratic(5, WEST, 0.5, -1.5);

        for target in targets_facing_west() {
            let is_seen = fov.sees(target);
            assert!(is_seen);
        }

        for target in targets_facing_west_hidden() {
            let is_seen = fov.sees(target);
            assert!(!is_seen);
        }
    }

    fn targets_behind_facing_north() -> Vec<Position> {
        vec![
            Position::new(-1, 1),
            Position::new(0, 1),
            Position::new(1, 1),
        ]
    }

    fn targets_side_facing_north() -> Vec<Position> {
        vec![
            Position::new(1, 0),
            Position::new(-1, 0),
            Position::new(2, 0),
            Position::new(-2, 0),
            Position::new(-2, -1),
            Position::new(2, -1),
        ]
    }

    fn targets_facing_east() -> Vec<Position> {
        vec![
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(1, -1),
            Position::new(2, 2),
            Position::new(2, -2),
            Position::new(3, -3),
        ]
    }

    fn targets_facing_west() -> Vec<Position> {
        vec![
            Position::new(1, -1),
            Position::new(1, 0),
            Position::new(1, 1),
            Position::new(0, 1),
            Position::new(0, 2),
            Position::new(-2, -2),
            Position::new(-2, -1),
            Position::new(-2, 0),
            Position::new(-2, 1),
            Position::new(-2, 2),
        ]
    }

    fn targets_facing_west_hidden() -> Vec<Position> {
        vec![
            Position::new(1, -2),
            Position::new(2, -1),
            Position::new(2, 0),
            Position::new(2, 1),
            Position::new(1, 2),
            Position::new(-6, 0),
        ]
    }

    fn targets_along_diagonal_nw() -> Vec<Position> {
        vec![
            Position::new(-1, -1),
            Position::new(-2, -2),
            Position::new(-3, -3),
            Position::new(-4, -4),
            Position::new(-5, -5),
        ]
    }
}
