//main function can running with coverage.
fn main() {
    println!("Hello, world!");
}


#[cfg(test)]

//When running in the windows.
//Can running the test and debug.
//But can't running with the coverage.
mod test{
    //When running in the windows.
    //Can running the test and debug.
    //But can't running with the coverage.
    #[test]
    fn it_works(){
        assert_eq!(2 + 2, 4);
    }
}