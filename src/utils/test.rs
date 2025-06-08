#[cfg(test)]
mod tests {
    use crate::utils::utils::zeroes;

    #[test]
    fn zero_fn_test() {
        let result = zeroes(2);
        assert_eq!(result, vec![0, 0]);
    }
}
