fn main() {
    let mut JIA3 = vec![];
    JIA3.push(3.0);
    JIA3.push(5.0);
    let mut YI3 = vec![];
    YI3.append(&mut vec![2.0, 9.0, 4.0, 22.0]);
    let _ans1 = JIA3[1 - 1];
    println!("{}", _ans1);
    let _ans2 = JIA3[2 - 1];
    println!("{}", _ans2);
    let _ans3 = YI3[4 - 1];
    let _ans4 = _ans3 + 45.0;
    println!("{}", _ans4);
}
