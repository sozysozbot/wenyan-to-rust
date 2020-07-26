fn main() {
    let mut JIA3 = vec![];
    JIA3.append(&mut vec![1.0, 2.0, 3.0]);
    let mut YI3 = JIA3;
    YI3[1 - 1] = 4.0;
    let _ans1 = JIA3;
    println!("{}", _ans1);
}
