fn main() {
    let JIA3 = false;
    let YI3 = false;
    let _ans1 = JIA3 || YI3;
    let BING3 = _ans1;
    let _ans2 = !BING3;
    let DING1 = _ans2;
    if DING1 {
        let _ans3 = "古之人誠不我欺。";
        println!("{}", _ans3);
    }
    let _ans4 = JIA3 || YI3;
    if !_ans4 {
        let _ans5 = "古之人誠不我欺。";
        println!("{}", _ans5);
    }
}
