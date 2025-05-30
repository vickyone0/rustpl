

pub struct Point<X1, Y1> {
   pub x: X1,
   pub y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    pub fn mix_up(self, other: Point<X1, Y1>) -> Point<X1, Y1> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}