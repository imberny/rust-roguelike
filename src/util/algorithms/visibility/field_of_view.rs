use std::ops::Sub;

use crate::{
    core::{
        constants::PI,
        types::{Facing, GridPos, Int, IntoGridPos, Real, RealPos},
    },
    util::algorithms::distance::chebyshev_distance,
};

const ORIGIN: GridPos = GridPos { x: 0, y: 0 };

pub trait FieldOfView {
    fn sees(&self, delta_position: GridPos) -> bool;
}

#[derive(Debug, Clone, Copy)]
struct ConeFOV {
    range: Int,
    angle: Real,
    facing: Facing,
}

#[derive(Debug, Clone, Copy)]
struct QuadraticFOV {
    range: Int,
    facing: Facing,
    a: Real,
    b: Real,
}

#[derive(Debug, Clone, Copy)]
struct OmniFOV {
    range: Int,
}

#[allow(dead_code)]
pub fn infinite_fov() -> impl FieldOfView {
    OmniFOV { range: Int::MAX }
}

#[allow(dead_code)]
pub fn omnidirectional_fov(range: Int) -> impl FieldOfView {
    OmniFOV { range: range.abs() }
}

#[allow(dead_code)]
pub fn cone_fov(range: Int, angle: Real, facing: Facing) -> impl FieldOfView {
    ConeFOV {
        range,
        angle,
        facing,
    }
}

#[allow(dead_code)]
pub fn quadratic_fov_default(range: Int, facing: Facing) -> impl FieldOfView {
    QuadraticFOV {
        range,
        facing,
        a: 0.5,
        b: -1.5,
    }
}

pub fn quadratic_fov(range: Int, facing: Facing, a: Real, b: Real) -> impl FieldOfView {
    QuadraticFOV {
        range,
        facing,
        a,
        b,
    }
}

struct PatternFOV {
    pattern: Vec<GridPos>,
    facing: Facing,
}

fn project_angle(start: GridPos, radius: f32, angle_radians: f32) -> GridPos {
    let degrees_radians = angle_radians + std::f32::consts::PI;
    GridPos::new(
        (0.0 - (start.x as f32 + radius * f32::sin(degrees_radians))) as i32,
        (start.y as f32 + radius * f32::cos(degrees_radians)) as i32,
    )
}

impl FieldOfView for PatternFOV {
    fn sees(&self, delta_position: GridPos) -> bool {
        let dist = chebyshev_distance(&GridPos::zero(), &delta_position);

        // let target = (self.facing * RealPos::from(delta_position));
        // this approach doesn't work, many pos map to same pos
        let dir: RealPos = delta_position.into();
        let target = (self.facing * dir.normalized()).round() * dist;

        // let mut target = delta_position.clone();

        // let mut rot_target = GridPos::new(target.y, -target.x);

        // let x_delta = (target.x - rot_target.x).abs();
        // let y_delta = (target.y - rot_target.y).abs();

        // let max = std::cmp::max(target.x.abs(), target.y.abs());
        // let min = std::cmp::min(target.x.abs(), target.y.abs());

        // if x_delta < y_delta {
        //     target.y += ((x_delta + y_delta) as Real / 2.0) as Int;
        // } else {
        //     target.x += ((x_delta + y_delta) as Real / 2.0) as Int;
        // }
        // // target.x += (target.x.abs() + target.y.abs()) / 2;

        // let radius = chebyshev_distance(&GridPos::zero(), &target) as Real;
        // target = project_angle(target, 4.0, -PI / 4.0);

        let result = self.pattern.contains(&target);
        let result2 = result;
        result2
    }
}

pub fn pattern_fov(pattern: Vec<GridPos>, facing: Facing) -> impl FieldOfView {
    PatternFOV { pattern, facing }
}

impl FieldOfView for OmniFOV {
    fn sees(&self, to: GridPos) -> bool {
        chebyshev_distance(&ORIGIN, &to) <= self.range
    }
}

impl FieldOfView for ConeFOV {
    fn sees(&self, pos: GridPos) -> bool {
        let distance = chebyshev_distance(&ORIGIN, &pos);
        let target = self.facing * RealPos::from(pos);
        let angle = target.normalized().dot(RealPos::unit_y()).acos();
        if angle.is_nan() {
            return true;
        }
        angle.abs() <= self.angle && distance <= self.range
    }
}

impl FieldOfView for QuadraticFOV {
    fn sees(&self, to: GridPos) -> bool {
        let distance = chebyshev_distance(&ORIGIN, &to);
        let target = self.facing * RealPos::from(to);
        let fov_limit = (target.x * self.a).powi(2) + self.b;

        fov_limit <= target.y && distance <= self.range
    }
}

#[cfg(test)]
mod tests {

    use crate::core::{
        constants::*,
        types::{Cardinal, GridPos},
    };

    use super::{
        cone_fov, infinite_fov, omnidirectional_fov, pattern_fov, quadratic_fov, FieldOfView,
    };

