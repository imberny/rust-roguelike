use crate::core::types::{GridPos, Int};

pub fn chessboard_rotate_one(pos: &GridPos, octants: Int) -> GridPos {
    let mut result = pos.clone();

    if 1 == octants % 2 {
        result = if result.x.signum() == -result.y.signum() {
            if result.x.abs() > result.y.abs() {
                GridPos::new(result.x, result.x + result.y)
            } else {
                GridPos::new(-result.y, result.x + result.y)
            }
        } else if result.x.abs() > result.y.abs() {
            GridPos::new(result.x - result.y, result.x)
        } else {
            GridPos::new(result.x - result.y, result.y)
        };
    }

    if octants >= 6 {
        GridPos::new(result.y, -result.x)
    } else if octants >= 4 {
        GridPos::new(-result.x, -result.y)
    } else if octants >= 2 {
        GridPos::new(-result.y, result.x)
    } else {
        result
    }
}

pub fn chessboard_rotate(positions: &[GridPos], octants: Int) -> Vec<GridPos> {
    positions
        .iter()
        .map(|pos| chessboard_rotate_one(pos, octants))
        .collect()
}

pub fn chessboard_rotate_and_place(
    origin: &GridPos,
    positions: &[GridPos],
    octants: Int,
) -> Vec<GridPos> {
    positions
        .iter()
        .map(|pos| *origin + chessboard_rotate_one(pos, octants))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{
        core::types::{Cardinal, Int},
        test,
    };

    use super::chessboard_rotate;

    #[test]
    fn chessboard_rotations() {
        test::rotation::cases().for_each(|case| {
            case.expected
                .iter()
                .enumerate()
                .for_each(|(octants, expected)| {
                    let actual = chessboard_rotate(&case.pattern, octants as Int);
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
