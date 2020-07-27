fn main() {
    let mut JIA3 = vec![];
    JIA3.push(3.0);
    JIA3.push(5.0);
    let YI3 = 1.0;
    if YI3 < JIA3[(YI3 as usize) - 1] {
        let _ans1 = YI3;
        println!("{}", _ans1);
    }
}
