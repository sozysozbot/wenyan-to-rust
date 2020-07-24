fn main() {
    let mut JIA3 = vec![];
    JIA3.push(3.0);
    JIA3.push(5.0);
    let mut YI3 = vec![];
    YI3.append(&mut vec![2.0, 9.0, 4.0, 22.0]);
    let _ans1 = [&JIA3[..], &YI3[..]].concat();
    let BING3 = _ans1;
    JIA3[1 - 1] = 5.0;
    YI3[3 - 1] = BING3[4 - 1];
}
