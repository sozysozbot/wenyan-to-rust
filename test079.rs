fn main() {
    let mut JIA3 = vec![];
    JIA3.push(3.0);
    JIA3.push(5.0);
    let _ans1 = JIA3.len() as f64;
    println!("{}", _ans1);
    let mut YI3 = 1.0;
    loop {
        if YI3 > (JIA3.len() as f64) {
            break;
        }
        let _ans2 = JIA3[(YI3 as usize) - 1];
        println!("{}", _ans2);
        let _ans3 = YI3 + 1.0;
        YI3 = _ans3;
    }
}
