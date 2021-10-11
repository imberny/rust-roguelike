use bevy::math::IVec2;

use crate::core::types::Int;

pub fn chessboard_distance(start: &IVec2, end: &IVec2) -> Int {
    let delta = *end - *start;
    std::cmp::max(delta.x.abs(), delta.y.abs())
}

pub fn chessboard_rotate_and_place(
    origin: &IVec2,
    positions: &[IVec2],
    octants: Int,
) -> Vec<IVec2> {
    positions
        .iter()
        .map(|pos| *origin + chessboard_rotate(pos, octants))
        .collect()
}

pub fn chessboard_rotate(pos: &IVec2, octants: Int) -> IVec2 {
    let mut result = pos.clone();

    if 1 == octants % 2 {
        result = rotate_half_quadrant(&result);
    }

    if octants >= 6 {
        IVec2::new(result.y, -result.x)
    } else if octants >= 4 {
        IVec2::new(-result.x, -result.y)
    } else if octants >= 2 {
        IVec2::new(-result.y, result.x)
    } else {
        result
    }
}

fn rotate_half_quadrant(result: &IVec2) -> IVec2 {
    if result.x.signum() == -result.y.signum() {
        if result.x.abs() > result.y.abs() {
            IVec2::new(result.x, result.x + result.y)
        } else {
            IVec2::new(-result.y, result.x + result.y)
        }
    } else if result.x.abs() > result.y.abs() {
        IVec2::new(result.x - result.y, result.x)
    } else {
        IVec2::new(result.x - result.y, result.y)
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::IVec2;

    use crate::{
        core::types::{Cardinal, Int},
        test,
    };

    use super::chessboard_rotate_and_place;

    #[test]
    fn chessboard_rotations() {
        test::rotation::cases().for_each(|case| {
            case.expected
                .iter()
                .enumerate()
                .for_each(|(octants, expected)| {
                    let actual =
                        chessboard_rotate_and_place(&IVec2::ZERO, &case.pattern, octants as Int);
                    actual.iter().for_each(|rotated_pos| {
                        assert!(
                            expected.contains(rotated_pos),
                            "Received wrong {:?} rotation for shape {:?}",
                            Cardinal::from(octants as Int),
                            case.shape
                        )
                    });
                });
        });
    }
}
