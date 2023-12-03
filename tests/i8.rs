use rsbit::BitOperation;
#[test]
pub fn test_i8_set_0() {
    let mut src: i8 = 0b1000_1010 as u8 as i8;
    (&mut src).set_0(0);
    assert_eq!(-118_i8, src);

    (&mut src).set_0(1);
    assert_eq!(-120_i8, src);

    (&mut src).set_0(7);
    assert_eq!(8 as i8, src);

    (&mut src).set_0(8);
    assert_eq!(8 as i8, src);
}

#[test]
pub fn test_i8_set_1() {
    let mut src = 0b0000_1010 as i8 as i8;
    (&mut src).set_1(1);
    assert_eq!(10_i8, src);

    (&mut src).set_1(0);
    assert_eq!(11_i8, src);

    (&mut src).set_1(7);
    assert_eq!(-117_i8, src);

    (&mut src).set_1(8);
    assert_eq!(-117_i8, src);
}

#[test]
pub fn test_i8_set_inverse() {
    let mut src = 0b1000_1010 as u8 as i8;
    (&mut src).set_inverse(0);
    assert_eq!(-117, src);

    (&mut src).set_inverse(1);
    assert_eq!(-119, src);

    (&mut src).set_inverse(7);
    assert_eq!(9, src);

    (&mut src).set_inverse(8);
    assert_eq!(9, src);
}
