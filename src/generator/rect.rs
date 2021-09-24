use crate::core::types::Int;

pub struct Rect {
    pub x1: Int,
    pub x2: Int,
    pub y1: Int,
    pub y2: Int,
}

impl Rect {
    pub fn new(x: Int, y: Int, w: Int, h: Int) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (Int, Int) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}
