use rsbit::BitOperation;
#[test]
pub fn test_char_set_0() {
    let mut src = 0b1000_1010 as char;
    (&mut src).set_0(0);
    assert_eq!(138 as char, src);

    (&mut src).set_0(1);
    assert_eq!(136 as char, src);

    (&mut src).set_0(7);
    assert_eq!(8 as char, src);

    (&mut src).set_0(8);
    assert_eq!(8 as char, src);
}

#[test]
pub fn test_char_set_1() {
    let mut src = 0b0000_0010 as char;
    (&mut src).set_1(1);
    assert_eq!(2 as char, src);

    (&mut src).set_1(0);
    assert_eq!(3 as char, src);

    (&mut src).set_1(7);
    assert_eq!(131 as char, src);

    (&mut src).set_1(8);
    assert_eq!(131 as char, src);
}

#[test]
pub fn test_char_set_inverse() {
    let mut src = 0b1000_1010 as char;
    (&mut src).set_inverse(0);
    assert_eq!(139 as char, src);

    (&mut src).set_inverse(1);
    assert_eq!(137 as char, src);

    (&mut src).set_inverse(7);
    assert_eq!(9 as char, src);

    (&mut src).set_inverse(8);
    assert_eq!(9 as char, src);
}
