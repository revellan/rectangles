struct Rect {
    width: u32,
    height: u32,
}
fn main() {
    let rect = Rect {
        width: 30,
        height: 50,
    };
    println!("the area of the rectangle is {} square pixels", area(&rect));
}
fn area(cords: &Rect) -> u32 {
    cords.width * cords.height
}
