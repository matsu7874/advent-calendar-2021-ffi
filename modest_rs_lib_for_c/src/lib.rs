#[no_mangle]
pub extern "C" fn gcd(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut y = b;
    if x < y {
        let t = x;
        x = y;
        y = t;
    }
    while y > 0 {
        let t = x % y;
        x = y;
        y = t;
    }
    x
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(12, 4), 4);
    assert_eq!(gcd(12, 3), 3);
    assert_eq!(gcd(12, 7), 1);
    assert_eq!(gcd(2, 70), 2);
}
