use std::sync::Once;
use std::error::Error;
use std::fs;

fn read_report_csv(filename: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    //panic if no file found
    //panic if no values read from file
    unimplemented!("Function unimplemented");
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    static INIT: Once = Once::new();
    static FILENAME: &str = "test_csv.csv";

    fn init_test_csv() {
        INIT.call_once ( || {
            fs::write(FILENAME, b"1721\n979\n366\n299\n675\n1456").unwrap();
        });
    }

    #[test]
    fn test_read_file() {
        init_test_csv();
        let result = read_report_csv(FILENAME).unwrap();
        unimplemented!("Test Unimplemented");
    }

    #[test]
    fn test_find_candidates() {
        unimplemented!("Test Unimplemented");
    }
}