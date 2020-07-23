fn main() {
    let mut JIA3 = 91.0;
    let mut YI3 = 49.0;
    loop {
        if JIA3 == YI3 {
            break;
        } else if JIA3 > YI3 {
            let _ans1 = JIA3 - YI3;
            JIA3 = _ans1;
        } else {
            let _ans2 = YI3 - JIA3;
            YI3 = _ans2;
        }
    }
    let _ans3 = JIA3;
    println!("{}", _ans3);
}
