use crate::core::types::{GridPos, Int};

pub fn chessboard_rotate(pos: &GridPos, octants: Int) -> GridPos {
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

pub fn chessboard_rotate_vec(positions: Vec<GridPos>, octants: Int) -> Vec<GridPos> {
    positions
        .iter()
        .map(|pos| chessboard_rotate(pos, octants))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::test::rotation;

    use super::chessboard_rotate_vec;

    #[test]
    fn chessboard_rotations() {
        for case in rotation::test_cases() {
            let actual = chessboard_rotate_vec(case.shape, case.cardinal.into());

            actual.iter().for_each(|rotated_pos| {
                assert!(
                    case.expected.contains(rotated_pos),
                    "Received {:?} from case {:?}",
                    rotated_pos,
                    case.name
                )
            })
        }
    }
}
