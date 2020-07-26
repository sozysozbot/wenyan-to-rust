fn main() {
    let mut JIA3 = 0.0;
    let mut YI3 = 0.0;
    for _ in 0..100 {
        let _ans1 = JIA3 + 1.0;
        JIA3 = _ans1;
        let _ans2 = JIA3 % 2.0;
        if _ans2 == 1.0 {
            continue;
        }
        let _ans3 = YI3 + JIA3;
        YI3 = _ans3;
    }
    let _ans4 = YI3;
    println!("{}", _ans4);
}
