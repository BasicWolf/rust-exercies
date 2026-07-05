fn main() {}

#[test]
fn f001_hello_world() {
    println!("Hello, world!");
}

#[test]
fn f002_math_power() {
    assert!(2u32.pow(2u32) == 4, "2^2 == 4");
    assert!(2f32.powi(-2i32) == 0.25, "2f32^-2i32 == 0.25");
    // but we cannot just do 2.pow(2), because type is ambiguous
    // hence, compiler can't understand which pow() it should use.
}

#[test]
fn f003_for_loop() {
    let mut j: i32 = 0;
    for i in 0..10 {
        if i == 9 {
            j = 1;
        }
    }
    assert!(1 == j);
}
