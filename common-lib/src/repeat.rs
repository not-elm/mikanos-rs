pub trait RepeatCall<R> {
    fn repeat_call(&mut self, count: usize) -> R;
}


impl<F, R> RepeatCall<()> for F where F: FnMut() -> R {
    fn repeat_call(&mut self, count: usize) {
        (0..count)
            .for_each(|_| {
                self();
            });
    }
}




#[cfg(test)]
mod tests {
    use crate::repeat::RepeatCall;

    #[test]
    fn it_repeat_3times() {
        let mut num = 0;
        let mut f = || {
            num += 1
        };
        f.repeat_call(3);

        assert_eq!(num, 3);
    }



}