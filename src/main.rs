#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let rect1: Rect = Rect {
        width: 310,
        height: 250,
    };
    let rect2: Rect = Rect {
        width: 40,
        height: 60,
    };
    println!(
        "the area of the rectangle is {} square pixels",
        rect1.area()
    );
    println!(
        "rect1 {} hold rect2",
        match rect1.can_hold(&rect2) {
            true => "can",
            false => "can't",
        }
    )
}
