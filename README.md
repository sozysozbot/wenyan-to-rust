# wenyan-to-rust

```
有數三。名之曰「甲」。
為是「甲」遍。
	吾有一言。曰「「問天地好在。」」。書之。
云云。
```

is compiled to

```
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

## todo
* Fix mutability issue and finish 明義第一
* Avoid name conflict
* Output numbers in Hanzi

