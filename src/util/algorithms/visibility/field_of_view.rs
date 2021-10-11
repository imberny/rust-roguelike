use bevy::math::{IVec2, Vec2};

use crate::{
    core::types::{Cardinal, Facing, Int, Real},
    util::{
        algorithms::geometry::{chessboard_distance, chessboard_rotate},
        helpers::GridRotator,
    },
};

pub enum FOV {
    Infinite,
    Omnidirectional(Int),
    Cone(Int, Real),
    Quadratic(Int, Real, Real),
    Pattern(Vec<IVec2>),
}

impl FOV {
    pub fn sees(&self, position: &IVec2, cardinal: Cardinal) -> bool {
        match self {
            FOV::Infinite => true,
            FOV::Omnidirectional(range) => is_in_range(position, *range),
            FOV::Cone(range, angle) => is_in_cone(position, cardinal, *range, *angle),
            FOV::Quadratic(range, a, b) => is_above_curve(position, cardinal, *range, *a, *b),
            FOV::Pattern(pattern) => is_in_pattern(pattern, position, cardinal),
        }
    }
}

fn is_in_range(position: &IVec2, range: Int) -> bool {
    chessboard_distance(&IVec2::ZERO, position) <= range
}

fn is_in_cone(pos: &IVec2, cardinal: Cardinal, range: Int, angle: Real) -> bool {
    let octants = (8 - Int::from(cardinal)) % 8;
    let target = chessboard_rotate(pos, octants).as_vec2();
    is_within_angle(&target, angle) && is_in_range(pos, range)
}

fn is_within_angle(target: &Vec2, angle: f32) -> bool {
    let target_angle = target.normalize().dot(-Vec2::Y).acos();
    if target_angle.is_nan() {
        return true;
    }
    target_angle.abs() <= angle
}

fn is_above_curve(pos: &IVec2, cardinal: Cardinal, range: Int, a: Real, b: Real) -> bool {
    let target = Facing::from(cardinal).rot_f(&pos.as_vec2());
    let fov_limit = (target.x * a).powi(2) + b;

    fov_limit <= target.y && is_in_range(pos, range)
}

fn is_in_pattern(pattern: &[IVec2], position: &IVec2, cardinal: Cardinal) -> bool {
    let pos = chessboard_rotate(position, cardinal.into());
    pattern.contains(&pos)
}

#[cfg(test)]
mod tests {

    use bevy::math::IVec2;

    use crate::{
        core::{constants::*, types::Cardinal},
        util::algorithms::field_of_view::FOV,
    };

    #[test]
    fn pattern() {
        let pattern: Vec<IVec2> = vec![
            IVec2::new(0, -1),
            IVec2::new(0, -2),
            IVec2::new(0, -3),
            IVec2::new(-1, -3),
            IVec2::new(1, -3),
            IVec2::new(0, -4),
            IVec2::new(-1, -4),
            IVec2::new(1, -4),
        ];
        let fov = FOV::Pattern(pattern.clone());

        for pos in pattern {
            assert!(fov.sees(&pos, Cardinal::North));
        }

        let pattern_nw: Vec<IVec2> = vec![
            IVec2::new(1, -1),
            IVec2::new(2, -2),
            IVec2::new(3, -3),
            IVec2::new(2, -3),
            IVec2::new(3, -2),
            IVec2::new(4, -4),
            IVec2::new(3, -4),
            IVec2::new(4, -3),
        ];

        for pos in pattern_nw {
            assert!(fov.sees(&pos, Cardinal::NorthWest));
        }
    }

    #[test]
    fn infinite() {
        let fov = FOV::Infinite;

        let targets = vec![
            IVec2::new(0, -100_000),
            IVec2::new(100_000, 0),
            IVec2::new(0, 100_000),
            IVec2::new(-100_000, 0),
            IVec2::new(0, 0),
            IVec2::new(100_000, 100_000),
        ];

        for target in targets {
            let is_seen = fov.sees(&target, Cardinal::North);
            assert!(is_seen, "Could not see {:?}", target);
        }
    }

    #[test]
    fn tiny_fov() {
        let fov = FOV::Omnidirectional(1);

        let far_targets = vec![
            IVec2::new(0, -100_000),
            IVec2::new(100_000, 0),
            IVec2::new(0, 100_000),
            IVec2::new(-100_000, 0),
            IVec2::new(100_000, 100_000),
        ];

        let near_targets = vec![IVec2::new(0, 0), IVec2::new(1, 1), IVec2::new(-1, -1)];

        for target in far_targets {
            let is_seen = fov.sees(&target, Cardinal::North);
            assert!(!is_seen);
        }

        for target in near_targets {
            let is_seen = fov.sees(&target, Cardinal::North);
            assert!(is_seen, "Could not see {:?}", target);
        }
    }

