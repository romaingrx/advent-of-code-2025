use std::ops::Range;

#[derive(Clone)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

pub struct Rectangle {
    x: Range<u64>,
    y: Range<u64>,
}

impl Rectangle {
    pub fn from_points(point_0: &Point, point_1: &Point) -> Self {
        let x_start = point_0.x.min(point_1.x);
        let x_end = point_0.x.max(point_1.x) + 1;
        let y_start = point_0.y.min(point_1.y);
        let y_end = point_0.y.max(point_1.y) + 1;
        Self { x: x_start..x_end, y: y_start..y_end }
    }

    pub fn area(&self) -> u64 {
        (self.x.end - self.x.start) * (self.y.end - self.y.start)
    }

    pub fn inner(&self) -> Rectangle {
        Self { x: (self.x.start + 1)..(self.x.end - 1), y: (self.y.start + 1)..(self.y.end - 1) }
    }

    pub fn overlaps(&self, other: &Rectangle) -> bool {
        (self.x.start < other.x.end)
            && (other.x.start < self.x.end)
            && (self.y.start < other.y.end)
            && (other.y.start < self.y.end)
    }
}
