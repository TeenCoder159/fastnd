pub fn zeroes(count: usize) -> Vec<isize>{
    let mut arr: Vec<isize> = vec![];
    for _ in 0..count{
        arr.push(0);
    }
    arr
}

// pub fn linspace(start: i32, end: i32, step: i32) -> Result<Vec<isize>, >
 #[cfg(test)]
 mod tests {
     use super::*;

     #[test]
     fn zero_fn_test() {
         let result = zeroes(2);
         assert_eq!(result, vec![0,0]);
     }

     #[test]
     fn temp_test(){
        assert_eq!(zeroes(0), vec![])
     }
 }