    #[test]
    fn directed_fov() {
        let fov = FOV::Cone(5, PI / 2.0);

        let north_targets = vec![IVec2::new(0, 0), IVec2::new(1, -1), IVec2::new(0, -3)];
        for target in north_targets {
            let is_seen = fov.sees(&target, Cardinal::North);
            assert!(is_seen, "Could not see {:?}", target);
        }

        for target in targets_along_diagonal_nw() {
            let is_seen = fov.sees(&target, Cardinal::North);
            assert!(is_seen, "Could not see {:?}", target);
        }

        let is_seen = fov.sees(&IVec2::new(-6, -6), Cardinal::North);
        assert!(!is_seen);
    }

    #[test]
    fn view_curve() {
        let fov = FOV::Quadratic(5, 0.5, -1.5);

        let north_targets = vec![IVec2::new(0, 0), IVec2::new(1, -1), IVec2::new(0, -3)];
        for target in north_targets {
            let is_seen = fov.sees(&target, Cardinal::North);
            assert!(is_seen, "Could not see {:?}", target);
        }

        for target in targets_along_diagonal_nw() {
            let is_seen = fov.sees(&target, Cardinal::North);
            assert!(is_seen, "Could not see {:?}", target);
        }

        for target in targets_behind_facing_north() {
            let is_seen = fov.sees(&target, Cardinal::North);
            assert!(is_seen, "Could not see {:?}", target);
        }

        for target in targets_side_facing_north() {
            let is_seen = fov.sees(&target, Cardinal::North);
            assert!(is_seen, "Could not see {:?}", target);
        }
    }

    #[test]
    fn view_curve_east() {
        let fov = FOV::Quadratic(5, 0.5, -1.5);

        for target in targets_facing_east() {
            let is_seen = fov.sees(&target, Cardinal::East);
            assert!(is_seen, "Could not see {:?}", target);
        }
    }

    #[test]
    fn view_curve_west() {
        let fov = FOV::Quadratic(5, 0.5, -1.5);

        for target in targets_facing_west() {
            let is_seen = fov.sees(&target, Cardinal::West);
            assert!(is_seen, "Could not see {:?}", target);
        }

        for target in targets_facing_west_hidden() {
            let is_seen = fov.sees(&target, Cardinal::West);
            assert!(!is_seen);
        }
    }

    fn targets_behind_facing_north() -> Vec<IVec2> {
        vec![IVec2::new(-1, 1), IVec2::new(0, 1), IVec2::new(1, 1)]
    }

    fn targets_side_facing_north() -> Vec<IVec2> {
        vec![
            IVec2::new(1, 0),
            IVec2::new(-1, 0),
            IVec2::new(2, 0),
            IVec2::new(-2, 0),
            IVec2::new(-2, -1),
            IVec2::new(2, -1),
        ]
    }

    fn targets_facing_east() -> Vec<IVec2> {
        vec![
            IVec2::new(1, 0),
            IVec2::new(1, 1),
            IVec2::new(1, -1),
            IVec2::new(2, 2),
            IVec2::new(2, -2),
            IVec2::new(3, -3),
        ]
    }

    fn targets_facing_west() -> Vec<IVec2> {
        vec![
            IVec2::new(1, -1),
            IVec2::new(1, 0),
            IVec2::new(1, 1),
            IVec2::new(0, 1),
            IVec2::new(0, 2),
            IVec2::new(-2, -2),
            IVec2::new(-2, -1),
            IVec2::new(-2, 0),
            IVec2::new(-2, 1),
            IVec2::new(-2, 2),
        ]
    }

    fn targets_facing_west_hidden() -> Vec<IVec2> {
        vec![
            IVec2::new(1, -2),
            IVec2::new(2, -1),
            IVec2::new(2, 0),
            IVec2::new(2, 1),
            IVec2::new(1, 2),
            IVec2::new(-6, 0),
        ]
    }

    fn targets_along_diagonal_nw() -> Vec<IVec2> {
        vec![
            IVec2::new(-1, -1),
            IVec2::new(-2, -2),
            IVec2::new(-3, -3),
            IVec2::new(-4, -4),
            IVec2::new(-5, -5),
        ]
    }
}
