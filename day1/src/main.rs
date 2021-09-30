extern crate csv;

use std::fs::File;
use std::error::Error;
use std::io::Read;

/// ## Function that reads a file until EOF into a string.
///
/// - file_name dictates a relative path from the current working directory. 
///
/// *Function panics if file doesn't exist*
///
/// TODO: Refactor this function into a lib
/// ```
/// let result = read_report_csv("doesnt_exist.csv");
/// assert_eq!(result, "filecontents");
/// ```
fn read_report_csv(file_name: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(file_name).expect("Could not open file");
    let mut file_contents : String = String::new();
    file.read_to_string(&mut file_contents).expect("Could not read file");    
    Ok(file_contents)
}

fn process_file_contents(contents: &String) -> Result<Vec<i32>, Box<dyn Error>>  {
    let mut csv_file = csv::ReaderBuilder::new().has_headers(false).from_reader(contents.as_bytes());
    let mut result_vec: Vec<i32> = Vec::new();

    for record in csv_file.records() {
        let result = record?;
        println!("{:?}", result);
        for field in result.iter() {
            result_vec.push(field.parse::<i32>()?);
        }
    }
    
    assert!(result_vec.len() > 0, "No values read from file");
    Ok(result_vec)
}


fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use std::sync::Once;
    use std::fs;

    use super::*;

    struct FsCleanup;

    impl Drop for FsCleanup {
        fn drop(&mut self) {
            // Error returned if file doesn't exist, lets just ignore it. 
            let _ = fs::remove_file(FILENAME);
        }
    }

    static INIT: Once = Once::new();
    static FILENAME: &str = "test_csv.csv";

    static VALUES: [i32; 6] = [1721, 979, 366, 299, 675, 1456];


    fn init_test_csv() {
        INIT.call_once ( || {
            let mut contents_string: String = "".to_owned();
            VALUES.map(|v| {
                contents_string.push_str(&format!("{}\n", v));
            });
            fs::write(FILENAME, contents_string.into_bytes()).unwrap();
        });
    }
    
    #[test]
    #[should_panic]
    fn test_panic_on_missing_file() {
        read_report_csv("nothing.csv").unwrap();
    }

    #[test]
    fn test_read_file() {
        let _cleanup = FsCleanup;
        init_test_csv();
        let mut contents_string: String = "".to_owned();
        VALUES.map(|v| {
            contents_string.push_str(&format!("{}\n", v));
        });
        let result = read_report_csv(FILENAME).unwrap();
        assert_eq!(result, contents_string, "String read from test file doesn't not match expected contents")
    }
   
    #[test]
    fn test_content_parsing() {
        let mut contents_string: String = "".to_owned();
        VALUES.map(|v| {
            contents_string.push_str(&format!("{}\n", v));
        });
        let result = process_file_contents(&contents_string).unwrap();
        assert_eq!(result, VALUES.to_vec(), "Values read from {} file does not match expected values", FILENAME)
    }

    #[test]
    #[should_panic]
    fn test_panic_on_missing_values() {
        let file_contents: String = String::new();
        process_file_contents(&file_contents).unwrap();
    }

    #[test]
    #[ignore]
    fn test_find_candidates() {
        unimplemented!("Test Unimplemented");
    }
}