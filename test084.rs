fn main() {
    let mut QI2SUO3 = vec![];
    let mut SUO3NAN2 = vec![];
    let mut QI2FANG1 = vec![];
    let mut SUO3ZHAN1 = vec![];
    let mut SUO3ZENG4 = vec![];
    let mut SUO3BAO4 = vec![];
    let mut SUO3GAN3 = vec![];
    let mut SUO3SHANG1 = vec![];
    QI2SUO3.append(&mut vec!["太山", "桂林", "漢陽", "雁門"]);
    SUO3NAN2.append(&mut vec!["樑父艱", "湘水深", "隴阪長", "雪雰雰"]);
    QI2FANG1.append(&mut vec!["東", "南", "西", "北"]);
    SUO3ZHAN1.append(&mut vec!["翰", "襟", "裳", "巾"]);
    SUO3ZENG4.append(&mut vec!["金錯刀", "琴琅玕", "貂襜褕", "錦繡段"]);
    SUO3BAO4.append(&mut vec!["英瓊瑤", "雙玉盤", "明月珠", "青玉案"]);
    SUO3GAN3.append(&mut vec!["逍遙", "惆悵", "踟躕", "增嘆"]);
    SUO3SHANG1.append(&mut vec!["勞", "傷", "紆", "惋"]);
    let mut ZHANG1 = 1.0;
    let _ans1 = QI2SUO3.len() as f64;
    let mut _rand1 = 0.0;
    while _rand1 < _ans1 {
        let _ans2 = "我所思兮在";
        let _ans3 = QI2SUO3[(ZHANG1 as usize) - 1];
        let _ans4 = "。欲往從之";
        let _ans5 = SUO3NAN2[(ZHANG1 as usize) - 1];
        let _ans6 = "。側身";
        let _ans7 = QI2FANG1[(ZHANG1 as usize) - 1];
        let _ans8 = "望涕沾";
        let _ans9 = SUO3ZHAN1[(ZHANG1 as usize) - 1];
        let _ans10 = "。美人贈我";
        let _ans11 = SUO3ZENG4[(ZHANG1 as usize) - 1];
        let _ans12 = "。何以報之";
        let _ans13 = SUO3BAO4[(ZHANG1 as usize) - 1];
        let _ans14 = "。路遠莫致倚";
        let _ans15 = SUO3GAN3[(ZHANG1 as usize) - 1];
        let _ans16 = "。何爲懷憂心煩";
        let _ans17 = SUO3SHANG1[(ZHANG1 as usize) - 1];
        println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", _ans2, _ans3, _ans4, _ans5, _ans6, _ans7, _ans8, _ans9, _ans10, _ans11, _ans12, _ans13, _ans14, _ans15, _ans16, _ans17);
        let _ans18 = ZHANG1 + 1.0;
        ZHANG1 = _ans18;
        _rand1 += 1.0;
    }
}
