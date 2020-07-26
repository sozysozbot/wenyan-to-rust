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
    let _ans2 = GUANG3 - 1.0;
    let _ans3 = _ans2 / 2.0;
    let BAN4 = _ans3;
    let mut SHU4 = 1.0;
    let _ans4 = GUANG3 * GUANG3;
    let mut _rand3 = 0.0;
    while _rand3 < _ans4 {
        let _ans5 = SHU4 - 1.0;
        let _ans6 = _ans5 % GUANG3;
        let ZHE2 = _ans6;
        let _ans7 = SHU4 - ZHE2;
        let _ans8 = _ans7 - 1.0;
        let _ans9 = _ans8 / GUANG3;
        let LVE4 = _ans9;
        let _ans10 = ZHE2 + BAN4;
        let _ans11 = _ans10 - LVE4;
        let LE4 = _ans11;
        let _ans12 = ZHE2 - BAN4;
        let _ans13 = _ans12 + LVE4;
        let NU3 = _ans13;
        let _ans14 = LE4 + GUANG3;
        let _ans15 = _ans14 % GUANG3;
        let _ans16 = _ans15 + 1.0;
        let HENG2 = _ans16;
        let _ans17 = NU3 + GUANG3;
        let _ans18 = _ans17 % GUANG3;
        let _ans19 = _ans18 + 1.0;
        let ZONG4 = _ans19;
        let _ans20 = ZONG4HENG2TU2[(ZONG4 as usize) - 1];
        let mut XING2 = _ans20;
        XING2[(HENG2 as usize) - 1] = SHU4;
        let _ans21 = SHU4 + 1.0;
        SHU4 = _ans21;
        _rand3 += 1.0;
    }
    for XING2 in ZONG4HENG2TU2 {
        let _ans22 = XING2;
        println!("{}", _ans22);
    }
}
