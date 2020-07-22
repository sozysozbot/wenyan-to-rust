# wenyan-to-rust

## current status
Can compile all the samples in [明義第一](https://github.com/wenyan-lang/book/blob/master/01%20%E6%98%8E%E7%BE%A9%E7%AC%AC%E4%B8%80.md), [變數第二](https://github.com/wenyan-lang/book/blob/master/02%20%E8%AE%8A%E6%95%B8%E7%AC%AC%E4%BA%8C.md), [算術第三](https://github.com/wenyan-lang/book/blob/master/03%20%E7%AE%97%E8%A1%93%E7%AC%AC%E4%B8%89.md) and [決策第四](https://github.com/wenyan-lang/book/blob/master/04%20%E6%B1%BA%E7%AD%96%E7%AC%AC%E5%9B%9B.md).

## samples

### test006.wy
```
有數三。名之曰「甲」。
為是「甲」遍。
	吾有一言。曰「「問天地好在。」」。書之。
云云。
```

is compiled to

```rust
fn main() {
    let JIA3 = 3.0;
    let mut _rand1 = 0.0;
    while _rand1 < JIA3 {
        let _ans1 = "問天地好在。";
        println!("{}", _ans1);
        _rand1 += 1.0;
    }
}
```

### test007.wy
```
吾有三數。曰三。曰九。曰七。名之曰「甲」。書之。
吾有三數。曰三。曰九。曰七。名之曰「乙」。曰「丙」。書之。
吾有三數。曰三。曰九。曰七。名之曰「丁」。曰「戊」。曰「己」。書之。
吾有三數。曰三。曰九。名之曰「庚」。曰「辛」。曰「壬」。曰「癸」。書之。
```

```rust
fn main() {
    let JIA3 = 3.0;
    let _ans1 = 9.0;
    let _ans2 = 7.0;
    println!("{} {}", _ans1, _ans2);
    let YI3 = 3.0;
    let BING3 = 9.0;
    let _ans3 = 7.0;
    println!("{}", _ans3);
    let DING1 ZHENG1 = 3.0;
    let WU4 = 9.0;
    let JI3 = 7.0;
    println!("");
    let GENG1 = 3.0;
    let XIN1 = 9.0;
    let REN2 = 0.0;
    println!("");
}
```

### test014.wy
```
吾有一數。曰三。名之曰「甲」。
吾有一言。曰「「問天地好在。」」。書之。

昔之「甲」者。今四是矣。
```

```rust
fn main() {
    let mut JIA3 = 3.0;
    let _ans1 = "問天地好在。";
    println!("{}", _ans1);
    JIA3 = 4.0;
}
```

### test027.wy
```
加一以三。加二以三。減其以其。
加一以三。加二以三。減其於其。
```

```rust
fn main() {
    let _ans1 = 1.0 + 3.0;
    let _ans2 = 2.0 + 3.0;
    let _ans3 = _ans2 - f64::NAN;
    let _ans4 = 1.0 + 3.0;
    let _ans5 = 2.0 + 3.0;
    let _ans6 = _ans5 - f64::NAN;
}
```

### test057.wy
```
吾有九爻。曰陽。曰陰。曰陰。曰陽。曰陰。曰陰。曰陰。曰陰。曰陽。
名之曰「魯人耶」曰「衛人耶」曰「德行科耶」曰「政事科耶」
曰「複姓耶」曰「未仕耶」曰「蚤死耶」曰「病厲耶」曰「戰死耶」

夫「魯人耶」。若其然者。
　夫「德行科耶」。若其然者。
　　夫「未仕耶」。若其然者。
　　　夫「蚤死耶」。若其然者。
　　　　吾有一言。曰「「賢哉。回也。人不堪其憂。回也不改其樂。」」書之。
　　　若非。
　　　　吾有一言。曰「「孝哉。閔子騫。人不間於其父母昆弟之間。」」書之也。
　　若非。
　　　夫「病厲耶」。若其然者。
　　　　吾有一言。曰「「亡之。命也夫。斯人也。而有斯疾也。」」書之。
　　　若非。
　　　　吾有一言。曰「「雍也。可使南面。」」書之也。
　　云云。
　若非。
　　夫「政事科耶」。若其然者。
　　　夫「戰死耶」。若其然者。
　　　　吾有一言。曰「「若由也。不得其死然。」」書之。
　　　若非。
　　　　吾有一言。曰「「求。無乃爾是過與。」」書之也。
　　若非。
　　　吾有一言。曰「「朽木不可雕也。糞土之牆不可杇也。於予與何誅。」」書之也。
　云云。
若非。
　夫「複姓耶」。若其然者。
　　吾有一言。曰「「賜也。始可與言詩已矣。告諸往而知來者。」」書之。
　若非。
　　夫「衛人耶」。若其然者。
　　　吾有一言。曰「「起予者。商也。始可與言詩已矣。」」書之。
　　若非。
　　　吾有一言。曰「「二三子。偃之言是也。」」書之也。
　云云。
云云。
```

```rust
fn main() {
    let LU3REN2YE2 = true;
    let WEI4REN2YE2 = false;
    let DE2XING2KE1YE2 = false;
    let ZHENG4SHI4KE1YE2 = true;
    let FU4XING4YE2 = false;
    let WEI4SHI4YE2 = false;
    let ZAO3SI3YE2 = false;
    let BING4LI4YE2 = false;
    let ZHAN4SI3YE2 = true;
    let _ans1 = LU3REN2YE2;
    if _ans1 {
        let _ans2 = DE2XING2KE1YE2;
        if _ans2 {
            let _ans3 = WEI4SHI4YE2;
            if _ans3 {
                let _ans4 = ZAO3SI3YE2;
                if _ans4 {
                    let _ans5 = "賢哉。回也。人不堪其憂。回也不改其樂。";
                    println!("{}", _ans5);
                } else {
                    let _ans6 = "孝哉。閔子騫。人不間於其父母昆弟之間。";
                    println!("{}", _ans6);
                }
            } else {
                let _ans7 = BING4LI4YE2;
                if _ans7 {
                    let _ans8 = "亡之。命也夫。斯人也。而有斯疾也。";
                    println!("{}", _ans8);
                } else {
                    let _ans9 = "雍也。可使南面。";
                    println!("{}", _ans9);
                }
            }
        } else {
            let _ans10 = ZHENG4SHI4KE1YE2;
            if _ans10 {
                let _ans11 = ZHAN4SI3YE2;
                if _ans11 {
                    let _ans12 = "若由也。不得其死然。";
                    println!("{}", _ans12);
                } else {
                    let _ans13 = "求。無乃爾是過與。";
                    println!("{}", _ans13);
                }
            } else {
                let _ans14 = "朽木不可雕也。糞土之牆不可杇也。於予與何誅。";
                println!("{}", _ans14);
            }
        }
    } else {
        let _ans15 = FU4XING4YE2;
        if _ans15 {
            let _ans16 = "賜也。始可與言詩已矣。告諸往而知來者。";
            println!("{}", _ans16);
        } else {
            let _ans17 = WEI4REN2YE2;
            if _ans17 {
                let _ans18 = "起予者。商也。始可與言詩已矣。";
                println!("{}", _ans18);
            } else {
                let _ans19 = "二三子。偃之言是也。";
                println!("{}", _ans19);
            }
        }
    }
}
```

## todo
* `println!` the numbers in Hanzi

