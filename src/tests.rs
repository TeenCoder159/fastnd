#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn zero_fn_test() {
        let result = zeroes(2);
        assert_eq!(result, vec![0, 0]);
    }
    #[test]
    fn temp_test() {
        assert_eq!(zeroes(0), vec![])
    }

    #[test]
    fn linspace_test() {
        assert_eq!(linspace(0, 5, 2), vec![0,2, 4]);
        assert_eq!(linspace(2, 10, 3), vec![2, 5, 8]);  
    }


}
