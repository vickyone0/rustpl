
pub struct Point<T, U> {
    pub x: T,
    pub y: U,
}

impl <T,U> Point<T,U> {
    pub fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32, f32> {
 pub fn distance_from_origin(&self) -> f32 {
 (self.x.powi(2) + self.y.powi(2)).sqrt()
 }
}