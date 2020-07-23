use super::*;
use serde_json;
use std::fs::File;
fn test(s: &str) {
    let pinyin_json = include_str!("../hanzi2roman-map-pinyin.json");
    let conversion_table: HashMap<String, String> = serde_json::from_str(pinyin_json).unwrap();

    let mut file = File::open(format!("{}.wy", s)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lex = lex::lex(&contents).unwrap();
    let parsed = parse::parse(&lex).unwrap();
    let compiled = compile::compile(&parsed, &conversion_table);

    let mut file2 = File::open(format!("{}.rs", s)).unwrap();
    let mut contents2 = String::new();
    file2.read_to_string(&mut contents2).unwrap();

    assert_eq!(
        str::replace(&compiled, "\r", ""),
        str::replace(&contents2, "\r", "")
    )
}

#[test]
fn test000() {
    test("test000")
}

#[test]
fn test001() {
    test("test001")
}
#[test]
fn test002() {
    test("test002")
}
#[test]
fn test003() {
    test("test003")
}

#[test]
fn test004() {
    test("test004")
}

#[test]
fn test005() {
    test("test005")
}

#[test]
fn test006() {
    test("test006")
}
#[test]
fn test007() {
    test("test007")
}
#[test]
fn test008() {
    test("test008")
}
#[test]
fn test009() {
    test("test009")
}
#[test]
fn test010() {
    test("test010")
}

#[test]
fn test011() {
    test("test011")
}

#[test]
fn test012() {
    test("test012")
}

#[test]
fn test013() {
    test("test013")
}

#[test]
fn test014() {
    test("test014")
}

#[test]
fn test015() {
    test("test015")
}

#[test]
fn test016() {
    test("test016")
}

#[test]
fn test017() {
    test("test017")
}

#[test]
fn test018() {
    test("test018")
}
#[test]
fn test019() {
    test("test019")
}
#[test]
fn test020() {
    test("test020")
}
#[test]
fn test021() {
    test("test021")
}
#[test]
fn test022() {
    test("test022")
}
#[test]
fn test023() {
    test("test023")
}
#[test]
fn test024() {
    test("test024")
}
#[test]
fn test025() {
    test("test025")
}
#[test]
fn test026() {
    test("test026")
}
#[test]
fn test027() {
    test("test027")
}
#[test]
fn test028() {
    test("test028")
}
#[test]
fn test029() {
    test("test029")
}
#[test]
fn test030() {
    test("test030")
}
#[test]
fn test031() {
    test("test031")
}
#[test]
fn test032() {
    test("test032")
}
#[test]
fn test033() {
    test("test033")
}
#[test]
fn test034() {
    test("test034")
}
#[test]
fn test035() {
    test("test035")
}
#[test]
fn test036() {
    test("test036")
}
#[test]
fn test037() {
    test("test037")
}
#[test]
fn test038() {
    test("test038")
}
#[test]
fn test039() {
    test("test039")
}
#[test]
fn test040() {
    test("test040")
}
#[test]
fn test041() {
    test("test041")
}
#[test]
fn test042() {
    test("test042")
}
#[test]
fn test043() {
    test("test043")
}
#[test]
fn test044() {
    test("test044")
}
#[test]
fn test045() {
    test("test045")
}
#[test]
fn test046() {
    test("test046")
}
#[test]
fn test047() {
    test("test047")
}
#[test]
fn test048() {
    test("test048")
}
#[test]
fn test049() {
    test("test049")
}
#[test]
fn test050() {
    test("test050")
}
#[test]
fn test051() {
    test("test051")
}
#[test]
fn test052() {
    test("test052")
}
#[test]
fn test053() {
    test("test053")
}
#[test]
fn test054() {
    test("test054")
}
#[test]
fn test055() {
    test("test055")
}
#[test]
fn test056() {
    test("test056")
}
#[test]
fn test057() {
    test("test057")
}
#[test]
fn test058() {
    test("test058")
}
#[test]
fn test059() {
    test("test059")
}
#[test]
fn test060() {
    test("test060")
}
#[test]
fn test061() {
    test("test061")
}
#[test]
fn test062() {
    test("test062")
}
#[test]
fn test063() {
    test("test063")
}
#[test]
fn test064() {
    test("test064")
}
#[test]
fn test065() {
    test("test065")
}
#[test]
fn test066() {
    test("test066")
}
#[test]
fn test067() {
    test("test067")
}
#[test]
fn test068() {
    test("test068")
}
#[test]
fn test069() {
    test("test069")
}
#[test]
fn test070() {
    test("test070")
}