    #[test]
    fn pattern() {
        let pattern: Vec<GridPos> = vec![
            GridPos::new(0, 1),
            GridPos::new(0, 2),
            GridPos::new(0, 3),
            GridPos::new(-1, 3),
            GridPos::new(1, 3),
            GridPos::new(0, 4),
            GridPos::new(-1, 4),
            GridPos::new(1, 4),
        ];
        let fov = pattern_fov(pattern.clone(), Cardinal::North.into());
        let fov_nw = pattern_fov(pattern.clone(), Cardinal::NorthWest.into());

        for pos in pattern {
            assert!(fov.sees(pos));
        }

        let pattern_nw: Vec<GridPos> = vec![
            GridPos::new(1, 1),
            GridPos::new(2, 2),
            GridPos::new(3, 3),
            GridPos::new(2, 3),
            GridPos::new(3, 2),
            GridPos::new(4, 4),
            GridPos::new(3, 4),
            GridPos::new(4, 3),
        ];

        for pos in pattern_nw {
            assert!(fov_nw.sees(pos));
        }
    }

    #[test]
    fn infinite() {
        let fov = infinite_fov();

        let targets = vec![
            GridPos::new(0, -100_000),
            GridPos::new(100_000, 0),
            GridPos::new(0, 100_000),
            GridPos::new(-100_000, 0),
            GridPos::new(0, 0),
            GridPos::new(100_000, 100_000),
        ];

        for target in targets {
            let is_seen = fov.sees(target);
            assert!(is_seen);
        }
    }

    #[test]
    fn tiny_fov() {
        let fov = omnidirectional_fov(1);

        let far_targets = vec![
            GridPos::new(0, -100_000),
            GridPos::new(100_000, 0),
            GridPos::new(0, 100_000),
            GridPos::new(-100_000, 0),
            GridPos::new(100_000, 100_000),
        ];

        let near_targets = vec![GridPos::new(0, 0), GridPos::new(1, 1), GridPos::new(-1, -1)];

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
        let fov = cone_fov(5, PI / 2.0, Cardinal::North.into());

        let north_targets = vec![GridPos::new(0, 0), GridPos::new(1, -1), GridPos::new(0, -3)];
        for target in north_targets {
            let is_seen = fov.sees(target);
            assert!(is_seen);
        }

        for target in targets_along_diagonal_nw() {
            let is_seen = fov.sees(target);
            assert!(is_seen);
        }

        let is_seen = fov.sees(GridPos::new(-6, -6));
        assert!(!is_seen);
    }

    #[test]
    fn view_curve() {
        let fov = quadratic_fov(5, Cardinal::North.into(), 0.5, -1.5);

        let north_targets = vec![GridPos::new(0, 0), GridPos::new(1, -1), GridPos::new(0, -3)];
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
        let fov = quadratic_fov(5, Cardinal::East.into(), 0.5, -1.5);

        for target in targets_facing_east() {
            let is_seen = fov.sees(target);
            assert!(is_seen);
        }
    }

    #[test]
    fn view_curve_west() {
        let fov = quadratic_fov(5, Cardinal::West.into(), 0.5, -1.5);

        for target in targets_facing_west() {
            let is_seen = fov.sees(target);
            assert!(is_seen);
        }

        for target in targets_facing_west_hidden() {
            let is_seen = fov.sees(target);
            assert!(!is_seen);
        }
    }

    fn targets_behind_facing_north() -> Vec<GridPos> {
        vec![GridPos::new(-1, 1), GridPos::new(0, 1), GridPos::new(1, 1)]
    }

    fn targets_side_facing_north() -> Vec<GridPos> {
        vec![
            GridPos::new(1, 0),
            GridPos::new(-1, 0),
            GridPos::new(2, 0),
            GridPos::new(-2, 0),
            GridPos::new(-2, -1),
            GridPos::new(2, -1),
        ]
    }

    fn targets_facing_east() -> Vec<GridPos> {
        vec![
            GridPos::new(1, 0),
            GridPos::new(1, 1),
            GridPos::new(1, -1),
            GridPos::new(2, 2),
            GridPos::new(2, -2),
            GridPos::new(3, -3),
        ]
    }

    fn targets_facing_west() -> Vec<GridPos> {
        vec![
            GridPos::new(1, -1),
            GridPos::new(1, 0),
            GridPos::new(1, 1),
            GridPos::new(0, 1),
            GridPos::new(0, 2),
            GridPos::new(-2, -2),
            GridPos::new(-2, -1),
            GridPos::new(-2, 0),
            GridPos::new(-2, 1),
            GridPos::new(-2, 2),
        ]
    }

    fn targets_facing_west_hidden() -> Vec<GridPos> {
        vec![
            GridPos::new(1, -2),
            GridPos::new(2, -1),
            GridPos::new(2, 0),
            GridPos::new(2, 1),
            GridPos::new(1, 2),
            GridPos::new(-6, 0),
        ]
    }

    fn targets_along_diagonal_nw() -> Vec<GridPos> {
        vec![
            GridPos::new(-1, -1),
            GridPos::new(-2, -2),
            GridPos::new(-3, -3),
            GridPos::new(-4, -4),
            GridPos::new(-5, -5),
        ]
    }
}
