// The Operation of bit
pub trait BitOperation {
    /// Set the pos bit value to 0;
    /// The pos must be between 0 to 7;
    /// # Examples
    ///
    /// 1010_1000
    ///
    /// If the pos is 1, self will become 1010_1010.
    fn set_0(&mut self, pos: u8);

    /// Set the pos bit value to 1;
    /// The pos must be between 0 to 7;
    /// # Examples
    ///
    /// 1010_1000
    ///
    /// If the pos is 1, self will become 1010_1010.
    fn set_1(&mut self, pos: u8);

    /// Set the pos bit value to the inverse value(0->1,1->0);
    /// The pos must be between 0 to 7;
    /// # Examples
    ///
    /// 1010_1010
    ///
    /// If the pos is 1, self will become 0b1010_1000.
    ///
    /// If the pos is 4, self will become 0b1011_1010 (base on the original 0b1010_1010).
    fn set_inverse(&mut self, pos: u8);
}

// The BitFlagOperation of bit
pub trait BitFlagOperation {
    /// Check the pos bit value is 0;
    /// The pos must be between 0 to 7;
    /// # Examples
    ///
    /// 1010_1000
    ///
    /// If the pos is 1, it will return true
    /// If the pos is 3, it will return false
    fn is_0(&self, pos: u8) -> bool;

    /// Check the pos bit value is 1;
    /// The pos must be between 0 to 7;
    /// # Examples
    ///
    /// 1010_1000
    ///
    /// If the pos is 1, it will return false
    /// If the pos is 3, it will return true
    fn is_1(&self, pos: u8) -> bool;
}

impl BitOperation for &mut u8 {
    fn set_0(&mut self, pos: u8) {
        if pos >= 8 {
            return;
        }
        let flag = 1 << pos;
        // pos位是0，直接返回
        if flag != **self & flag {
            return;
        }
        // pos位是1，将其置为0
        **self = **self & (!flag);
    }

    fn set_1(&mut self, pos: u8) {
        if pos >= 8 {
            return;
        }
        let flag = 1 << pos;
        // pos位是1，直接返回
        if flag == **self & flag {
            return;
        }
        // pos位是0，将其置为1
        **self = **self & u8::MAX | flag;
    }

    fn set_inverse(&mut self, pos: u8) {
        if pos >= 8 {
            return;
        }
        let flag = 1 << pos;
        // pos位为1，将其置为0
        if flag == **self & flag {
            // pos位是1，将其置为0
            **self = **self & (!flag);
            return;
        }
        // pos位是0，将其置为1
        **self = **self & u8::MAX | flag;
    }
}

impl BitFlagOperation for &mut u8 {
    fn is_0(&self, pos: u8) -> bool {
        !self.is_1(pos)
    }

    fn is_1(&self, pos: u8) -> bool {
        if pos >= 8 {
            return false;
        }
        let flag = 1 << pos;

        **self & flag == flag
    }
}

impl BitFlagOperation for &u8 {
    fn is_0(&self, pos: u8) -> bool {
        !self.is_1(pos)
    }

    fn is_1(&self, pos: u8) -> bool {
        if pos >= 8 {
            return false;
        }
        let flag = 1 << pos;

        *self & flag == flag
    }
}

impl BitFlagOperation for u8 {
    fn is_0(&self, pos: u8) -> bool {
        !self.is_1(pos)
    }

    fn is_1(&self, pos: u8) -> bool {
        if pos >= 8 {
            return false;
        }
        let flag = 1 << pos;

        *self & flag == flag
    }
}

impl BitOperation for &mut i8 {
    fn set_0(&mut self, pos: u8) {
        if pos >= 8 {
            return;
        }
        let flag = 1 << pos;
        // pos位是0，直接返回
        if flag != **self & flag {
            return;
        }
        // pos位是1，将其置为0
        **self = **self & (!flag);
    }

    fn set_1(&mut self, pos: u8) {
        if pos >= 8 {
            return;
        }
        let flag = 1 << pos;
        // pos位是1，直接返回
        if flag == **self & flag {
            return;
        }
        // pos位是0，将其置为1
        **self = **self & 0b1111_1111u8 as i8 | flag;
    }

    fn set_inverse(&mut self, pos: u8) {
        if pos >= 8 {
            return;
        }
        let flag = 1 << pos;
        // pos位为1，将其置为0
        if flag == **self & flag {
            // pos位是1，将其置为0
            **self = **self & (!flag);
            return;
        }
        // pos位是0，将其置为1
        **self = **self & 0b1111_1111u8 as i8 | flag;
    }
}

impl BitFlagOperation for &mut i8 {
    fn is_0(&self, pos: u8) -> bool {
        !self.is_1(pos)
    }

    fn is_1(&self, pos: u8) -> bool {
        if pos >= 8 {
            return false;
        }
        let flag = 1 << pos;

        **self & flag == flag
    }
}

impl BitFlagOperation for &i8 {
    fn is_0(&self, pos: u8) -> bool {
        !self.is_1(pos)
    }

    fn is_1(&self, pos: u8) -> bool {
        if pos >= 8 {
            return false;
        }
        let flag = 1 << pos;

        *self & flag == flag
    }
}

impl BitFlagOperation for i8 {
    fn is_0(&self, pos: u8) -> bool {
        !self.is_1(pos)
    }

    fn is_1(&self, pos: u8) -> bool {
        if pos >= 8 {
            return false;
        }
        let flag = 1 << pos;

        *self & flag == flag
    }
}

impl BitOperation for &mut char {
    fn set_0(&mut self, pos: u8) {
        let mut c = **self as u8;
        (&mut c).set_0(pos);
        **self = c as char;
    }

    fn set_1(&mut self, pos: u8) {
        let mut c = **self as u8;
        (&mut c).set_1(pos);
        **self = c as char;
    }

    fn set_inverse(&mut self, pos: u8) {
        let mut c = **self as u8;
        (&mut c).set_inverse(pos);
        **self = c as char;
    }
}
