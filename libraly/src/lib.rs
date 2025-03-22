pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
//When running in the windows.
//Can running the test and debug.
//But can't running with the coverage.
mod tests {
    use super::*;


    //When running in the windows.
    //Can running the test and debug.
    //But can't running with the coverage
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
