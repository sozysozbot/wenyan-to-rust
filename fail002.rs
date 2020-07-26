fn main() {
    let mut XING2YI1 = vec![];
    XING2YI1.append(&mut vec![4.0, 9.0, 2.0]);
    let mut XING2ER4 = vec![];
    XING2ER4.append(&mut vec![3.0, 5.0, 7.0]);
    let mut XING2SAN1 = vec![];
    XING2SAN1.append(&mut vec![8.0, 1.0, 6.0]);
    let mut JIU3GONG1 = vec![];
    JIU3GONG1.append(&mut vec![XING2YI1, XING2ER4, XING2SAN1]);
    for XING2 in JIU3GONG1 {
        let _ans1 = XING2;
        println!("{}", _ans1);
    }
    let GUANG3 = 9.0;
    let mut ZONG4HENG2TU2 = vec![];
    let mut _rand1 = 0.0;
    while _rand1 < GUANG3 {
        let mut XING2 = vec![];
        let mut _rand2 = 0.0;
        while _rand2 < GUANG3 {
            XING2.push(0.0);
            _rand2 += 1.0;
        }
        ZONG4HENG2TU2.push(XING2);
        _rand1 += 1.0;
    }
}
