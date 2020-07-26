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
}
