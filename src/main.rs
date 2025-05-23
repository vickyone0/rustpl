struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
    self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };
    let v = Rectangle::area(&self.rect1);

    print!("Area of the rectangle is {}, {}/n is can hold is avalible ? {}", rect1.width(),rect1.area(), rect2.can_hold(&rect1));
}
