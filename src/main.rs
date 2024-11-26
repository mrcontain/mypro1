fn main() {
    println!("Hello, rust222!");
    let mut my = [1, 2, 3, 4, 5];
    let my2 = &mut my[1..3];
    println!("{:?}", my2);
}
