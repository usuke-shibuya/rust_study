fn main() {
    let mut x = 5;
    let _y = &x;
    let z = &mut x;
    dbg!(z);
    dbg!(x);
}
