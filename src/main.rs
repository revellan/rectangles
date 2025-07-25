#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let rect = Rect {
        width: 30,
        height: 50,
    };
    println!("the area of the rectangle is {} square pixels", rect.area())
}
