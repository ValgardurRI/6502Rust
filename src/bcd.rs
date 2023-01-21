// Implementation of Binary-Coded Decimal (BCD) numbers. This project only needs support for u8 size BCDs, but they can theoretically be arbitrary amount of digits

use std::fmt;

#[macro_export]
macro_rules! high_nibble {
    ($x:expr) => {
        ($x & 0xF0) >> 0x04
    };
}

#[macro_export]
macro_rules! low_nibble {
    ($x:expr) => {
        $x & 0x0F
    };
}

#[derive(Debug)]
pub struct BcdConvertError {
    details: String
}

impl BcdConvertError {
    fn new(msg: &str) -> BcdConvertError {
        BcdConvertError{details: msg.to_string()}
    }
}

impl fmt::Display for BcdConvertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}


pub trait BcdOps {
    fn overflowing_add_bcd(&self, value: &u8, carry: &bool) -> (u8, bool);
    fn overflowing_sub_bcd(&self, value: &u8, carry: &bool) -> (u8, bool);
    fn from_dec(value: &u8) -> Result<u8, BcdConvertError>;
    fn to_dec(&self) -> u8;
}

impl BcdOps for u8 {
    /// Returns true if value overflowed
    fn overflowing_add_bcd(&self, value: &Self, carry: &bool) -> (Self, bool) {
        let mut low_nibble_sum = low_nibble!(self) + low_nibble!(value) + if *carry {1} else {0};
        let mut high_nibble_sum = high_nibble!(self)+ high_nibble!(value);

        // If sum of low nibbles is larger than a decimal digit, we carry the overflowing digit to the next decimal. (since 15 (hex) - 9 (decimal) = 6)
        if low_nibble_sum > 9 {
            low_nibble_sum -= 0x0A;
            high_nibble_sum += 0x01;
        }

        let mut overflow = false;
        if  high_nibble_sum > 9 {
            high_nibble_sum  -= 0x0A; 
            overflow = true
        }

        (high_nibble_sum << 0x04 | low_nibble_sum, overflow)
    }

    fn overflowing_sub_bcd(&self, value: &Self, carry: &bool) -> (Self, bool) {
        let (mut low_nibble_sub, low_nibble_overflow) = low_nibble!(self).overflowing_sub(low_nibble!(value) + if *carry {0} else {1}); 
        let (mut high_nibble_sub, mut high_nibble_overflow) = high_nibble!(self).overflowing_sub(high_nibble!(value)); 

        // if low nibble overflowed, we borrow from high_nibble
        if low_nibble_overflow{
            // Add 10 to account for the borrowed nibble
            low_nibble_sub = low_nibble_sub.wrapping_add(10);

            let (high_nibble_sub2, high_nibble_overflow2) = high_nibble_sub.overflowing_sub(1);
            high_nibble_overflow |= high_nibble_overflow2;
            high_nibble_sub = high_nibble_sub2;
        }

        if high_nibble_overflow {
            // if we know that the high nibble overflowed, we can "borrow" from the overflow
            high_nibble_sub = high_nibble_sub.wrapping_add(10);
        }
        (high_nibble_sub << 0x04 | low_nibble_sub, high_nibble_overflow)
    }

    
    fn from_dec(value: &u8) -> Result<Self, BcdConvertError> {
        let high = *value / 10;
        let low = *value % 10;
        if high > 9 || low > 9{
            return Err(BcdConvertError::new("Binary-coded decimal must be between 0x00 and 0x99!"))
        }
        let value = high << 0x04 | low;
        Ok(value)
    }

    fn to_dec(&self) -> u8 {
        high_nibble!(self)*10 + low_nibble!(self)
    }
}