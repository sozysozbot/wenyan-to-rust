fn main() {
    let mut JIA3 = vec![];
    JIA3.push(3.0);
    JIA3.push(5.0);
    let _ans1 = JIA3.len() as f64;
    println!("{}", _ans1);
    let mut YI3 = 1.0;
    loop {
        if (JIA3.len() as f64) < YI3 {
            break;
        }
        let _ans2 = YI3 + 1.0;
        YI3 = _ans2;
    }
}
