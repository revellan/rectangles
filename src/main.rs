struct Rect(u32,u32);
fn main() {
    let rect= Rect(30,50);
    println!("the area of the rectangle is {} square pixels",area(rect));
}
fn area(cords: Rect) -> u32 {
    cords.0 * cords.1
}
