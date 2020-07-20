# wenyan-to-rust

Finished 明義第一.

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

## todo
* `println!` the numbers in Hanzi

