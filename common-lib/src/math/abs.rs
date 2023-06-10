pub trait Abs {
    type Output;


    fn abs(&self) -> Self::Output;
}


macro_rules! abs {
    ($target: ty, $output: ty) => {
        impl Abs for $target {
            type Output = $output;

            fn abs(&self) -> Self::Output {
                let num = *self;
                if num < 0 {
                    -num as Self::Output
                } else {
                    num as Self::Output
                }
            }
        }
    };
}


abs!(isize, usize);


pub fn abs(num: isize) -> usize {
    if num < 0 {
        -num as usize
    } else {
        num as usize
    }
}
