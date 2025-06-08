use crate::utils::zeroes;

#[test]
fn zero_fn_test() {
    let result = zeroes(2);
    assert_eq!(result, vec![0,0]);
}

#[test]
fn temp_test(){
    assert_eq!(zeroes(0), vec![])
}