use rsbit::BitOperation;
#[test]
pub fn test_u8_set_0() {
    let mut src = 0b1000_1010 as u8;
    (&mut src).set_0(0);
    assert_eq!(138 as u8, src);

    (&mut src).set_0(1);
    assert_eq!(136 as u8, src);

    (&mut src).set_0(7);
    assert_eq!(8 as u8, src);

    (&mut src).set_0(8);
    assert_eq!(8 as u8, src);
}

#[test]
pub fn test_u8_set_1() {
    let mut src = 0b0000_0010 as u8;
    (&mut src).set_1(1);
    assert_eq!(2 as u8, src);

    (&mut src).set_1(0);
    assert_eq!(3 as u8, src);

    (&mut src).set_1(7);
    assert_eq!(131 as u8, src);

    (&mut src).set_1(8);
    assert_eq!(131 as u8, src);
}

#[test]
pub fn test_u8_set_inverse() {
    let mut src = 0b1000_1010 as u8;
    (&mut src).set_inverse(0);
    assert_eq!(139 as u8, src);

    (&mut src).set_inverse(1);
    assert_eq!(137 as u8, src);

    (&mut src).set_inverse(7);
    assert_eq!(9 as u8, src);

    (&mut src).set_inverse(8);
    assert_eq!(9 as u8, src);
}

use rsbit::BitFlagOperation;
#[test]
pub fn test_u8_is_0() {
    let mut src = 0b1000_1010 as u8;
    assert_eq!(true, (&mut src).is_0(0));
    assert_eq!(false, (&mut src).is_0(1));
    assert_eq!(true, (&mut src).is_0(2));
    assert_eq!(false, (&mut src).is_0(3));
}

#[test]
pub fn test_u8_is_1() {
    let src = 0b1000_1010 as u8;
    assert_eq!(false, src.is_1(0));
    assert_eq!(true, src.is_1(1));
    assert_eq!(false, src.is_1(2));
    assert_eq!(true, src.is_1(3));
}
