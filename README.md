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

## todo
* `println!` the numbers in Hanzi

