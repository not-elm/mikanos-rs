pub trait Abs {
    /// Computes the absolute value
    fn abs(&self) -> Self;


    /// Computes the absolute value and cast to usize
    fn abs_usize(&self) -> usize;
}


macro_rules! abs {
    ($target: ty) => {
        impl Abs for $target {
            fn abs(&self) -> Self {
                let num = *self;
                if num < 0 {
                    -num
                } else {
                    num
                }
            }


            fn abs_usize(&self) -> usize {
                self.abs() as usize
            }
        }
    };
}

abs!(i8);
abs!(i16);
abs!(i32);
abs!(i64);
abs!(isize);


pub fn abs(num: isize) -> usize {
    if num < 0 {
        -num as usize
    } else {
        num as usize
    }
}


#[cfg(test)]
mod tests {
    use crate::math::abs::Abs;

    #[test]
    fn it_abs_integer_types() {
        assert_eq!(32_i8.abs(), 32_i8);
        assert_eq!((-32_i8).abs(), 32_i8);

        assert_eq!(32_i16.abs(), 32_i16);
        assert_eq!((-32_i16).abs(), 32_i16);

        assert_eq!(32_i32.abs(), 32_i32);
        assert_eq!((-32_i32).abs(), 32_i32);

        assert_eq!(32_i64.abs(), 32_i64);
        assert_eq!((-32_i64).abs(), 32_i64);

        assert_eq!(32_isize.abs(), 32_isize);
        assert_eq!((-32_isize).abs(), 32_isize);
    }


    #[test]
    fn it_abs_usize_integer_types() {
        assert_eq!(32i8.abs_usize(), 32usize);
        assert_eq!((-32_i8).abs_usize(), 32usize);

        assert_eq!(32i16.abs_usize(), 32usize);
        assert_eq!((-32_i16).abs_usize(), 32usize);

        assert_eq!(32i32.abs_usize(), 32usize);
        assert_eq!((-32_i32).abs_usize(), 32usize);

        assert_eq!(32i64.abs_usize(), 32usize);
        assert_eq!((-32_i64).abs_usize(), 32usize);

        assert_eq!(32isize.abs_usize(), 32usize);
        assert_eq!((-32_isize).abs_usize(), 32usize);
    }
}
