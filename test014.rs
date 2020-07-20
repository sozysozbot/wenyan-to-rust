fn main() {
    let JIA3 = 3.0;
    let _ans1 = "問天地好在。";
    println!("{}", _ans1);
    JIA3 = 4.0; // error[E0384]: cannot assign twice to immutable variable
}
