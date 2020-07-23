fn main() {
    let SHI3 = 999.0;
    let mut WU4 = SHI3;
    let mut _rand1 = 0.0;
    while _rand1 < SHI3 {
        let mut XU1 = WU4;
        let mut _rand2 = 0.0;
        while _rand2 < WU4 {
            let _ans1 = WU4 * XU1;
            let JIA3 = _ans1;
            if JIA3 < 10.0 {
                let _ans2 = XU1;
                let _ans3 = WU4;
                let _ans4 = "如";
                let _ans5 = JIA3;
                println!("{} {} {} {}", _ans2, _ans3, _ans4, _ans5);
            } else {
                let _ans6 = XU1;
                let _ans7 = WU4;
                let _ans8 = JIA3;
                println!("{} {} {}", _ans6, _ans7, _ans8);
            }
            let _ans9 = XU1 - 1.0;
            XU1 = _ans9;
            _rand2 += 1.0;
        }
        let _ans10 = WU4 - 1.0;
        WU4 = _ans10;
        _rand1 += 1.0;
    }
}
