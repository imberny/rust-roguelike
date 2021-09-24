use std::cmp::{max, min};

use crate::core::types::GridPos;

/// source: https://github.com/amethyst/bracket-lib/blob/master/bracket-geometry/src/distance.rs
/// Calculates a Chebyshev distance between two points
/// See: http://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html
pub fn distance2d_chebyshev(start: GridPos, end: GridPos) -> i32 {
    let dx = max(start.x, end.x) - min(start.x, end.x);
    let dy = max(start.y, end.y) - min(start.y, end.y);
    if dx > dy {
        dx
    } else {
        dy
    }
}
