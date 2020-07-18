fn num_to_hanzi(a: i32) -> String {
    match a {
        3 => "三".to_string(),
        0 => "零".to_string(),
        _ => unimplemented!()
    }
}

fn main() {
    let _ans1 = "問天地好在。";
    let _ans2 = "";
    println!("{} {}", _ans1, _ans2);
    let _ans3 = "天地";
    println!("{}", _ans3);
    let _ans4 = "宇宙";
    let _ans5 = "洪荒";
    let _ans6 = "";
    let _ans7 = "";
    println!("{} {} {} {}", _ans4, _ans5, _ans6, _ans7);
    let _ans8 = 3;
    let _ans9 = 0;
    println!("{} {}", num_to_hanzi(_ans8), num_to_hanzi(_ans9));
}
