pub fn array_ne<T: PartialEq>(x1: &[T], x2: &[T]) -> bool {
    !eq_array(x1, x2)
}


pub fn eq_array<T: PartialEq>(x1: &[T], x2: &[T]) -> bool {
    if x1.len() != x2.len() {
        return false;
    }

    x1.iter()
        .zip(x2)
        .all(|(e1, e2)| e1 == e2)
}


#[cfg(test)]
mod tests {
    use crate::array::{array_ne, eq_array};

    #[test]
    fn it_equal_array() {
        let x1 = [1, 2, 3];
        let x2 = [1, 2, 3];
        assert!(eq_array(&x1, &x2));
    }


    #[test]
    fn it_ne_when_array_diff_size() {
        let x1 = [1, 2, 3];
        let x2 = [1, 2, 3, 4];
        assert!(array_ne(&x1, &x2));
    }


    #[test]
    fn it_ne_when_array_diff_element() {
        let x1 = [1, 2, 3];
        let x2 = [2, 1, 3];
        assert!(array_ne(&x1, &x2));
    }
}
