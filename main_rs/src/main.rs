extern "C" {
    fn gcd(a: i32, b: i32) -> i32;
}

fn safe_like_gcd(a: i32, b: i32) -> i32 {
    let mut res = 0;
    unsafe {
        res = gcd(a, b);
    }
    res
}

fn main() {
    let mut res = 0;
    unsafe {
        res = gcd(12, 8);
    }
    assert_eq!(res, 4);
    println!("{:?}", res);

    res = safe_like_gcd(12, 8);
    assert_eq!(res, 4);
    println!("{:?}", res);
}
